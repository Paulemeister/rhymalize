use super::ipa::Word;
use anyhow::anyhow;
pub mod json;
pub mod wiktionary;

pub trait IpaConverter {
    fn convert(&self, inputs: Vec<&str>) -> Vec<Result<Vec<&str>, anyhow::Error>>;
    fn convert_single(&self, input: &str) -> Result<Vec<&str>, anyhow::Error>;
    fn get_ipa(&self, inputs: Vec<&str>) -> Vec<Result<Vec<Word>, anyhow::Error>> {
        inputs.iter().map(|x| self.get_ipa_single(x)).collect()
    }
    fn get_ipa_single(&self, input: &str) -> Result<Vec<Word>, anyhow::Error> {
        self.convert_single(input)?
            .iter()
            .map(|&x| Word::try_from(x))
            .collect::<Result<Vec<Word>, _>>()
            .map_err(|x| anyhow!("test"))
    }
}
