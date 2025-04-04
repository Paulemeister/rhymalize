use std::path::Path;
use std::sync::{Arc, RwLock, Weak};
use std::{fs, vec};

use anyhow::Context;
use futures::{SinkExt, Stream, StreamExt};
use iced::stream::{channel, try_channel};
use iced::widget::{button, Column, Row};
use iced::widget::{
    column, container, row, scrollable::Scrollable, text, text_input, Container, MouseArea, Text,
};

use iced::futures::channel::mpsc;
use iced::task::Task;
use iced::{Color, Subscription};
use rhymalize::ipa_utils::fetching::IpaConverter;
use rhymalize::ipa_utils::fetching::{json::JsonLookupConverter, wiktionary::WiktionaryConverter};
use rhymalize::ipa_utils::{self, ipa::*};

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

#[derive(Debug, Clone)]
struct DisplaySyllable {
    syllable: Syllable,
    rhymes: Vec<Arc<RwLock<RhymeSyllable>>>,
}
#[allow(dead_code)]
#[derive(Debug)]
struct RhymeSyllable {
    cur: Weak<RwLock<DisplaySyllable>>,
    rhyme: Weak<RwLock<Rhyme>>,
    prev: Option<Weak<RwLock<DisplaySyllable>>>,
    prev_dist: Option<usize>,
    next: Option<Weak<RwLock<DisplaySyllable>>>,
    next_dist: Option<usize>,
}
#[derive(Debug, Clone)]
enum GetSylMessage {
    Started(
        mpsc::Sender<(
            Vec<Arc<RwLock<DisplayWord>>>,
            Arc<RwLock<JsonLookupConverter>>,
        )>,
    ),
    Processed((Arc<RwLock<DisplayWord>>, Vec<DisplaySyllable>)),
    Finished,
}
#[allow(dead_code)]
struct App {
    raw_text: String,
    text: Vec<Vec<Arc<RwLock<DisplayWord>>>>,
    rhymes: Vec<Arc<RwLock<Rhyme>>>,
    get_syl: bool,
    ipa_converter: Arc<RwLock<JsonLookupConverter>>,
    input_field_text: String,
}

impl App {
    fn calc_rhyme(&mut self) -> Task<Message> {
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
            Color::from_rgb8(135, 255, 0),
            Color::from_rgb8(255, 102, 0),
            Color::from_rgb8(255, 0, 153),
            Color::from_rgb8(102, 0, 255),
            Color::from_rgb8(0, 153, 255),
            Color::from_rgb8(0, 255, 102),
            Color::from_rgb8(0, 255, 0),
            Color::from_rgb8(255, 255, 0),
            Color::from_rgb8(255, 0, 0),
            Color::from_rgb8(255, 0, 255),
            Color::from_rgb8(0, 0, 255),
            Color::from_rgb8(0, 255, 255),
            Color::from_rgb8(204, 255, 153),
            Color::from_rgb8(255, 204, 153),
            Color::from_rgb8(255, 153, 201),
            Color::from_rgb8(204, 153, 255),
            Color::from_rgb8(153, 204, 255),
            Color::from_rgb8(153, 255, 204),
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
                        prev: new_rhyme.read().unwrap().members.last().map(Weak::clone),
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
        Task::none()
    }

    fn get_syl_subscription() -> impl Stream<Item = GetSylMessage> {
        channel(1, move |mut output| async move {
            let (sender, mut receiver) = iced::futures::channel::mpsc::channel(100);

            _ = output.send(GetSylMessage::Started(sender)).await;

            let disp_words: Vec<Arc<RwLock<DisplayWord>>>;
            let ipa_converter_arc: Arc<RwLock<JsonLookupConverter>>;
            (disp_words, ipa_converter_arc) = receiver.select_next_some().await;

            let ipa_converter = ipa_converter_arc.read().unwrap().clone();

            // disp_words: Vec<Arc<RwLock<DisplayWord>>>,
            // ipa_converter: Arc<RwLock<JsonLookupConverter>>,

            for arc in disp_words {
                if let Ok(mut disp_word) = arc.write() {
                    let word_str = disp_word
                        .text
                        .to_ascii_lowercase()
                        .trim()
                        .replace([',', '.', '!', '?', ';', '.', ',', '"', '\''], "");

                    let syllables =
                        App::get_disp_syllables_from_word_str2(&word_str, &ipa_converter)
                            .unwrap_or_default();

                    disp_word.syllables = syllables;
                    // _ = output
                    //     .send(GetSylMessage::Processed((arc.clone(), syllables)))
                    //     .await;
                }
            }
            _ = output.send(GetSylMessage::Finished).await;
        })
    }

