use std::path::Path;
use std::{fs, vec};

use iced::widget::{column, row, Button, Column, Container, Row, Text};
use iced::Color;
use iced::{Background, Settings};
use iced::{Renderer, Sandbox, Theme};

use rhymalize::ipa_utils::fetching::json::JsonLookupConverter;
use rhymalize::ipa_utils::fetching::IpaConverter;
use rhymalize::ipa_utils::{self, ipa::*};
use serde_json::to_string;

struct Rhyme {
    color: Color,
}

#[derive(Clone)]
struct DisplayWord {
    text: String,
    syllables: Option<Vec<(Syllable, Option<Color>)>>,
}
struct App {
    text: Vec<Vec<DisplayWord>>,
}

impl Sandbox for App {
    type Message = ();

    fn new() -> Self {
        let a = "\u{006C}\u{02E0}";
        let converter = JsonLookupConverter::new(Path::new("./en_US.json")).unwrap();
        App {
            //text: fs::read_to_string("./text.txt")
            //   .unwrap()
            text: fs::read_to_string("./text.txt")
                .unwrap()
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
                                            (z.to_owned(), Some(Color::from_rgb(1.0, 0.0, 0.0)))
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
        }
    }

    fn title(&self) -> String {
        String::from("Rhymalize")
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
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
                            syllables
                                .iter()
                                .fold(row!().spacing(5), |srow, (syl, col)| {
                                    srow.push(
                                        Text::new(syl.to_string().clone())
                                            .style(col.unwrap_or_default()),
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

        Container::new(words)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    App::run(Settings::default())
}
