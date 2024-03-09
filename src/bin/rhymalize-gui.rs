use std::borrow::BorrowMut;
use std::path::Path;
use std::{fs, vec};

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

#[derive(Clone)]
struct Rhyme {
    color: Color,
}

#[derive(Clone)]
struct DisplayWord {
    text: String,
    syllables: Option<Vec<RefCell<DisplaySyllable>>>,
}

#[derive(Clone)]
struct DisplaySyllable {
    syllable: Syllable,
    rhymes: Vec<RhymeSyllable>,
}

#[derive(Clone)]
struct RhymeSyllable {
    rhyme: RefCell<Rhyme>,
    prev: Option<RefCell<DisplaySyllable>>,
    prev_dist: Option<usize>,
    next: Option<RefCell<DisplaySyllable>>,
    next_dist: Option<usize>,
}
struct App {
    raw_text: String,
    text: Vec<Vec<DisplayWord>>,
    rhymes: Vec<RefCell<Rhyme>>,
}

impl App {
    fn calc_rhyme(&mut self) {
        for line in &self.text {
            for word in line {
                if let Some(syls) = &word.syllables {
                    for syl in syls {
                        syl.borrow_mut().rhymes = vec![RhymeSyllable {
                            rhyme: self.rhymes[0].clone(),
                            prev: None,
                            prev_dist: None,
                            next: None,
                            next_dist: None,
                        }];
                    }
                }
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
                rhymes: vec![RefCell::new(Rhyme {
                    color: Color::from_rgb8(255, 0, 0),
                })],
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
                                                RefCell::new(DisplaySyllable {
                                                    syllable: z.to_owned(),
                                                    rhymes: vec![],
                                                }) //Some(Color::from_rgb(1.0, 0.0, 0.0)))
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
                                        if let Some(rhyme_syl) = syl.borrow().rhymes.get(0) {
                                            rhyme_syl.rhyme.borrow().color
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