    fn get_disp_syllables_from_word_str2(
        word_str: &str,
        converter: &impl IpaConverter,
    ) -> Result<Vec<Arc<RwLock<DisplaySyllable>>>, anyhow::Error> {
        let ipas = converter.get_ipa_single(word_str)?;
        let word = ipas.first().with_context(|| "possible ipa vec was empty")?;

        Ok(
            syls_from_word(&word, &ipa_utils::ipa::english::EnglishSyllableRule)
                .iter()
                .map(|z| {
                    Arc::new(RwLock::new(DisplaySyllable {
                        syllable: z.to_owned(),
                        rhymes: vec![],
                    })) //Some(Color::from_rgb(1.0, 0.0, 0.0)))
                })
                .collect(),
        )
    }

    fn get_disp_syllables_from_word_str(
        &self,
        word_str: &str,
        converter: &impl IpaConverter,
    ) -> Result<Vec<Arc<RwLock<DisplaySyllable>>>, anyhow::Error> {
        let ipas = converter.get_ipa_single(word_str)?;
        let word = ipas.first().with_context(|| "possible ipa vec was empty")?;

        Ok(
            syls_from_word(&word, &ipa_utils::ipa::english::EnglishSyllableRule)
                .iter()
                .map(|z| {
                    Arc::new(RwLock::new(
                        DisplaySyllable {
                            syllable: z.to_owned(),
                            rhymes: vec![],
                        }, //Some(Color::from_rgb(1.0, 0.0, 0.0)))
                    ))
                })
                .collect(),
        )
    }

    fn get_syllables(&mut self) -> Task<Message> {
        let converter = WiktionaryConverter::new();
        let converter = JsonLookupConverter::new(Path::new("./en_US.json")).unwrap();

        let disp_words = self
            .text
            .iter()
            .flat_map(|x| x.iter())
            .flat_map(|z| z.write());

        for mut word in disp_words {
            let word_str = word
                .text
                .to_ascii_lowercase()
                .trim()
                .replace([',', '.', '!', '?', ';', '.', ',', '"', '\''], "");

            word.syllables = self
                .get_disp_syllables_from_word_str(&word_str, &converter)
                .unwrap_or_default();
        }

        Task::none()
    }

    fn test(&mut self, input: Weak<RwLock<DisplaySyllable>>, highlight: bool) -> Task<Message> {
        let a = input.upgrade().unwrap();
        println!("{}", a.read().unwrap().syllable);

        // if let Some(mut b) = a
        //     .read()
        //     .ok()
        //     .and_then(|x| {
        //         x.rhymes
        //             .first()
        //             .and_then(|x| x.read().ok())
        //             .and_then(|x| x.rhyme.upgrade())
        //     })
        //     .and_then(|z| z.write().ok().map(|x| x.))
        // {
        //     b.highlighted = highlight
        // };
        #[allow(clippy::option_map_unit_fn)]
        a.read()
            .ok()
            .and_then(|x| {
                x.rhymes
                    .first()
                    .and_then(|x| x.read().ok())
                    .and_then(|x| x.rhyme.upgrade())
            })
            .map(|x| {
                if let Ok(mut b) = x.write() {
                    b.highlighted = highlight
                }
            });

        Task::none()
    }

    fn load_text(&mut self) -> Task<Message> {
        self.text = self
            .raw_text
            .split('\n')
            .map(|line| {
                line.split(' ')
                    .map(|word| {
                        Arc::new(RwLock::new(DisplayWord {
                            text: word.to_string(),
                            syllables: vec![],
                        }))
                    })
                    .collect()
            })
            .collect();

        Task::none()
    }
    fn set_text_from_genius_search(&mut self) -> Task<Message> {
        self.raw_text = match crate::ipa_utils::fetching::genius::get_lyrics(&self.input_field_text)
        {
            Ok(a) => a,
            Err(e) => format!("Error getting '{}':\n{}", &self.input_field_text, e),
        };

        self.text = vec![];
        self.rhymes = vec![];
        Task::none()
    }
}

#[derive(Debug, Clone)]
enum Message {
    LoadText,
    CalculateRhyme,
    GetSyllables,
    HighlightRhyme(Weak<RwLock<DisplaySyllable>>),
    InputFieldChanged(String),
    SetGeniusLyrics,
    DehighlightRhyme(Weak<RwLock<DisplaySyllable>>),
    GetSylMessage(GetSylMessage),
}

