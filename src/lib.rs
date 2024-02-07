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
}
