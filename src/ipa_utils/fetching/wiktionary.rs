use super::IpaConverter;
use anyhow::bail;
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
    fn convert(&self, inputs: &Vec<&str>) -> Vec<Result<Vec<String>, anyhow::Error>> {
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
    async fn extract_ipa_text_helper(
        &self,
        sec_id: i64,
        page_id: i64,
    ) -> Result<Vec<String>, Error> {
        // get the content of the "pronunciation" section (id=3 ?) of the current revision
        let search_url = format!("{}?format=json&action=query&pageids={}&prop=revisions&rvslots=main&rvprop=content&rvsection={}",API_URL,page_id,sec_id);

        let pron_sec_res = from_str::<Value>(
            &self
                .client
                .get(&search_url)
                .send()
                .and_then(|x| x.text())
                .await?,
        )
        .context("making value from pron_sec failed")?;

        // store the text of the section
        let pron_sec = pron_sec_res
            .get("query")
            .context("no query field")?
            .get("pages")
            .context("no index field")?
            .get(page_id.to_string())
            .context("didn't find id")?
            .get("revisions")
            .context("no revisions field")?[0]["slots"]["main"]["*"]
            .as_str()
            .context("failed to read section as str")?;

        println!("{pron_sec}");
        // filter out only the ipa pronunciations

        let prons: Vec<String> = pron_sec
            .to_string()
            .split("{{")
            .filter(|x| x.starts_with("IPA|en|") || x.starts_with("IPA-lite|en|"))
            .map(|z| z.split_once("}}"))
            .collect::<Option<Vec<_>>>()
            .with_context(|| "didn't find ending parenthesis")?
            .iter()
            .flat_map(|(a, _)| a.split("|"))
            .skip(2)
            .filter(|x| x.starts_with('/') || x.starts_with('['))
            .map(|x| x.to_string())
            .collect();
        if !prons.is_empty() {
            Ok(prons)
        } else {
            bail!("couldn't find ipa section")
        }
    }
    async fn get_single(&self, word: &str) -> Result<Vec<String>, Error> {
        // id of first hit
        let page_id = self
            .get_id(word)
            .await
            .context("getting first_hit_id failed")?;

        let pron_sec_ids = self
            .get_pron_sec_ids(page_id)
            .await
            .context("getting pron_sec_id failed")?;

        let mut last_err = anyhow!("didn't find ipa section");

        for pron_sec_id in pron_sec_ids {
            let res = self.extract_ipa_text_helper(pron_sec_id, page_id).await;
            match res {
                Ok(ipas) => return Ok(ipas),
                Err(e) => last_err = e,
            }
        }
        Err(last_err)
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

    async fn get_pron_sec_ids(&self, id: i64) -> Result<Vec<i64>, Error> {
        let mut result = vec![];
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
                result.push(
                    section
                        .get("index")
                        .context("'index' not in section")?
                        .as_str()
                        .context("couldn't convert to str")?
                        .parse::<i64>()
                        .with_context(|| "couldn't parse str as i64")?,
                );
            }
        }
        if result.is_empty() {
            anyhow::bail!("No Pronunciation section found")
        } else {
            Ok(result)
        }
    }

    fn test(&self) {
        let a = ["a", "b"];
        let b = a
            .iter()
            .map(|x| self.get_id(x).and_then(|id| self.get_pron_sec_ids(id)));
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_top_10000_english() {
        let converter = WiktionaryConverter::new();
        let file_contents = std::fs::read_to_string("./google-10000-english.txt").unwrap();
        let words = file_contents.lines().take(100).collect();
        let ipas = converter.get_ipa(&words);
        for (i, ipa) in ipas.iter().enumerate() {
            assert!(ipa.is_ok(), "couldnt convert \"{}\": {:?}", &words[i], ipa)
        }
    }
}
