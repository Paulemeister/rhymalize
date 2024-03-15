use std::io::Read;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, RwLock, RwLockReadGuard, Weak};
use std::{fs, vec};

use iced::futures::lock::Mutex;
use iced::widget::{
    column, row, scrollable::Scrollable, Button, Column, Container, MouseArea, Row, Text,
};
use iced::{executor, Application, Color};
use iced::{Background, Settings};
use iced::{Renderer, Sandbox, Theme};

use iced::command::Command;
use rhymalize::ipa_utils::fetching::IpaConverter;
use rhymalize::ipa_utils::fetching::{json::JsonLookupConverter, wiktionary::WiktionaryConverter};
use rhymalize::ipa_utils::{self, ipa::*};
use serde_json::to_string;
//use std::cell::RefCell;
#[derive(Debug)]
struct Rhyme {
    color: Color,
    members: Vec<Weak<RwLock<DisplaySyllable>>>,
    highlighted: bool,
}
#[derive(Debug)]
struct DisplayWord {
    text: String,
    syllables: Vec<Arc<RwLock<DisplaySyllable>>>,
}

#[derive(Debug)]
struct DisplaySyllable {
    syllable: Syllable,
    rhymes: Vec<Arc<RwLock<RhymeSyllable>>>,
}
#[derive(Debug)]
struct RhymeSyllable {
    cur: Weak<RwLock<DisplaySyllable>>,
    rhyme: Weak<RwLock<Rhyme>>,
    prev: Option<Weak<RwLock<DisplaySyllable>>>,
    prev_dist: Option<usize>,
    next: Option<Weak<RwLock<DisplaySyllable>>>,
    next_dist: Option<usize>,
}
struct App {
    raw_text: String,
    text: Vec<Vec<Arc<RwLock<DisplayWord>>>>,
    rhymes: Vec<Arc<RwLock<Rhyme>>>,
}

impl App {
    fn calc_rhyme(&mut self) -> Command<Message> {
        self.rhymes = vec![];

        let syls: Vec<Arc<RwLock<DisplaySyllable>>> = self
            .text
            .iter()
            .flat_map(|x| x.iter().map(|y| y.read().unwrap().syllables.clone()))
            .flatten()
            .collect();

        for syl in syls.iter() {
            syl.write().unwrap().rhymes = vec![];
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
            {
                if !syl.read().unwrap().rhymes.is_empty() {
                    // if syllable has rhyme, continue
                    continue;
                }
            }
            let new_rhyme = Arc::new(RwLock::new(Rhyme {
                color: colors[col_index],
                members: vec![],
                highlighted: false,
            }));
            let mut added_root_rhyme = false;
            let mut last_rhyme_syl = i;
            for (j, other_syl) in syls.iter().enumerate().skip(i + 1) {
                //println!("{}, {}", syl.read().unwrap().syllable, other_syl.read().unwrap().syllable);
                if other_syl.read().unwrap().syllable.nucleus
                    == syl.read().unwrap().syllable.nucleus
                {
                    //
                    if !added_root_rhyme {
                        let new_rhyme_syl = Arc::new(RwLock::new(RhymeSyllable {
                            cur: Arc::downgrade(syl),
                            rhyme: Arc::downgrade(&new_rhyme),
                            prev: None,
                            prev_dist: None,
                            next: None,
                            next_dist: None,
                        }));
                        new_rhyme.write().unwrap().members.push(Arc::downgrade(syl));
                        syl.write().unwrap().rhymes.push(new_rhyme_syl);
                        added_root_rhyme = true;
                    }
                    let dist = Some(j - last_rhyme_syl);
                    if let Some(last_syl) = new_rhyme.read().unwrap().members.last() {
                        if let Some(rhyme_syl) = last_syl
                            .upgrade()
                            .unwrap()
                            .write()
                            .unwrap()
                            .rhymes
                            .last_mut()
                        {
                            rhyme_syl.write().unwrap().next = Some(Arc::downgrade(other_syl));
                            rhyme_syl.write().unwrap().next_dist = dist;
                        }
                    }
                    let new_rhyme_syl = Arc::new(RwLock::new(RhymeSyllable {
                        cur: Arc::downgrade(other_syl),
                        rhyme: Arc::downgrade(&new_rhyme),
                        prev: new_rhyme
                            .read()
                            .unwrap()
                            .members
                            .last()
                            .map(|z| Weak::clone(z)),
                        prev_dist: dist,
                        next: None,
                        next_dist: None,
                    }));
                    new_rhyme
                        .write()
                        .unwrap()
                        .members
                        .push(Arc::downgrade(other_syl));
                    other_syl.write().unwrap().rhymes.push(new_rhyme_syl);
                    last_rhyme_syl = j;
                }
            }
            col_index += 1;
            if col_index == colors.len() {
                col_index = 0
            }
            if !new_rhyme.read().unwrap().members.is_empty() {
                self.rhymes.push(Arc::clone(&new_rhyme));
            }
        }
        Command::none()
    }

