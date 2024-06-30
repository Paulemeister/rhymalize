use anyhow::{anyhow, Context};
use genius_lyrics;
use reqwest;
use serde_json::Value;
use std::env;

pub fn get_lyrics(songtitle: &str) -> Result<String, anyhow::Error> {
    let client = reqwest::blocking::Client::new();

    let token = env::var("GENIUS_API_TOKEN")?;

    let url = format!(
        "https://api.genius.com/search?access_token={}&q={}",
        token, songtitle
    );

    let result: Value = serde_json::from_str::<Value>(&client.get(url).send()?.text()?)?;

    let song_url = result
        .get("response")
        .with_context(|| "no response field")?
        .get(0)
        .with_context(|| "no response.0 field")?
        .get("result")
        .with_context(|| "no response.0.result field")?
        .get("url")
        .with_context(|| "no response.0.result.url field")?
        .as_str()
        .with_context(|| "can't convert to str")?;

    let lyrics =
        genius_lyrics::get_lyrics_from_url_blocking(song_url).map_err(|x| anyhow!("{:?}", x))?;

    Ok(lyrics)
}
