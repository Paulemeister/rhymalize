use anyhow::{anyhow, Context};
use scraper::{ElementRef, Html, Node, Selector};
use serde_json::Value;
use std::env;

pub fn get_lyrics(songtitle: &str) -> Result<String, anyhow::Error> {
    async_std::task::block_on(get_lyrics_async(songtitle))
}

pub async fn get_lyrics_async(songtitle: &str) -> Result<String, anyhow::Error> {
    let token = env::var("GENIUS_API_TOKEN")?;

    let client = reqwest::blocking::Client::builder()
        .use_rustls_tls() // needed somehow because of bug maybe?
        .build()?;

    // get song url from api, using first hit of search
    let url = format!(
        "https://api.genius.com/search?q={}&access_token={}",
        songtitle, token
    );

    let request = client.get(&url).build()?;

    let response = client.execute(request)?;

    let genius_res_text = response
        .text()
        .with_context(|| "error reading genius response text")?;
    let result: Value = serde_json::from_str::<Value>(&genius_res_text)
        .with_context(|| format! {"error converting \"{}\" to json",genius_res_text})?;

    /*
    {
        "meta": {...},
        "response": {
            "hits": [
                {
                    ...
                    "result": {
                        ...
                        "url": "https://genius.com/Eminem-rap-god-lyrics",
                        ...
                    }
                },
                ...
            ]
        }
    }
    */

    let song_url = result
        .get("response")
        .with_context(|| "genius api response: no response field")?
        .get("hits")
        .with_context(|| "genius api response: no response.hits field")?
        .get(0)
        .with_context(|| "genius api response: no response.hits.0 field")?
        .get("result")
        .with_context(|| "genius api response: no response.hits.0.result field")?
        .get("url")
        .with_context(|| "genius api response: no response.hits.0.result.url field")?
        .as_str()
        .with_context(|| "genius api response: can't convert response.hits.0.result.url to str")?;

    // get content of song page
    let html_text = client
        .get(song_url)
        .send()
        .context(format!("error sending GET request to {song_url}"))?
        .text()
        .context(format!("error getting response text from {song_url}"))?;
    let html = Html::parse_document(&html_text);

    // scrape text from page
    let selector =
        Selector::parse("div[data-lyrics-container=true]").map_err(|e| anyhow!("{:?}", e))?;

    let lyrics_containers: Vec<_> = html.select(&selector).collect();

    let mut lyrics = String::new(); // passing mut ref to func is not rustlike
    for lyrics_container in lyrics_containers {
        lyrics.push('\n');
        extract_text(&lyrics_container, &mut lyrics);
    }

    Ok(lyrics)
}

fn extract_text(element: &ElementRef, lyrics: &mut String) {
    for child in element.children() {
        match child.value() {
            Node::Element(ref e) => match e.name() {
                "br" => lyrics.push('\n'),
                "div" => (), // skip divs because of additional info in div at the start
                _ => {
                    if let Some(new_element) = ElementRef::wrap(child) {
                        extract_text(&new_element, lyrics);
                    }
                } //maybe specify a|i|span|...
            },
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
