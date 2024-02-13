pub mod ipa_utils;

#[cfg(test)]
mod tests {
    use super::ipa_utils::ipa::*;
    #[test]
    fn vowel_display() {
        let one = Vowel {
            height: VowelHeight::Close,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Unrounded,
        };
        assert_eq!(one.to_string(), String::from("\u{0069}"));
    }
    #[test]
    fn vowel_from_str() {
        let str = "\u{0065}\u{031E}";
        let vowel = Vowel::try_from(str).unwrap();
        assert_eq!(
            vowel,
            Vowel {
                height: VowelHeight::Mid,
                backness: VowelBackness::Front,
                roundedness: VowelRoundedness::Unrounded
            }
        )
    }
    #[test]
    fn pul_cons_display() {
        let one = PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Postalveolar,
            voicing: ConsonantVoicing::Voiceless,
        };
        assert_eq!(
            one.to_string(),
            String::from("\u{030A}\u{0279}\u{0331}\u{02D4}")
        );
    }
    #[test]
    fn pul_cons_from_str() {
        let str = "\u{030A}\u{0279}\u{0331}\u{02D4}";
        let cons = PulmonicConsonant::try_from(str).unwrap();
        assert_eq!(
            cons,
            PulmonicConsonant {
                manner: PulmonicConsonantManner::NonSibilantFricative,
                place: ConsonantPlace::Postalveolar,
                voicing: ConsonantVoicing::Voiceless,
            }
        )
    }
    #[test]
    fn letter_display() {
        let one = Letter {
            ipa_type: LetterType::Vowel(Vowel {
                height: VowelHeight::Open,
                backness: VowelBackness::Back,
                roundedness: VowelRoundedness::Unrounded,
            }),
            diacritics: Some(vec![Diacritic::Nasalized]),
        };
        let two = Letter {
            ipa_type: LetterType::PulmonicConsonant(PulmonicConsonant {
                manner: PulmonicConsonantManner::NonSibilantFricative,
                place: ConsonantPlace::Postalveolar,
                voicing: ConsonantVoicing::Voiceless,
            }),
            diacritics: None,
        };
        assert_eq!(one.to_string(), String::from("ɑ̃"));
        assert_eq!(
            two.to_string(),
            String::from("\u{030A}\u{0279}\u{0331}\u{02D4}")
        );
    }
}
