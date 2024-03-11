use std::path::Path;
use std::rc::Rc;
use std::{fs, vec};

use iced::futures::future::OrElse;
use iced::widget::{column, row, scrollable::Scrollable, Button, Column, Container, Row, Text};
use iced::{executor, Application, Color};
use iced::{Background, Settings};
use iced::{Renderer, Sandbox, Theme};

use iced::command::Command;
use rhymalize::ipa_utils::fetching::json::JsonLookupConverter;
use rhymalize::ipa_utils::fetching::IpaConverter;
use rhymalize::ipa_utils::{self, ipa::*};
use serde_json::to_string;
use std::cell::RefCell;

struct Rhyme {
    color: Color,
    members: Vec<Rc<RefCell<DisplaySyllable>>>,
}

struct DisplayWord {
    text: String,
    syllables: Option<Vec<Rc<RefCell<DisplaySyllable>>>>,
}

struct DisplaySyllable {
    syllable: Syllable,
    rhymes: Vec<RhymeSyllable>,
}

struct RhymeSyllable {
    cur: Rc<RefCell<DisplaySyllable>>,
    rhyme: Rc<RefCell<Rhyme>>,
    prev: Option<Rc<RefCell<DisplaySyllable>>>,
    prev_dist: Option<usize>,
    next: Option<Rc<RefCell<DisplaySyllable>>>,
    next_dist: Option<usize>,
}
struct App {
    raw_text: String,
    text: Vec<Vec<DisplayWord>>,
    rhymes: Vec<Rc<RefCell<Rhyme>>>,
}

impl App {
    fn calc_rhyme(&mut self) {
        self.rhymes = vec![];

        let syls: Vec<&Rc<RefCell<DisplaySyllable>>> = self
            .text
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.syllables.iter()))
            .flatten()
            .collect();

        for syl in syls.iter() {
            syl.borrow_mut().rhymes = vec![];
        }

        let colors = [
            Color::from_rgb8(0, 0, 255),
            Color::from_rgb8(0, 255, 0),
            Color::from_rgb8(0, 255, 255),
            Color::from_rgb8(255, 0, 0),
            Color::from_rgb8(255, 0, 255),
            Color::from_rgb8(255, 255, 0),
        ];
        let mut col_index = 0;

        for (i, syl) in syls.iter().enumerate() {
            if !syl.borrow().rhymes.is_empty() {
                // if syllable has rhyme, continue
                continue;
            }
            let new_rhyme = Rc::new(RefCell::new(Rhyme {
                color: colors[col_index],
                members: vec![],
            }));
            let mut added_root_rhyme = false;
            let mut last_rhyme_syl = i;
            for (j, other_syl) in syls.iter().enumerate().skip(i + 1) {
                //println!("{}, {}", syl.borrow().syllable, other_syl.borrow().syllable);
                if other_syl.borrow().syllable.nucleus == syl.borrow().syllable.nucleus {
                    //
                    if !added_root_rhyme {
                        let new_rhyme_syl = RhymeSyllable {
                            cur: Rc::clone(syl),
                            rhyme: Rc::clone(&new_rhyme),
                            prev: None,
                            prev_dist: None,
                            next: None,
                            next_dist: None,
                        };
                        new_rhyme
                            .borrow_mut()
                            .members
                            .push(Rc::clone(&new_rhyme_syl.cur));
                        syl.borrow_mut().rhymes.push(new_rhyme_syl);
                        added_root_rhyme = true;
                    }
                    let dist = Some(j - last_rhyme_syl);
                    if let Some(last_syl) = new_rhyme.borrow().members.last() {
                        if let Some(rhyme_syl) = last_syl.borrow_mut().rhymes.last_mut() {
                            rhyme_syl.next = Some(Rc::clone(other_syl));
                            rhyme_syl.next_dist = dist;
                        }
                    }
                    let new_rhyme_syl = RhymeSyllable {
                        cur: Rc::clone(&other_syl),
                        rhyme: Rc::clone(&new_rhyme),
                        prev: new_rhyme.borrow().members.last().map(Rc::clone),
                        prev_dist: dist,
                        next: None,
                        next_dist: None,
                    };
                    new_rhyme
                        .borrow_mut()
                        .members
                        .push(Rc::clone(&new_rhyme_syl.cur));
                    other_syl.borrow_mut().rhymes.push(new_rhyme_syl);
                    last_rhyme_syl = j;
                }
            }
            col_index += 1;
            if col_index == colors.len() {
                col_index = 0
            }
            if !new_rhyme.borrow().members.is_empty() {
                self.rhymes.push(Rc::clone(&new_rhyme));
            }
        }
    }
}