impl App {
    fn new(_flags: ()) -> (Self, Task<Message>) {
        let text = fs::read_to_string("./text.txt").unwrap();
        //let text = crate::ipa_utils::fetching::genius::get_lyrics("Rapgod").unwrap();

        (
            App {
                //text: fs::read_to_string("./text.txt")
                //   .unwrap()
                rhymes: vec![],
                raw_text: text.clone(),
                text: vec![],
                get_syl: false,
                ipa_converter: Arc::new(RwLock::new(
                    JsonLookupConverter::new(Path::new("./en_US.json")).unwrap(),
                )),
                input_field_text: "".into(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rhymalize")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GetSylMessage(m) => match m {
                GetSylMessage::Started(mut s) => {
                    let words = self
                        .text
                        .iter()
                        .flat_map(|x| x.iter())
                        .map(|x| x.clone())
                        .collect();

                    let converter = self.ipa_converter.clone();
                    s.send((words, converter));
                    Task::none()
                }
                GetSylMessage::Finished => Task::none(),
                GetSylMessage::Processed((word_arc, syls)) => {
                    if let Ok(mut word) = word_arc.write() {
                        word.syllables = syls
                            .iter()
                            .map(|x| Arc::new(RwLock::new(x.clone())))
                            .collect();
                    }
                    Task::none()
                }
            },
            Message::LoadText => self.load_text(),
            Message::CalculateRhyme => self.calc_rhyme(),
            Message::GetSyllables => self.get_syllables(),
            Message::HighlightRhyme(a) => self.test(a, true),
            Message::SetGeniusLyrics => self.set_text_from_genius_search(),
            Message::DehighlightRhyme(a) => self.test(a, false),
            Message::InputFieldChanged(a) => {
                self.input_field_text = a;
                Task::none()
            }
        }
    }
    fn view(&self) -> iced::Element<'_, Message> {
        fn make_row<'a>(line: &[Arc<RwLock<DisplayWord>>]) -> Row<'a, Message> {
            line.iter()
                .fold(row!(), |row, words| row.push(make_text_ipa_col(words)))
                .spacing(5)
        }
        fn make_syl_row<'a>(syls: Vec<Arc<RwLock<DisplaySyllable>>>) -> Row<'a, Message> {
            let mut row: Row<'_, Message> = row!();
            //let mut highlighted = Some(false);
            //let mut color = Color::default();

            for syl in syls {
                let highlighted = syl.read().ok().and_then(|x| {
                    x.rhymes.first().and_then(|x| {
                        x.read()
                            .ok()
                            .and_then(|x| x.rhyme.upgrade())
                            .and_then(|x| x.read().ok().map(|x| x.highlighted))
                    })
                });

                let container_style = if highlighted.unwrap_or(false) {
                    iced::widget::container::Style::default()
                        .background(Color::from_rgb(0.9, 0.9, 0.9))
                } else {
                    iced::widget::container::Style::default()
                };

                let text_color = syl
                    .read()
                    .unwrap()
                    .rhymes
                    .first()
                    .and_then(|x| x.read().ok())
                    .and_then(|rsyl| {
                        let under_max_dist = [rsyl.next_dist, rsyl.prev_dist]
                            .iter()
                            .flatten()
                            .min()
                            .map(|x| x < &6)
                            .unwrap_or(false);
                        let default_color = Color::from_rgb(0.9, 0.9, 0.9);
                        if under_max_dist {
                            rsyl.rhyme
                                .upgrade()
                                .and_then(|x| x.read().ok().map(|x| x.color))
                        } else {
                            Some(default_color)
                        }
                    });
                let syl_text_style = iced::widget::text::Style { color: text_color };

                row = row.push(
                    MouseArea::new(
                        container(
                            text!("{}", syl.read().unwrap().syllable)
                                .style(move |_| syl_text_style),
                        )
                        .style(move |_| container_style),
                    )
                    .on_enter(Message::HighlightRhyme(Arc::downgrade(&syl)))
                    .on_exit(Message::DehighlightRhyme(Arc::downgrade(&syl))),
                );
            }

            row.spacing(5)
        }
        fn make_text_ipa_col<'a>(word: &Arc<RwLock<DisplayWord>>) -> Column<'a, Message> {
            let text = Text::new(word.read().unwrap().text.clone());
            let syl_row = make_syl_row(word.read().unwrap().syllables.clone());

            column!(text, syl_row).align_x(iced::Alignment::Center)
        }
        fn make_lyrics_column<'a>(app: &App) -> Column<'a, Message> {
            app.text
                .iter()
                .fold(column!(), |col, line| col.push(make_row(line)))
        }

        let words = make_lyrics_column(self);
        column!(
            row!(
                button("Load Text").on_press(Message::LoadText),
                button("Load IPA").on_press(Message::GetSyllables),
                button("Calculate Rhymes").on_press(Message::CalculateRhyme),
                text_input("Title", &self.input_field_text)
                    .on_input(Message::InputFieldChanged)
                    .on_submit(Message::SetGeniusLyrics)
            )
            .width(iced::Length::Fill),
            Scrollable::new(
                Container::new(words)
                    .width(iced::Length::Fill)
                    .center_x(iced::Length::Fill),
            )
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
        )
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.get_syl {
            let ipa_converter = self.ipa_converter.clone();
            Subscription::run(App::get_syl_subscription).map(Message::GetSylMessage)
        } else {
            Subscription::none()
        }
    }
}

fn main() -> Result<(), iced::Error> {
    dotenv::dotenv().ok();
    iced::application(App::title, App::update, App::view).run_with(|| App::new(()))
}
