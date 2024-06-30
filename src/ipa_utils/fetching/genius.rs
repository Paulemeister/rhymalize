use anyhow::{anyhow, Context};
use reqwest;
use scraper::{ElementRef, Html, Node, Selector};
use serde_json::Value;
use std::env;

pub fn get_lyrics(songtitle: &str) -> Result<String, anyhow::Error> {
    async_std::task::block_on(get_lyrics_async(songtitle))
}

pub async fn get_lyrics_async(songtitle: &str) -> Result<String, anyhow::Error> {
    let token = env::var("GENIUS_API_TOKEN")?;
    let client = reqwest::blocking::Client::new();

    let url = format!(
        "https://api.genius.com/search?access_token={}&q={}",
        token, songtitle
    );

    let result: Value = serde_json::from_str::<Value>(&client.get(url).send()?.text()?)?;

    let song_url = result
        .get("response")
        .with_context(|| "no response field")?
        .get("hits")
        .with_context(|| "no response.hits field")?
        .get(0)
        .with_context(|| "no response.hits.0 field")?
        .get("result")
        .with_context(|| "no response.hits.0.result field")?
        .get("url")
        .with_context(|| "no response.hits.0.result.url field")?
        .as_str()
        .with_context(|| "can't convert to str")?;

    let html_text = client.get(song_url).send()?.text()?;
    let html = Html::parse_document(&html_text);

    let selector =
        Selector::parse("div[data-lyrics-container=true]").map_err(|e| anyhow!("{:?}", e))?;

    let lyrics_containers: Vec<_> = html.select(&selector).collect();

    let mut lyrics = String::new();
    for lyrics_container in lyrics_containers {
        lyrics.push('\n');
        extract_text(&lyrics_container, &mut lyrics);
    }

    Ok(lyrics)
}

fn extract_text(element: &ElementRef, lyrics: &mut String) {
    for child in element.children() {
        match child.value() {
            Node::Element(ref e) if e.name() == "br" => {
                lyrics.push('\n');
            }
            Node::Element(e) => {
                if let Some(new_element) = ElementRef::wrap(child) {
                    extract_text(&new_element, lyrics);
                }
            }
            Node::Text(ref text) => {
                lyrics.push_str(text);
            }
            Node::Comment(_) => {}
            Node::Doctype(_) => {}
            _ => {
                println!("Encountered unsupported node: {:?}", child.value());
            }
        }
    }
}
