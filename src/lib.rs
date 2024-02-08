pub mod ipa_utils;

#[cfg(test)]
mod tests {
    use super::ipa_utils::ipa::*;
    #[test]
    fn test_ipa_display() {
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
}
