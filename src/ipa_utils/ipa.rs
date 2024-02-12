#![allow(dead_code)]
use core::fmt;
use phf::{phf_map, Map};

const DIACRITIC_MAP: Map<char, Diacritic> = phf_map! {
    '\u{0329}'=>Diacritic::Syllabic,
    '\u{030D}'=>Diacritic::Syllabic,
    '\u{032F}'=>Diacritic::NonSyllabic,
    '\u{0311}'=>Diacritic::NonSyllabic,
    '\u{02B0}'=>Diacritic::Aspirated,
    '\u{031A}'=>Diacritic::NoAudibleRelease,
    '\u{207F}'=>Diacritic::NasalRelease,
    '\u{02E1}'=>Diacritic::LateralRelease,
    '\u{1DBF}'=>Diacritic::VoicelessDentalFricativeRelease,
    '\u{02E3}'=>Diacritic::VoicelessVelarFricativeRelease,
    '\u{1D4A}'=>Diacritic::MidCentralVowelRelease,
    '\u{0325}'=>Diacritic::Voiceless,
    '\u{030A}'=>Diacritic::Voiceless,
    '\u{032C}'=>Diacritic::Voiced,
    '\u{0324}'=>Diacritic::BreathyVoiced,
    '\u{0330}'=>Diacritic::CreakyVoiced,
    '\u{032A}'=>Diacritic::Dental,
    '\u{0346}'=>Diacritic::Dental,
    '\u{033C}'=>Diacritic::Linguolabial,
    '\u{033A}'=>Diacritic::Apical,
    '\u{033B}'=>Diacritic::Laminal,
    '\u{031F}'=>Diacritic::Advanced,// has other
    '\u{0320}'=>Diacritic::Retracted,
    '\u{0304}'=>Diacritic::Retracted,
    '\u{0308}'=>Diacritic::Centralized,
    '\u{033D}'=>Diacritic::MidCentralized,
    '\u{031D}'=>Diacritic::NonSyllabic,
    '\u{02D4}'=>Diacritic::Raised,
    '\u{031E}'=>Diacritic::Lowered,
    '\u{02D5}'=>Diacritic::Lowered,
    '\u{0339}'=>Diacritic::MoreRounded,
    '\u{0357}'=>Diacritic::MoreRounded,
    '\u{031C}'=>Diacritic::LessRounded,
    '\u{0351}'=>Diacritic::LessRounded,
    '\u{02B7}'=>Diacritic::Labialized,
    '\u{02B2}'=>Diacritic::Palatalized,
    '\u{02E0}'=>Diacritic::Velarized,
    '\u{0334}'=>Diacritic::VelarizedOrPharyngealized,
    '\u{02E4}'=>Diacritic::Pharyngealized,
    '\u{0318}'=>Diacritic::AdvancedTongueRoot,
    '\u{AB6A}'=>Diacritic::AdvancedTongueRoot,
    '\u{0319}'=>Diacritic::RetractedTongueRoot,
    '\u{AB6B}'=>Diacritic::RetractedTongueRoot,
    '\u{0303}'=>Diacritic::Nasalized,
    '\u{02DE}'=>Diacritic::Rhoticity,
};

const SUPRASEGREMENTAL_MAP: Map<char, Suprasegmental> = phf_map! {
    '\u{02C8}' => Suprasegmental::PrimaryStress,
    '\u{02CC}' => Suprasegmental::SecondaryStress,
    '\u{02D0}' => Suprasegmental::Long,
    '\u{02D1}' => Suprasegmental::HalfLong,
    '\u{0306}' => Suprasegmental::ExtraShort,
    '\u{002E}' => Suprasegmental::SyllableBreak,
    '\u{203F}' => Suprasegmental::Linking,
    '\u{007C}' => Suprasegmental::MinorBreak,
    '\u{2016}' => Suprasegmental::MajorBreak,
    '\u{2197}' => Suprasegmental::GlobalRise,
    '\u{2198}' => Suprasegmental::GlobalFall,

};

