use rhymalize::ipa_utils::{
    fetching::{json::JsonLookupConverter, IpaConverter},
    ipa::*,
};
use std::path::Path;

fn main() {
    // let ipa_stings = ["\u{0065}\u{031E}", "\u{0061}\u{0308}", "\u{0251}"];
    // let ipa_chars: Vec<Letter> = ipa_stings
    //     .iter()
    //     .map(|&x| Letter::try_from(x).unwrap())
    //     .collect();
    // let reconverted: Vec<String> = ipa_chars.iter().map(|x| x.to_string()).collect();

    // for (i, str) in ipa_stings.iter().enumerate() {
    //     println!("{} {}", str, reconverted[i]);
    // }

    // println!(
    //     "{}",
    //     PulmonicConsonant {
    //         manner: PulmonicConsonantManner::NonSibilantFricative,
    //         place: ConsonantPlace::Linguolabial,
    //         voicing: ConsonantVoicing::Voiceless
    //     }
    // );

    // let b = Letter::try_from("ɑ̃").unwrap();
    // println!("{b} {b:?}");
    // let words = ["ɡ̊uːt", "ˈʃtuːdi̯ʊm", "ɔʁt", "ˈkeɪ.ɒs"];

    // let a = Word::try_from(words[1]).unwrap();
    // let opts = ipa_utils::ipa::english::EnglishSyllableRule {};
    // let b = syls_from_word(&a, &opts);
    //println!("{a:?}");
    //println!("{b:?}");
    // for i in b {
    //     println!("{}", i);
    // }

    let _converter = JsonLookupConverter::new(Path::new("./en_US.json")).unwrap();
    let converter = rhymalize::ipa_utils::fetching::wiktionary::WiktionaryConverter::new();
    let output = converter.convert(&["can't", "abkhazian"]);

    for i in output {
        if let Ok(k) = i {
            for j in k {
                match Word::try_from(j.as_str()) {
                    Ok(t) => println!("{t}"),
                    Err(e) => println!("{j}: {e}"),
                }
            }
        } else {
            println!("Err: {}", i.unwrap_err())
        }
    }
}