    fn get_syllables(&mut self) -> Command<Message> {
        let converter = JsonLookupConverter::new(Path::new(
            "/home/paulemeister/Code/Rust/rhymalize/en_US.json",
        ))
        .unwrap();
        //let converter = WiktionaryConverter {};

        for word in self.text.iter_mut().flat_map(|x| x.iter_mut()) {
            let word2 = word
                .read()
                .unwrap()
                .text
                .to_ascii_lowercase()
                .trim()
                .replace(",", "")
                .replace(".", "");
            let ipas2 = converter.get_ipa_single(&word2);
            if ipas2.is_err() {
                println!("{ipas2:?}")
            }
            word.write().unwrap().syllables = if let Ok(ipas) = ipas2 {
                syls_from_word(
                    &ipas[0], // use first possible pronunciation
                    &ipa_utils::ipa::english::EnglishSyllableRule,
                )
                .iter()
                .map(|z| {
                    Arc::new(RwLock::new(DisplaySyllable {
                        syllable: z.to_owned(),
                        rhymes: vec![],
                    })) //Some(Color::from_rgb(1.0, 0.0, 0.0)))
                })
                .collect()
            } else {
                vec![]
            }
        }
        Command::perform(async {}, |_| Message::CalculateRhyme)
    }

    fn test(&mut self, input: Weak<RwLock<DisplaySyllable>>, highlight: bool) -> Command<Message> {
        let a = input.upgrade().unwrap();
        println!("{}", a.read().unwrap().syllable);
        a.read()
            .unwrap()
            .rhymes
            .first()
            .unwrap()
            .read()
            .unwrap()
            .rhyme
            .upgrade()
            .unwrap()
            .write()
            .unwrap()
            .highlighted = highlight;
        Command::none()
    }
}

#[derive(Debug, Clone)]
enum Message {
    CalculateRhyme,
    GetSyllables,
    HighlightRhyme(Weak<RwLock<DisplaySyllable>>),

    DehighlightRhyme(Weak<RwLock<DisplaySyllable>>),
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let text = fs::read_to_string("/home/paulemeister/Code/Rust/rhymalize/text.txt").unwrap();
        (
            App {
                //text: fs::read_to_string("./text.txt")
                //   .unwrap()
                rhymes: vec![],
                raw_text: text.clone(),
                text: text
                    .split("\n")
                    .map(|line| {
                        line.split(" ")
                            .map(|word| {
                                Arc::new(RwLock::new(DisplayWord {
                                    text: word.to_string(),
                                    syllables: vec![],
                                }))
                            })
                            .collect()
                    })
                    .collect(),
            },
            Command::perform(async {}, |_| Message::GetSyllables),
        )
    }

    fn title(&self) -> String {
        String::from("Rhymalize")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::CalculateRhyme => self.calc_rhyme(),
            Message::GetSyllables => self.get_syllables(),
            Message::HighlightRhyme(a) => self.test(a, true),

            Message::DehighlightRhyme(a) => self.test(a, false),
            _ => Command::none(),
        }
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
                        let text = Text::new(words.read().unwrap().text.clone());
                        // let mut srow = row!();
                        // for (syl, col) in words.syllables.as_ref().unwrap() {
                        //     let stext: Text<Theme, Renderer> = Text::new(syl.to_string());
                        //     srow = srow.push(stext);
                        // }
                        let syllables = &words.read().unwrap().syllables;
                        let sylls = syllables.iter().fold(row!().spacing(5), |srow, syl| {
                            srow.push(
                                MouseArea::new(
                                    Container::new(
                                        Text::new(syl.read().unwrap().syllable.to_string().clone())
                                            .style({
                                                if let Some(rhyme_syl) =
                                                    syl.read().unwrap().rhymes.first()
                                                {
                                                    // if let Some(a) = rhyme_syl.prev.as_ref() {
                                                    //     print!("{}", a.read().unwrap().syllable)
                                                    // } else {
                                                    //     print!("None")
                                                    // }
                                                    // print!(", {}, ", syl.read().unwrap().syllable);
                                                    // if let Some(a) = rhyme_syl.next.as_ref() {
                                                    //     println!("{}", a.read().unwrap().syllable)
                                                    // } else {
                                                    //     println!("None")
                                                    // }

                                                    if [
                                                        rhyme_syl.read().unwrap().next_dist,
                                                        rhyme_syl.read().unwrap().prev_dist,
                                                    ]
                                                    .iter()
                                                    .flatten()
                                                    .min()
                                                    .map_or(false, |x| x < &6)
                                                    {
                                                        rhyme_syl
                                                            .read()
                                                            .unwrap()
                                                            .rhyme
                                                            .upgrade()
                                                            .unwrap()
                                                            .read()
                                                            .unwrap()
                                                            .color
                                                    } else {
                                                        Color::from_rgb(0.9, 0.9, 0.9)
                                                    }
                                                } else {
                                                    Color::from_rgb(0.9, 0.9, 0.9)
                                                }
                                            }),
                                    )
                                    .style(
                                        if if let Some(r) = syl.read().unwrap().rhymes.first() {
                                            r.read()
                                                .unwrap()
                                                .rhyme
                                                .upgrade()
                                                .unwrap()
                                                .read()
                                                .unwrap()
                                                .highlighted
                                        } else {
                                            false
                                        } {
                                            iced::widget::container::Appearance::default()
                                                .with_background(Color::from_rgb(0.9, 0.9, 0.9))
                                        } else {
                                            iced::widget::container::Appearance::default()
                                        },
                                    ),
                                )
                                .on_enter(Message::HighlightRhyme(Arc::downgrade(syl)))
                                .on_exit(Message::DehighlightRhyme(Arc::downgrade(syl))),
                            )
                        });
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
