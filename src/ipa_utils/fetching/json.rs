use crate::ipa_utils::fetching::*;
use anyhow::{Context, Error};
use serde_json::{json, Value};
use std::io::{read_to_string, BufReader};
use std::{fs::File, io::Read, path::Path};
pub struct JsonLookupConverter {
    pub lookup_content: Value,
}
impl JsonLookupConverter {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let reader = BufReader::new(File::open(path)?);
        Ok(Self {
            lookup_content: serde_json::from_reader(reader)?,
        })
    }
}

fn a() {
    let a = JsonLookupConverter {
        lookup_content: json!(""),
    };
    a.convert_single("a");
}

impl IpaConverter for JsonLookupConverter {
    fn convert_single(&self, input: &str) -> Result<Vec<String>, Error> {
        Ok(self
            .lookup_content
            .get(input)
            .context(format!("couldn't find \"{}\" in lookup json", input))?
            .as_str()
            .context(format!("json value for \"{}\" wasn't as string", input))?
            .split(", ")
            .map(|x| x.to_string())
            .collect())
    }

    fn convert(&self, inputs: Vec<&str>) -> Vec<Result<Vec<String>, Error>> {
        inputs
            .iter()
            .map(|input| Self::convert_single(self, input))
            .collect()
    }
}
