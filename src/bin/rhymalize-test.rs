use std::{path::Path, vec};

use rhymalize::ipa_utils::fetching::{
    json::JsonLookupConverter, wiktionary::WiktionaryConverter, IpaConverter,
};

fn main() {
    let _converter = JsonLookupConverter::new(Path::new("./en_US.json")).unwrap();
    let converter = WiktionaryConverter::new();
    // what to search

    // let word = "example";

    // let ipa_s = converter.convert_single(word);

    // match ipa_s {
    //     Ok(a) => println!("{:?}", a),
    //     Err(e) => println!("{}", e),
    // }

    //let word_list = vec!["can't", "example", "what", "a", "in", "are", "home"];
    let word_list = vec!["PM"];

    let ipa_s_list = converter.get_ipa(&word_list);

    ipa_s_list.iter().for_each(|x| {
        match x {
            Ok(a) => a.iter().for_each(|x| println!("{}", x)),
            Err(e) => println!("{}", e),
        }
        println!();
    });
}
