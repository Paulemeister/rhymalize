use anyhow::{ensure, Context, Error};
use serde_json::{from_str, Value};

static api_url: &str = "https://en.wiktionary.org/w/api.php";

pub fn get_single(word: &str) -> Result<Vec<String>, Error> {
    // id of first hit
    let first_hit_id = get_id(word)?;

    let pron_sec_id = get_pron_sec_id(word)?;
    // get the content of the "pronunciation" section (id=3 ?) of the current revision
    let search_url = format!("{}?format=json&action=query&pageids={}&prop=revisions&rvslots=main&rvprop=content&rvsection={}",api_url,first_hit_id,pron_sec_id);

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

fn get_sec_by_id(page_id: i64, sec_id: i64) -> Result<String, Error> {
    let search_url = format!("{}?format=json&action=query&pageids={}&prop=revisions&rvslots=main&rvprop=content&rvsection={}",api_url,page_id,sec_id);

    let pron_sec_res = from_str::<Value>(reqwest::blocking::get(&search_url)?.text()?.as_str())?;

    Ok(
        pron_sec_res["query"]["pages"][page_id.to_string()]["revisions"][0]["slots"]["main"]["*"]
            .as_str()
            .context("failed to read section as str")?
            .to_string(),
    )
}

fn get_id(word: &str) -> Result<i64, Error> {
    // search for word and get first hit
    let search_url = format!(
        "{}?format=json&action=query&list=search&srwhat=nearmatch&srlimit=1&srsearch={}",
        api_url, word
    );
    let search_response_str = reqwest::blocking::get(&search_url)?.text()?;
    let search_response = from_str::<Value>(search_response_str.as_str())?;
    // id of first hit
    search_response["query"]["search"][0]["pageid"]
        .as_i64()
        .context("converting or reading ID from search failed")
}

fn get_pron_sec_id(word: &str) -> Result<i64, Error> {
    let search_url = format!(
        "{}?format=json&action=parse&prop=sections&page={}",
        api_url, word
    );
    let res = from_str::<Value>(reqwest::blocking::get(&search_url)?.text()?.as_str())?;
    let sections0 = res
        .get("parse")
        .context("no 'parse' in response")?
        .get("sections")
        .context("no 'sections' in response[parse]")?;

    let sections = sections0.as_array().context("couldn't convert to array")?;
    //println!("{:?}", sections);
    //println!("{:?}", sections0);

    for i in 0..sections.len() {
        let section = sections.get(i).context("index not in sections")?;
        let title = section
            .get("line")
            .context("line not in section")?
            .as_str()
            .context("couldn't convert to str")?;

        if title == "Pronunciation" {
            return section
                .get("index")
                .context("'index' not in section")?
                .as_str()
                .context("couldn't convert to str")?
                .parse::<i64>()
                .with_context(|| "couldn't parse str as i64");
        }
    }
    anyhow::bail!("No Pronunciation section found")
}

pub fn get_multiple(words: Vec<&str>) -> Result<Vec<Vec<String>>, Error> {
    let max = 255;
    ensure!(words.len() < max, "Too many words");

    let ids = words
        .iter()
        .map(|&x| get_id(x))
        .collect::<Result<Vec<i64>, Error>>()?;

    let pron_sec_ids = words
        .iter()
        .map(|&x| get_pron_sec_id(x))
        .collect::<Result<Vec<i64>, Error>>()?;
    // get the content of the "pronunciation" section (id=3 ?) of the current revision

    let contents = pron_sec_ids
        .iter()
        .zip(ids.iter())
        .map(|(&sec_id, &page_id)| get_sec_by_id(page_id, sec_id))
        .collect::<Result<Vec<String>, Error>>()?;

    // filter out only the ipa pronunciations

    let mut res = vec![];
    for s in contents.iter() {
        res.push(
            s.split('\n')
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
                .collect::<Vec<String>>(),
        )
    }
    Ok(res)
}
