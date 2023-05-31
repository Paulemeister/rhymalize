use anyhow::{ensure, Context, Error};
use serde_json::{from_str, Value};

pub fn get_single(word: &str) -> Result<Vec<String>, Error> {
    let api_url = "https://en.wiktionary.org/w/api.php";

    // search for word and get first hit
    let search_url = format!(
        "{}?format=json&action=query&list=search&srprop=redirecttitle&srlimit=1&srsearch={}",
        api_url, word
    );
    let search_response_str = reqwest::blocking::get(&search_url)?.text()?;
    let search_response = from_str::<Value>(search_response_str.as_str())?;
    // id of first hit
    let first_hit_id: &i64 = &search_response["query"]["search"][0]["pageid"]
        .as_i64()
        .context("converting or reading ID from search failed")?;

    // get the content of the "pronunciation" section (id=3 ?) of the current revision
    let search_url = format!("{}?format=json&action=query&pageids={}&prop=revisions&rvslots=main&rvprop=content&rvsection=3",api_url,first_hit_id);

    let pron_sec_res = from_str::<Value>(reqwest::blocking::get(&search_url)?.text()?.as_str())?;

    // store the text of the section
    let pron_sec = pron_sec_res["query"]["pages"][first_hit_id.to_string()]["revisions"][0]
        ["slots"]["main"]["*"]
        .as_str()
        .context("failed to read section as str")?;

    // filter out only the ipa pronunciations
    Ok(pron_sec
        .split('\n')
        .filter(|&x| x.to_owned().starts_with("* {{a|")) // filter wanted lines
        .map(|x| {
            x.split("{{")
                .last() // get second parethesis
                .context(format!("{} doesnt have a last element after splitting", x))
        })
        .collect::<Result<Vec<_>, Error>>()?
        .iter()
        .flat_map(|&x| {
            x.split(['|', '}'])
                .skip(2)
                .filter(|&x| !x.is_empty()) // filter out the last two '{'
                .map(|x| x.to_string())
        })
        .collect::<Vec<String>>())
}

pub fn get_multiple(words: Vec<&str>) -> Result<Vec<Vec<String>>, Error> {
    let max = 255;
    ensure!(words.len() > max, "Too many words");

    let api_url = "https://en.wiktionary.org/w/api.php";

    // search for word and get first hit
    let search_url = format!(
        "{}?format=json&action=query&list=search&srwhat=nearmatch&srlimit=1&srsearch={}",
        api_url,
        words.join("|")
    );
    let search_response_str = reqwest::blocking::get(&search_url)?.text()?;
    let search_response = from_str::<Value>(search_response_str.as_str())?;
    // id of first hit
    let first_hit_id: &i64 = &search_response["query"]["search"][0]["pageid"]
        .as_i64()
        .context("converting or reading ID from search failed")?;

    // get the content of the "pronunciation" section (id=3 ?) of the current revision
    let search_url = format!("{}?format=json&action=query&pageids={}&prop=revisions&rvslots=main&rvprop=content&rvsection=3",api_url,first_hit_id);

    let pron_sec_res = from_str::<Value>(reqwest::blocking::get(&search_url)?.text()?.as_str())?;

    // store the text of the section
    let pron_sec = pron_sec_res["query"]["pages"][first_hit_id.to_string()]["revisions"][0]
        ["slots"]["main"]["*"]
        .as_str()
        .context("failed to read section as str")?;

    // filter out only the ipa pronunciations
    Ok(vec![pron_sec
        .split('\n')
        .filter(|&x| x.to_owned().starts_with("* {{a|")) // filter wanted lines
        .map(|x| {
            x.split("{{")
                .last() // get second parethesis
                .context(format!("{} doesnt have a last element after splitting", x))
        })
        .collect::<Result<Vec<_>, Error>>()?
        .iter()
        .flat_map(|&x| {
            x.split(['|', '}'])
                .skip(2)
                .filter(|&x| !x.is_empty()) // filter out the last two '{'
                .map(|x| x.to_string())
        })
        .collect::<Vec<String>>()])
}
