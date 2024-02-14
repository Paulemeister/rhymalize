use crate::ipa_utils::ipa::*;

pub struct EnglishSyllableRule;

impl SyllableRule for EnglishSyllableRule {
    fn is_allowed_neighbour(&self, first: &Letter, second: &Letter) -> bool {
        !(first
            == &Letter {
                ipa_type: LetterType::Suprasegmental(Suprasegmental::Long),
                diacritics: None,
            })
    }
    fn is_diphthong(&self, first: &Letter, second: &Letter) -> bool {
        matches!(
            (first, second),
            (
                Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::CloseMid,
                        backness: VowelBackness::Front,
                        roundedness: VowelRoundedness::Unrounded,
                    }),
                    diacritics: None,
                } | Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::Open,
                        backness: VowelBackness::Front,
                        roundedness: VowelRoundedness::Unrounded,
                    }),
                    diacritics: None,
                } | Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::OpenMid,
                        backness: VowelBackness::Back,
                        roundedness: VowelRoundedness::Rounded,
                    }),
                    diacritics: None,
                },
                Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::NearClose,
                        backness: VowelBackness::Front,
                        roundedness: VowelRoundedness::Unrounded,
                    }),
                    diacritics: None,
                },
            ) | (
                Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::Mid,
                        backness: VowelBackness::Central,
                        roundedness: VowelRoundedness::Unrounded,
                    }),
                    diacritics: None,
                } | Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::CloseMid,
                        backness: VowelBackness::Back,
                        roundedness: VowelRoundedness::Rounded,
                    }),
                    diacritics: None,
                } | Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::Open,
                        backness: VowelBackness::Front,
                        roundedness: VowelRoundedness::Unrounded,
                    }),
                    diacritics: None,
                },
                Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::NearClose,
                        backness: VowelBackness::Back,
                        roundedness: VowelRoundedness::Rounded,
                    }),
                    diacritics: None,
                },
            )
        )
    }
}
