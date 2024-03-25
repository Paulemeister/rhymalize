use crate::ipa_utils::ipa::*;

pub struct EnglishSyllableRule;

// see https://en.wikipedia.org/wiki/English_phonology

impl SyllableRule for EnglishSyllableRule {
    fn makes_valid_onset(&self, new: &Letter, rest: &Vec<Letter>) -> bool {
        true
        // !matches!((rest.as_slice(), new), ())
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
            ) | (
                // own, not included in wikipedia
                Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::OpenMid,
                        backness: VowelBackness::Front,
                        roundedness: VowelRoundedness::Unrounded,
                    }),
                    diacritics: None,
                },
                Letter {
                    ipa_type: LetterType::Vowel(Vowel {
                        height: VowelHeight::Mid,
                        backness: VowelBackness::Central,
                        roundedness: VowelRoundedness::Unrounded,
                    }),
                    diacritics: None,
                }
            )
        )
    }
}
