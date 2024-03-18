use super::IpaConverter;
use anyhow::{anyhow, ensure, Context, Error};
use serde_json::from_str;
use serde_json::Value;
use std::num::NonZeroI128;
static API_URL: &str = "https://en.wiktionary.org/w/api.php";
use futures::{stream, stream::FlatMap, StreamExt, TryFutureExt};
pub struct WiktionaryConverter {
    client: reqwest::Client,
}

impl IpaConverter for WiktionaryConverter {
    fn convert_single(&self, input: &str) -> Result<Vec<String>, anyhow::Error> {
        async_std::task::block_on(self.get_single(input))
    }
    fn convert(&self, inputs: Vec<&str>) -> Vec<Result<Vec<String>, anyhow::Error>> {
        async_std::task::block_on(
            stream::iter(inputs)
                .map(|z| async { self.get_single(z).await })
                .buffered(200)
                .collect::<Vec<_>>(),
        )
    }
}
impl Default for WiktionaryConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl WiktionaryConverter {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    async fn get_single(&self, word: &str) -> Result<Vec<String>, Error> {
        // id of first hit
        let first_hit_id = self.get_id(word).await?;

        let pron_sec_id = self.get_pron_sec_id(first_hit_id).await?;
        // get the content of the "pronunciation" section (id=3 ?) of the current revision
        let search_url = format!("{}?format=json&action=query&pageids={}&prop=revisions&rvslots=main&rvprop=content&rvsection={}",API_URL,first_hit_id,pron_sec_id);

        let pron_sec_res = from_str::<Value>(
            &self
                .client
                .get(&search_url)
                .send()
                .and_then(|x| x.text())
                .await?,
        )?;

        // store the text of the section
        let pron_sec = pron_sec_res["query"]["pages"][first_hit_id.to_string()]["revisions"][0]
            ["slots"]["main"]["*"]
            .as_str()
            .context("failed to read section as str")?;

        //println!("{pron_sec}");
        // filter out only the ipa pronunciations

        let prons: Vec<String> = pron_sec
            .to_string()
            .split("{{")
            .filter(|x| x.starts_with("IPA|en|"))
            .map(|z| z.strip_prefix("IPA|en|").unwrap().split_once("}}"))
            .collect::<Option<Vec<_>>>()
            .with_context(|| "didn't find ending parenthesis")?
            .iter()
            .flat_map(|(a, _)| a.split("|"))
            .map(|x| x.to_string())
            .collect();
        if prons.is_empty() {
            Err(anyhow!("didn't find ipa section"))
        } else {
            Ok(prons)
        }
    }

    async fn get_sec_by_id(&self, page_id: i64, sec_id: i64) -> Result<String, Error> {
        let search_url = format!("{}?format=json&action=query&pageids={}&prop=revisions&rvslots=main&rvprop=content&rvsection={}",API_URL,page_id,sec_id);

        let pron_sec_res =
            from_str::<Value>(&self.client.get(&search_url).send().await?.text().await?)?;

        Ok(
            pron_sec_res["query"]["pages"][page_id.to_string()]["revisions"][0]["slots"]["main"]
                ["*"]
                .as_str()
                .context("failed to read section as str")?
                .to_string(),
        )
    }

    async fn get_id(&self, word: &str) -> Result<i64, Error> {
        // search for word and get first hit
        let search_url = format!(
            "{}?format=json&action=query&list=search&srwhat=nearmatch&srlimit=1&srsearch={}",
            API_URL, word
        );
        let search_response_str = self.client.get(&search_url).send().await?.text().await?;
        let search_response = from_str::<Value>(&search_response_str)?;
        // id of first hit
        search_response["query"]["search"][0]["pageid"]
            .as_i64()
            .context("converting or reading ID from search failed")
    }

    async fn get_pron_sec_id(&self, id: i64) -> Result<i64, Error> {
        let search_url = format!(
            "{}?format=json&action=parse&prop=sections&pageid={}",
            API_URL, id
        );
        let res = from_str::<Value>(&self.client.get(&search_url).send().await?.text().await?)?;
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

    fn test(&self) {
        let a = ["a", "b"];
        let b = a
            .iter()
            .map(|x| self.get_id(x).and_then(|id| self.get_pron_sec_id(id)));
    }
    // async fn get_multiple(&self, words: Vec<&str>) -> Vec<Result<Vec<String>, anyhow::Error>> {
    //     todo!();
    //     const CONCURRENT_REQUESTS: usize = 2;

    //     let pron_sec_ids = futures::stream::iter(words)
    //         .map(|x| {
    //             self.get_id(x)
    //                 .and_then(|z| async { self.get_pron_sec_id(z).await })
    //         })
    //         .buffered(CONCURRENT_REQUESTS)
    //         .collect::<Vec<_>>()
    //         .await;

    //     // get the content of the "pronunciation" section (id=3 ?) of the current revision
    //     let ids = [];
    //     let contents = pron_sec_ids
    //         .iter()
    //         .zip(ids.iter())
    //         .map(|(&sec_id, &page_id)| self.get_sec_by_id(page_id, sec_id))
    //         .collect::<Result<Vec<String>, Error>>()?;

    //     // filter out only the ipa pronunciations

    //     let mut res = vec![];
    //     for s in contents.iter() {
    //         res.push(
    //             s.split('\n')
    //                 .filter(|&x| x.to_owned().starts_with("* {{a|")) // filter wanted lines
    //                 .map(|x| {
    //                     x.split("{{")
    //                         .last() // get second parethesis
    //                         .context(format!("{} doesnt have a last element after splitting", x))
    //                 })
    //                 .collect::<Result<Vec<_>, Error>>()?
    //                 .iter()
    //                 .flat_map(|&x| {
    //                     x.split(['|', '}'])
    //                         .skip(2)
    //                         .filter(|&x| !x.is_empty()) // filter out the last two '{'
    //                         .map(|x| x.to_string())
    //                 })
    //                 .collect::<Vec<String>>(),
    //         )
    //     }
    //     Ok(res)
    // }
}
#[cfg(text)]
mod tests {

    #[test]
    fn test() {
        println!("Hi 1");
        assert_eq!(1, 1);
    }
}