#[derive(Debug)]
enum Message {
    CalculateRhyme,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let converter = JsonLookupConverter::new(Path::new("./en_US.json")).unwrap();
        let text = fs::read_to_string("./text.txt").unwrap();
        (
            App {
                //text: fs::read_to_string("./text.txt")
                //   .unwrap()
                rhymes: vec![Rc::new(RefCell::new(Rhyme {
                    color: Color::from_rgb8(255, 0, 0),
                    members: vec![],
                }))],
                raw_text: text.clone(),
                text: text
                    .split("\n")
                    .map(|line| {
                        line.split(" ")
                            .map(|word| {
                                let word2 = word
                                    .to_ascii_lowercase()
                                    .trim()
                                    .replace(",", "")
                                    .replace(".", "");
                                let ipas2 = converter.get_ipa_single(&word2);
                                if ipas2.is_err() {
                                    println!("{ipas2:?}")
                                }
                                DisplayWord {
                                    text: word.to_string(),
                                    syllables: if let Ok(ipas) = ipas2 {
                                        Some(
                                            syls_from_word(
                                                &ipas[0], // use first possible pronunciation
                                                &ipa_utils::ipa::english::EnglishSyllableRule,
                                            )
                                            .iter()
                                            .map(|z| {
                                                Rc::new(RefCell::new(DisplaySyllable {
                                                    syllable: z.to_owned(),
                                                    rhymes: vec![],
                                                })) //Some(Color::from_rgb(1.0, 0.0, 0.0)))
                                            })
                                            .collect(),
                                        )
                                    } else {
                                        None
                                    },
                                }
                            })
                            .collect()
                    })
                    .collect(),
            },
            Command::perform(async {}, |_| Message::CalculateRhyme),
        )
    }

    fn title(&self) -> String {
        String::from("Rhymalize")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CalculateRhyme => self.calc_rhyme(),
        }
        Command::none()
    }

    // if let Some(syls) = x.syllables {
    //     Some(Text::new(
    //         syls.iter().map(|(syl, _)| syl).collect::<Syllable>>(),
    //     ))
    // } else {
    //     None
    // },
    //.map(|x| Text::new(x.text.clone()))

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let words = self
            .text
            .iter()
            .map(|line| {
                line.iter()
                    .fold(row!(), |row, words| {
                        let text = Text::new(words.text.clone());
                        // let mut srow = row!();
                        // for (syl, col) in words.syllables.as_ref().unwrap() {
                        //     let stext: Text<Theme, Renderer> = Text::new(syl.to_string());
                        //     srow = srow.push(stext);
                        // }
                        let sylls = if let Some(syllables) = words.syllables.as_ref() {
                            syllables.iter().fold(row!().spacing(5), |srow, syl| {
                                srow.push(
                                    Text::new(syl.borrow().syllable.to_string().clone()).style({
                                        if let Some(rhyme_syl) = syl.borrow().rhymes.first() {
                                            // if let Some(a) = rhyme_syl.prev.as_ref() {
                                            //     print!("{}", a.borrow().syllable)
                                            // } else {
                                            //     print!("None")
                                            // }
                                            // print!(", {}, ", syl.borrow().syllable);
                                            // if let Some(a) = rhyme_syl.next.as_ref() {
                                            //     println!("{}", a.borrow().syllable)
                                            // } else {
                                            //     println!("None")
                                            // }

                                            if [rhyme_syl.next_dist, rhyme_syl.prev_dist]
                                                .iter()
                                                .flatten()
                                                .min()
                                                .map_or(false, |x| x < &6)
                                            {
                                                rhyme_syl.rhyme.borrow().color
                                            } else {
                                                Color::from_rgb(0.9, 0.9, 0.9)
                                            }
                                        } else {
                                            Color::from_rgb(0.9, 0.9, 0.9)
                                        }
                                    }),
                                )
                            })
                        } else {
                            row!()
                        };
                        row.push(column!(text, sylls).align_items(iced::Alignment::Center))
                    })
                    .spacing(5)
            })
            .fold(column!(), |col, row| col.push(row));

        Scrollable::new(Container::new(words).width(iced::Length::Fill).center_x())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    App::run(Settings::default())
}
