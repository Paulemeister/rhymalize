use std::{fs, vec};

use iced::widget::{column, row, Button, Column, Container, Row, Text};
use iced::Color;
use iced::{Background, Settings};
use iced::{Renderer, Sandbox, Theme};

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
        App {
            //text: fs::read_to_string("./text.txt")
            //   .unwrap()
            text: [
                vec![
                    ("Time", vec!["/ˈtaɪm/"]),
                    ("to", vec!["/ˈtu/", "/tə/", "/tɪ/"]),
                    ("get", vec!["/ˈɡɛt/", "/ˈɡɪt/"]),
                    ("creative", vec!["/kɹiˈeɪtɪv/"]),
                    ("y'all", vec!["/ˌjɔɫ/"]),
                    ("know", vec!["/ˈnoʊ/"]),
                    ("coffee", vec!["/ˈkɑfi/", "/ˈkɔfi/"]),
                    ("is", vec!["/ˈɪz/", "/ɪz/"]),
                    ("the", vec!["/ˈðə/", "/ðə/", "/ði/"]),
                    ("drug", vec!["/ˈdɹəɡ/"]),
                    ("of", vec!["/ˈəv/"]),
                    ("choice/n", vec!["/ˈtʃɔɪs/"]),
                ],
                vec![
                    ("Knockin'", vec!["/ˈnɑkɪŋ/"]),
                    ("all", vec!["/ˈɔɫ/"]),
                    ("them", vec!["/ˈðɛm/", "/ðəm/"]),
                    ("sloppy", vec!["/ˈsɫɑpi/"]),
                    ("demons", vec!["/ˈdimənz/"]),
                    ("off", vec!["/ˈɔf/"]),
                    ("me", vec!["/ˈmi/"]),
                    ("just", vec!["/ˈdʒəst/", "/dʒɪst/"]),
                    ("to", vec!["/ˈtu/", "/tə/", "/tɪ/"]),
                    ("hush", vec!["/ˈhəʃ/"]),
                    ("the", vec!["/ˈðə/", "/ðə/", "/ði/"]),
                    ("noise", vec!["/ˈnɔɪz/"]),
                ],
            ]
            .iter()
            .map(|line| {
                line.iter()
                    .map(|(word, ipas)| DisplayWord {
                        text: word.to_string(),
                        syllables: Some(
                            syls_from_word(
                                &Word::try_from(ipas[0]).unwrap_or(Word::default()),
                                &ipa_utils::ipa::english::EnglishSyllableRule,
                            )
                            .iter()
                            .map(|z| (z.to_owned(), Some(Color::from_rgb(1.0, 0.0, 0.0))))
                            .collect(),
                        ),
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
                        let sylls = words.syllables.as_ref().unwrap().iter().fold(
                            row!().spacing(5),
                            |srow, (syl, col)| {
                                srow.push(
                                    Text::new(syl.to_string().clone())
                                        .style(col.unwrap_or_default()),
                                )
                            },
                        );
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
