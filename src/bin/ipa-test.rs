use rhymalize::ipa_utils::ipa::*;

fn main() {
    let ipa_stings = ["\u{0065}\u{031E}", "\u{0061}\u{0308}", "\u{0251}"];
    let ipa_chars: Vec<Letter> = ipa_stings
        .iter()
        .map(|&x| Letter::try_from(x).unwrap())
        .collect();
    let reconverted: Vec<String> = ipa_chars.iter().map(|x| x.to_string()).collect();

    for (i, str) in ipa_stings.iter().enumerate() {
        println!("{} {}", str, reconverted[i]);
    }

    println!(
        "{}",
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Linguolabial,
            voicing: ConsonantVoicing::Voiceless
        }
    );
}