static CONSONANT_LIST: [(PulmonicConsonant, &[char]); 71] = [
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Postalveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{030A}', '\u{0279}', '\u{0331}', '\u{02D4}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{027B}', '\u{030A}', '\u{02D4}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Postalveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0279}', '\u{0331}', '\u{02D4}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{006D}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Labiodental,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{006D}', '\u{006D}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Linguolabial,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{006E}', '\u{033C}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{006E}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0273}', '\u{030A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0272}', '\u{030A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{014B}', '\u{030A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0274}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Labiodental,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0062}', '\u{032A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Labiodental,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0070}', '\u{032A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Linguolabial,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0064}', '\u{033C}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Linguolabial,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0074}', '\u{033C}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Linguolabial,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{00F0}', '\u{033C}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Linguolabial,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{033C}', '\u{03B8}'], // attention to output order
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{00F0}', '\u{0331}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0331}', '\u{03B8}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{027B}', '\u{02D4}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Approximant,
            place: ConsonantPlace::Glottal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0294}', '\u{0330}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{006D}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Labiodental,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0271}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{006E}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0273}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0272}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{014B}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Nasal,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0274}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0062}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0070}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0064}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0074}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0256}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0288}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{025F}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0063}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0261}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{006B}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0262}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0071}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Pharyngeal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{02A1}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Plosive,
            place: ConsonantPlace::Glottal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0294}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::SibilantFricative,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{007A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::SibilantFricative,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0073}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::SibilantFricative,
            place: ConsonantPlace::Postalveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0292}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::SibilantFricative,
            place: ConsonantPlace::Postalveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0283}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::SibilantFricative,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0290}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::SibilantFricative,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0282}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::SibilantFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0291}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::SibilantFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0255}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{03B2}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0278}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Labiodental,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0076}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Labiodental,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0066}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Dental,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{00F0}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Dental,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{03B8}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{029D}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{00E7}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0263}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0078}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0281}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{03C7}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Pharyngeal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0295}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Pharyngeal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0127}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Glottal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0266}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::NonSibilantFricative,
            place: ConsonantPlace::Glottal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0068}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Approximant,
            place: ConsonantPlace::Labiodental,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{028B}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Approximant,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0279}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Approximant,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{027B}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Approximant,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{006A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Approximant,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0270}'],
    ),
];

pub struct Letter {
    ipa_type: LetterType,
    diacritics: Option<Vec<Diacritic>>,
}

impl TryFrom<&str> for Letter {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let diacritics = get_diacritics(value);
        let ipa_type = LetterType::try_from(value)?;
        Ok(Self {
            ipa_type,
            diacritics: if diacritics.is_empty() {
                None
            } else {
                Some(diacritics)
            },
        })
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ipa_type)
    }
}

pub enum LetterType {
    PulmonicConsonant(PulmonicConsonant),
    NonPulmonicConsonant,
    Vowel(Vowel),
    Suprasegmental(Suprasegmental),
}

impl TryFrom<&str> for LetterType {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(vow) = Vowel::try_from(value) {
            return Ok(Self::Vowel(vow));
        }
        if let Ok(sup) = Suprasegmental::try_from(value) {
            return Ok(Self::Suprasegmental(sup));
        }
        Err(())
    }
}

impl fmt::Display for LetterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LetterType::PulmonicConsonant(p) => p.to_string(),
                LetterType::Vowel(v) => v.to_string(),
                _ => return Err(fmt::Error),
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Diacritic {
    Syllabic,
    NonSyllabic,
    Aspirated,
    NoAudibleRelease,
    NasalRelease,
    LateralRelease,
    VoicelessDentalFricativeRelease,
    VoicelessVelarFricativeRelease,
    MidCentralVowelRelease,
    Voiceless,
    Voiced,
    BreathyVoiced,
    CreakyVoiced,
    Dental,
    Linguolabial,
    Apical,
    Laminal,
    Advanced,
    Retracted,
    Centralized,
    MidCentralized,
    Raised,
    Lowered,
    MoreRounded,
    LessRounded,
    Labialized,
    Palatalized,
    Velarized,
    VelarizedOrPharyngealized,
    Pharyngealized,
    AdvancedTongueRoot,
    RetractedTongueRoot,
    Nasalized,
    Rhoticity,
}

