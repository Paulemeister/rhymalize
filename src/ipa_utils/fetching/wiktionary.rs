use super::IpaConverter;
use anyhow::bail;
use anyhow::{anyhow, Context, Error};
use serde_json::from_str;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::format;
static API_URL: &str = "https://en.wiktionary.org/w/api.php";
use futures::{stream, StreamExt, TryFutureExt};
use std::sync::{Arc, RwLock};
pub struct WiktionaryConverter {
    client: reqwest::Client,
    cache: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl IpaConverter for WiktionaryConverter {
    fn convert_single(&self, input: &str) -> Result<Vec<String>, anyhow::Error> {
        async_std::task::block_on(self.get_single(input))
    }
    fn convert(&self, inputs: &[&str]) -> Vec<Result<Vec<String>, anyhow::Error>> {
        let res = async_std::task::block_on(
            stream::iter(inputs)
                .map(|z| async { self.get_single(z).await })
                .buffered(200)
                .collect::<Vec<_>>(),
        );
        let _ = self.save_cache();

        res
    }
}
impl Default for WiktionaryConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl WiktionaryConverter {
    pub fn new() -> Self {
        let _ = std::fs::create_dir("./.wiktionary_cache");
        let mut new = Self {
            client: reqwest::Client::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        };
        let _ = new.load_cache();
        new
    }
    fn load_cache(&mut self) -> Result<(), anyhow::Error> {
        let contents = std::fs::read_to_string("./.wiktionary_cache/wikitonary_cache.json")?;
        let json: Value = serde_json::from_str(&contents)?;
        {
            let mut cache = self
                .cache
                .write()
                .map_err(|_| anyhow!("can't aquire lock"))?;
            for (key, ipas) in json.as_object().context("cache malformed")? {
                let values = ipas
                    .as_array()
                    .context("cache malformed")?
                    .iter()
                    .map(|x| x.as_str().unwrap().to_string())
                    .collect();
                cache.insert(key.clone(), values);
            }
        }
        Ok(())
    }
    fn save_cache(&self) -> Result<(), anyhow::Error> {
        let cache = self
            .cache
            .write()
            .map_err(|_| anyhow!("can't aquire lock"))?;
        std::fs::write(
            "./.wiktionary_cache/wikitonary_cache.json",
            serde_json::to_string(&*cache)?,
        )?;
        Ok(())
    }
    async fn extract_ipa_text_helper(
        &self,
        sec_id: i64,
        page_id: i64,
    ) -> Result<Vec<String>, Error> {
        let section = self.get_section(sec_id, page_id).await?;
        self.extract_ipa_from_pron(&section).await
    }
    async fn extract_ipa_other_helper(
        &self,
        sec_id: i64,
        page_id: i64,
    ) -> Result<Vec<String>, Error> {
        let section = self.get_section(sec_id, page_id).await?;
        self.extract_ipa_from_other(&section).await
    }

    async fn get_section(&self, sec_id: i64, page_id: i64) -> Result<String, anyhow::Error> {
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
        .context(format!(
            "making value from request for page {}, id {} failed",
            page_id, sec_id
        ))?;

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

        Ok(pron_sec.to_string())
    }
    async fn extract_ipa_from_pron(&self, text: &str) -> Result<Vec<String>, anyhow::Error> {
        let prons: Vec<String> = text
            .to_string()
            .split("{{")
            .filter(|x| x.starts_with("IPA|en|") || x.starts_with("IPA-lite|en|"))
            .map(|z| z.split_once("}}"))
            .collect::<Option<Vec<_>>>()
            .with_context(|| "didn't find ending parenthesis")?
            .iter()
            .flat_map(|(a, _)| a.split('|'))
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
    async fn extract_ipa_from_other(&self, text: &str) -> Result<Vec<String>, anyhow::Error> {
        let prefixes = ["infl of", "plural of", "pronunciation spelling of"];
        let lang = "en";
        let pattern: Vec<_> = prefixes.iter().map(|z| format!("{z}|{lang}|")).collect();

        let prons: Vec<&str> = text
            .split("{{")
            .filter(|x| pattern.iter().any(|prefix| x.starts_with(prefix)))
            .map(|z| z.split_once("}}"))
            .collect::<Option<Vec<_>>>()
            .with_context(|| "didn't find ending parenthesis")?
            .iter()
            .flat_map(|(a, _)| a.split('|'))
            .collect();
        let current_prefix = prons.first().context("malformed template")?;
        let extra_ipa = match *current_prefix {
            "infl of" => match *prons
                .last()
                .context("couldnt 'infl of' template malformed")?
            {
                "ed-form" => "d",
                "ing-form" => "ɪŋ",
                _ => "", //e => bail!("don't know replacement ipa for {}", e),
            },
            "plural of" => "s",
            _ => "",
        };
        let basis = prons.get(2).context("malformed template")?;
        let mut new: Vec<String> = self.convert_single(basis)?;
        new.iter_mut().for_each(|z| {
            z.insert_str(z.len() - 1, extra_ipa);
        });
        Ok(new)
    }

    async fn get_single(&self, word: &str) -> Result<Vec<String>, Error> {
        if let Ok(cache) = self.cache.read() {
            if let Some(vals) = cache.get(word) {
                return Ok(vals.clone());
            }
        }

        // id of first hit
        let page_id = self
            .get_id(word)
            .await
            .context("getting first_hit_id failed")?;

        let pron_sec_ids = self
            .get_sec_ids("Pronunciation", page_id)
            .await
            .context("getting pron_sec_id failed")?;

        let mut last_err = anyhow!("didn't find ipa section");

        for pron_sec_id in pron_sec_ids {
            let res = self.extract_ipa_text_helper(pron_sec_id, page_id).await;
            match res {
                Ok(ipas) => {
                    ipas.iter().for_each(|x| println!("{}", x));
                    self.cache
                        .write()
                        .map_err(|_| anyhow!("can't aquire lock"))?
                        .insert(word.to_string(), ipas.clone());
                    return Ok(ipas);
                }
                Err(e) => last_err = e,
            }
        }
        for heading in ["Verb", "Noun"] {
            let verb_sec_ids = match self
                .get_sec_ids(heading, page_id)
                .await
                .context("getting pron_sec_id failed")
            {
                Ok(a) => a,
                Err(e) => {
                    last_err = e;
                    continue;
                }
            };

            for other_sec_id in verb_sec_ids {
                let res = self.extract_ipa_other_helper(other_sec_id, page_id).await;
                match res {
                    Ok(ipas) => {
                        ipas.iter().for_each(|x| println!("{}", x));
                        self.cache
                            .write()
                            .map_err(|_| anyhow!("can't aquire lock"))?
                            .insert(word.to_string(), ipas.clone());
                        return Ok(ipas);
                    }
                    Err(e) => last_err = e,
                }
            }
        }
        Err(last_err)
    }
    #[allow(dead_code)]
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

    async fn get_sec_ids(&self, sec_name: &str, id: i64) -> Result<Vec<i64>, Error> {
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

            if title == sec_name {
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
        let words: Vec<_> = file_contents.lines().take(100).collect();
        let ipas = converter.get_ipa(&words);
        for (i, ipa) in ipas.iter().enumerate() {
            assert!(ipa.is_ok(), "couldnt convert \"{}\": {:?}", &words[i], ipa)
        }
    }
}