fn get_diacritics(value: &str) -> Vec<Diacritic> {
    let mut diacs = vec![];
    for (entry, dia) in DIACRITIC_MAP.entries() {
        if value.contains(*entry) {
            diacs.push(*dia);
        }
    }
    diacs
}

impl fmt::Display for Diacritic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((text, _)) = DIACRITIC_MAP.entries().find(|(_, dia)| *dia == self) {
            write!(f, "{}", text)
        } else {
            Err(fmt::Error)
        }
    }
}
#[derive(PartialEq, Clone)]
pub enum Suprasegmental {
    PrimaryStress,
    SecondaryStress,
    Long,
    HalfLong,
    ExtraShort,
    SyllableBreak,
    Linking,
    MinorBreak,
    MajorBreak,
    GlobalRise,
    GlobalFall,
    Upstep,
    Downstep,
    PitchDiacritic(PitchDiacritic),
    ChaoToneLetter(ChaoToneLetter),
}

impl fmt::Display for Suprasegmental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((text, _)) = SUPRASEGREMENTAL_MAP.entries().find(|(_, sup)| *sup == self) {
            write!(f, "{}", text)
        } else {
            Err(fmt::Error)
        }
    }
}

impl TryFrom<&str> for Suprasegmental {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for i in value.chars() {
            if let Some(a) = SUPRASEGREMENTAL_MAP.get(&i) {
                return Ok(a.clone());
            }
        }
        Err(())
    }
}

#[derive(PartialEq, Clone)]
pub struct ChaoToneLetter {
    contour: Vec<ChaoToneLetterHeight>,
    reversed: bool,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ChaoToneLetterHeight {
    ExtraHigh,
    High,
    Mid,
    Low,
    ExtraLow,
}

#[derive(PartialEq, Clone, Copy)]
pub enum PitchDiacritic {
    ExtraHigh,
    High,
    Mid,
    Low,
    ExtraLow,
    Rising,
    Falling,
    Peaking,
    Dipping,
    MidRising,
    LowRising,
    HighFalling,
    MidFalling,
}

#[derive(PartialEq, Debug)]
pub struct Vowel {
    pub height: VowelHeight,
    pub backness: VowelBackness,
    pub roundedness: VowelRoundedness,
}

impl std::fmt::Display for Vowel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Vowel {
            height: h,
            backness: b,
            roundedness: r,
        } = self;
        let str = match h {
            VowelHeight::Close => match b {
                VowelBackness::Back => match r {
                    VowelRoundedness::Rounded => "\u{0075}",
                    VowelRoundedness::Unrounded => "\u{026F}",
                },
                VowelBackness::Central => match r {
                    VowelRoundedness::Rounded => "\u{0289}",
                    VowelRoundedness::Unrounded => "\u{0268}",
                },
                VowelBackness::Front => match r {
                    VowelRoundedness::Rounded => "\u{0079}",
                    VowelRoundedness::Unrounded => "\u{0069}",
                },
            },
            VowelHeight::NearClose => match b {
                VowelBackness::Back => match r {
                    VowelRoundedness::Rounded => "\u{028A}",
                    VowelRoundedness::Unrounded => return Err(std::fmt::Error),
                },
                VowelBackness::Central => return Err(std::fmt::Error),
                VowelBackness::Front => match r {
                    VowelRoundedness::Rounded => "\u{028F}",
                    VowelRoundedness::Unrounded => "\u{026A}",
                },
            },
            VowelHeight::CloseMid => match b {
                VowelBackness::Back => match r {
                    VowelRoundedness::Rounded => "\u{006F}",
                    VowelRoundedness::Unrounded => "\u{0264}",
                },
                VowelBackness::Central => match r {
                    VowelRoundedness::Rounded => "\u{0275}",
                    VowelRoundedness::Unrounded => "\u{0258}",
                },
                VowelBackness::Front => match r {
                    VowelRoundedness::Rounded => "\u{00F8}",
                    VowelRoundedness::Unrounded => "\u{0065}",
                },
            },
            VowelHeight::Mid => match b {
                VowelBackness::Back => match r {
                    VowelRoundedness::Rounded => "\u{006F}\u{031E}",
                    VowelRoundedness::Unrounded => "\u{0264}\u{031E}",
                },
                VowelBackness::Central => "\u{0259}",
                VowelBackness::Front => match r {
                    VowelRoundedness::Rounded => "\u{00F8}\u{031E}",
                    VowelRoundedness::Unrounded => "\u{0065}\u{031E}",
                },
            },
            VowelHeight::OpenMid => match b {
                VowelBackness::Back => match r {
                    VowelRoundedness::Rounded => "\u{0254}",
                    VowelRoundedness::Unrounded => "\u{028C}",
                },
                VowelBackness::Central => match r {
                    VowelRoundedness::Rounded => "\u{025E}",
                    VowelRoundedness::Unrounded => "\u{025C}",
                },
                VowelBackness::Front => match r {
                    VowelRoundedness::Rounded => "\u{0153}",
                    VowelRoundedness::Unrounded => "\u{025B}",
                },
            },
            VowelHeight::NearOpen => match b {
                VowelBackness::Back => return Err(std::fmt::Error),
                VowelBackness::Central => "\u{0250}",
                VowelBackness::Front => "\u{00E6}",
            },
            VowelHeight::Open => match b {
                VowelBackness::Back => match r {
                    VowelRoundedness::Rounded => "\u{0252}",
                    VowelRoundedness::Unrounded => "\u{0251}",
                },
                VowelBackness::Central => match r {
                    VowelRoundedness::Rounded => return Err(std::fmt::Error),
                    VowelRoundedness::Unrounded => "\u{0061}\u{0308}",
                },
                VowelBackness::Front => match r {
                    VowelRoundedness::Rounded => "\u{0276}",
                    VowelRoundedness::Unrounded => "\u{0061}",
                },
            },
        };
        write!(f, "{}", str)
    }
}

impl TryFrom<&str> for Vowel {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(
            //// multiple conditions first
            if value.contains('\u{0061}') && value.contains('\u{0308}') {
                // account for ä as one char?
                Vowel {
                    height: VowelHeight::Open,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{006F}') && value.contains('\u{031E}') {
                Vowel {
                    height: VowelHeight::Mid,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0264}') && value.contains('\u{031E}') {
                Vowel {
                    height: VowelHeight::Mid,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{00F8}') && value.contains('\u{031E}') {
                Vowel {
                    height: VowelHeight::Mid,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0065}') && value.contains('\u{031E}') {
                Vowel {
                    height: VowelHeight::Mid,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Unrounded,
                }
            }
            /////////////
            else if value.contains('\u{0075}') {
                Vowel {
                    height: VowelHeight::Close,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{026F}') {
                Vowel {
                    height: VowelHeight::Close,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{0289}') {
                Vowel {
                    height: VowelHeight::Close,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0268}') {
                Vowel {
                    height: VowelHeight::Close,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{0079}') {
                Vowel {
                    height: VowelHeight::Close,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0069}') {
                Vowel {
                    height: VowelHeight::Close,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Unrounded,
                }
            }
            /////////////////////////////
            else if value.contains('\u{028A}') {
                Vowel {
                    height: VowelHeight::NearClose,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{028F}') {
                Vowel {
                    height: VowelHeight::NearClose,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{026A}') {
                Vowel {
                    height: VowelHeight::NearClose,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Unrounded,
                }
            }
            ///////////////
            else if value.contains('\u{006F}') {
                Vowel {
                    height: VowelHeight::CloseMid,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0264}') {
                Vowel {
                    height: VowelHeight::CloseMid,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{0275}') {
                Vowel {
                    height: VowelHeight::CloseMid,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0258}') {
                Vowel {
                    height: VowelHeight::CloseMid,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{00F8}') {
                Vowel {
                    height: VowelHeight::CloseMid,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0065}') {
                Vowel {
                    height: VowelHeight::CloseMid,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Unrounded,
                }
            }
            ////////////////
            else if value.contains('\u{0259}') {
                Vowel {
                    height: VowelHeight::Mid,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Rounded,
                }
            }
            ///////////
            else if value.contains('\u{0254}') {
                Vowel {
                    height: VowelHeight::OpenMid,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{028C}') {
                Vowel {
                    height: VowelHeight::OpenMid,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{025E}') {
                Vowel {
                    height: VowelHeight::OpenMid,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{025C}') {
                Vowel {
                    height: VowelHeight::OpenMid,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{0153}') {
                Vowel {
                    height: VowelHeight::OpenMid,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{025B}') {
                Vowel {
                    height: VowelHeight::OpenMid,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Unrounded,
                }
            }
            ///////////
            else if value.contains('\u{0252}') {
                Vowel {
                    ////////one possiblility
                    height: VowelHeight::NearOpen,
                    backness: VowelBackness::Central,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{00E6}') {
                Vowel {
                    ////////one possiblility
                    height: VowelHeight::NearOpen,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Unrounded,
                }
            }
            ///////////
            else if value.contains('\u{0252}') {
                Vowel {
                    height: VowelHeight::Open,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0251}') {
                Vowel {
                    height: VowelHeight::Open,
                    backness: VowelBackness::Back,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else if value.contains('\u{0276}') {
                Vowel {
                    height: VowelHeight::Open,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Rounded,
                }
            } else if value.contains('\u{0061}') {
                Vowel {
                    height: VowelHeight::Open,
                    backness: VowelBackness::Front,
                    roundedness: VowelRoundedness::Unrounded,
                }
            } else {
                return Err(());
            },
        )
    }
}
#[derive(PartialEq, Debug)]
pub enum VowelHeight {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

#[derive(PartialEq, Debug)]
pub enum VowelBackness {
    Front,
    Central,
    Back,
}
#[derive(PartialEq, Debug)]
pub enum VowelRoundedness {
    Unrounded,
    Rounded,
}

#[derive(PartialEq, Clone, Debug)]
pub struct PulmonicConsonant {
    pub manner: PulmonicConsonantManner,
    pub place: ConsonantPlace,
    pub voicing: ConsonantVoicing,
    // exception, as sometimes voiceless consonant has different symbol
    // otherwise it should be a diacritic
}

fn test(s: &PulmonicConsonant) -> Result<String, ()> {
    let a: Option<String> = CONSONANT_LIST
        .iter()
        .find(|(consonant, _)| s == consonant)
        .map(|(_, chars)| chars.iter().collect());
    a.ok_or(())
}

fn test2(text: &str) -> Result<PulmonicConsonant, ()> {
    for (i, j) in CONSONANT_LIST.iter() {
        if j.iter().all(|x| text.contains(*x)) {
            return Ok(i.clone());
        }
    }
    Err(())
}

impl std::fmt::Display for PulmonicConsonant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(a) = CONSONANT_LIST
            .iter()
            .find(|(consonant, _)| self == consonant)
            .map(|(_, chars)| chars.iter().collect::<String>())
        {
            write!(f, "{}", a)
        } else {
            Err(fmt::Error)
        }
    }
}
impl TryFrom<&str> for PulmonicConsonant {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for (i, j) in CONSONANT_LIST.iter() {
            if j.iter().all(|x| value.contains(*x)) {
                return Ok(i.clone());
            }
        }
        Err(())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum PulmonicConsonantManner {
    Nasal,
    Plosive,
    SibilantFricative,
    NonSibilantFricative,
    Approximant,
    Tap,
    Trill,
    LateralFricative,
    LateralApproximant,
    LateralTap,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ConsonantPlace {
    Bilabial,
    Labiodental,
    Linguolabial, // Only Pulmonic
    Dental,
    Aveolar,
    Postalveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal, // Not in Non-pulmonic
}

#[derive(PartialEq, Clone, Debug)]
pub enum ConsonantVoicing {
    Voiced,
    Voiceless,
}
