#![allow(dead_code)]
use anyhow::anyhow;
use core::fmt;
use phf::{phf_map, Map};
use std::vec;
use unicode_segmentation::UnicodeSegmentation;

pub mod english;

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
    '\u{031D}'=>Diacritic::Raised,
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

const PULMONIC_CONSONANT_LIST: [(PulmonicConsonant, &[char]); 111] = [
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
        &['\u{006D}'],
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
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{2C71}', '\u{031F}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Labiodental,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{2C71}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Linguolabial,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{027E}', '\u{033C}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{027E}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{027E}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{027D}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{027D}', '\u{030A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0262}', '\u{0306}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Tap,
            place: ConsonantPlace::Pharyngeal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{02A1}', '\u{0306}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0299}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Bilabial,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0299}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0072}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0072}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{027D}', '\u{0072}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{027D}', '\u{030A}', '\u{0072}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{0280}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{0280}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Pharyngeal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{02A2}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::Trill,
            place: ConsonantPlace::Pharyngeal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{029C}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{026E}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{026C}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{1DF05}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{A78E}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{028E}', '\u{031D}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{1DF06}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{029F}', '\u{031D}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{1DF04}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{029F}', '\u{031D}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralFricative,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{1DF04}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralApproximant,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{006C}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralApproximant,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{026D}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralApproximant,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{028E}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralApproximant,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{029F}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralApproximant,
            place: ConsonantPlace::Uvular,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{029F}', '\u{0320}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralTap,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{027A}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralTap,
            place: ConsonantPlace::Aveolar,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{027A}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralTap,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{1DF08}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralTap,
            place: ConsonantPlace::Retroflex,
            voicing: ConsonantVoicing::Voiceless,
        },
        &['\u{1DF08}', '\u{0325}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralTap,
            place: ConsonantPlace::Palatal,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{028E}', '\u{0306}'],
    ),
    (
        PulmonicConsonant {
            manner: PulmonicConsonantManner::LateralTap,
            place: ConsonantPlace::Velar,
            voicing: ConsonantVoicing::Voiced,
        },
        &['\u{029F}', '\u{0306}'],
    ),
];

const VOWEL_LIST: [(Vowel, &[char]); 33] = [
    (
        Vowel {
            height: VowelHeight::Mid,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{006F}', '\u{031E}'],
    ),
    (
        Vowel {
            height: VowelHeight::Mid,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0264}', '\u{031E}'],
    ),
    (
        Vowel {
            height: VowelHeight::Mid,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{00F8}', '\u{031E}'],
    ),
    (
        Vowel {
            height: VowelHeight::Mid,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0065}', '\u{031E}'],
    ),
    (
        Vowel {
            height: VowelHeight::Open,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0061}', '\u{0308}'],
    ),
    (
        Vowel {
            height: VowelHeight::Close,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{0075}'],
    ),
    (
        Vowel {
            height: VowelHeight::Close,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{026F}'],
    ),
    (
        Vowel {
            height: VowelHeight::Close,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{0289}'],
    ),
    (
        Vowel {
            height: VowelHeight::Close,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0268}'],
    ),
    (
        Vowel {
            height: VowelHeight::Close,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{0079}'],
    ),
    (
        Vowel {
            height: VowelHeight::Close,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0069}'],
    ),
    (
        Vowel {
            height: VowelHeight::NearClose,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{028A}'],
    ),
    (
        Vowel {
            height: VowelHeight::NearClose,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{028F}'],
    ),
    (
        Vowel {
            height: VowelHeight::NearClose,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{026A}'],
    ),
    (
        Vowel {
            height: VowelHeight::CloseMid,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{006F}'],
    ),
    (
        Vowel {
            height: VowelHeight::CloseMid,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{006F}'],
    ),
    (
        Vowel {
            height: VowelHeight::CloseMid,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{0275}'],
    ),
    (
        Vowel {
            height: VowelHeight::CloseMid,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0258}'],
    ),
    (
        Vowel {
            height: VowelHeight::CloseMid,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{00F8}'],
    ),
    (
        Vowel {
            height: VowelHeight::CloseMid,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0065}'],
    ),
    (
        Vowel {
            height: VowelHeight::OpenMid,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{0153}'],
    ),
    (
        Vowel {
            height: VowelHeight::OpenMid,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{025B}'],
    ),
    (
        Vowel {
            height: VowelHeight::OpenMid,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{025E}'],
    ),
    (
        Vowel {
            height: VowelHeight::OpenMid,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{025C}'],
    ),
    (
        Vowel {
            height: VowelHeight::OpenMid,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{0254}'],
    ),
    (
        Vowel {
            height: VowelHeight::OpenMid,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{028C}'],
    ),
    (
        // double check roundedness
        Vowel {
            height: VowelHeight::Mid,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0259}'],
    ),
    (
        Vowel {
            height: VowelHeight::NearOpen,
            backness: VowelBackness::Central,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0250}'],
    ),
    (
        Vowel {
            height: VowelHeight::NearOpen,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{00E6}'],
    ),
    (
        Vowel {
            height: VowelHeight::Open,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{0252}'],
    ),
    (
        Vowel {
            height: VowelHeight::Open,
            backness: VowelBackness::Back,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0251}'],
    ),
    (
        Vowel {
            height: VowelHeight::Open,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Rounded,
        },
        &['\u{0276}'],
    ),
    (
        Vowel {
            height: VowelHeight::Open,
            backness: VowelBackness::Front,
            roundedness: VowelRoundedness::Unrounded,
        },
        &['\u{0061}'],
    ),
];

const REPLACE_LIST: [(char, &str); 3] = [
    ('ɫ', "l\u{02E0}"),
    ('ɚ', "\u{0259}\u{02DE}"),
    ('ɝ', "\u{025C}\u{02DE}"),
];

pub trait SyllableRule {
    fn is_allowed_neighbour(&self, first: &Letter, second: &Letter) -> bool;
    fn is_diphthong(&self, first: &Letter, second: &Letter) -> bool;
}

#[derive(Debug, Clone)]
pub struct Syllable {
    pub onset: Vec<Letter>,
    pub nucleus: Vec<Letter>,
    pub coda: Vec<Letter>,
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let onset_str: String = self.onset.iter().map(|x| x.to_string()).collect();
        let nucleus_str: String = self.nucleus.iter().map(|x| x.to_string()).collect();
        let coda_str: String = self.coda.iter().map(|x| x.to_string()).collect();
        write!(f, "{onset_str}{nucleus_str}{coda_str}")
    }
}

impl From<Word> for Vec<Letter> {
    fn from(value: Word) -> Self {
        value.0
    }
}

pub fn syls_from_word(input: &Word, options: &dyn SyllableRule) -> Vec<Syllable> {
    let mut out = vec![];
    let mut onset = vec![];
    let mut nucleus = vec![];
    let mut coda = vec![];
    let mut last = input.0.last().unwrap();
    let mut found_nuc = match last {
        Letter {
            ipa_type: LetterType::Vowel(_),
            diacritics: _,
        } => {
            nucleus.push(last.clone());
            true
        }
        _ => {
            coda.push(last.clone());
            false
        }
    };
    for letter in input.0.iter().rev().skip(1) {
        match (letter, last) {
            (
                Letter {
                    ipa_type: LetterType::Vowel(_),
                    diacritics: _,
                },
                Letter {
                    ipa_type: LetterType::Vowel(_),
                    diacritics: _,
                },
            ) => {
                if !options.is_diphthong(letter, last) {
                    onset.reverse();
                    nucleus.reverse();
                    coda.reverse();
                    out.push(Syllable {
                        onset,
                        nucleus,
                        coda,
                    });
                    onset = vec![];
                    nucleus = vec![letter.clone()];
                    coda = vec![];
                    found_nuc = true
                } else {
                    nucleus.push(letter.clone());
                }
            }
            (
                Letter {
                    ipa_type: LetterType::PulmonicConsonant(_) | LetterType::Suprasegmental(_),
                    diacritics: _,
                },
                Letter {
                    ipa_type: LetterType::PulmonicConsonant(_) | LetterType::Suprasegmental(_),
                    diacritics: _,
                },
            ) => {
                if !options.is_allowed_neighbour(letter, last) {
                    onset.reverse();
                    nucleus.reverse();
                    coda.reverse();
                    out.push(Syllable {
                        onset,
                        nucleus,
                        coda,
                    });
                    onset = vec![];
                    nucleus = vec![];
                    coda = vec![letter.clone()];
                    found_nuc = false;
                } else if found_nuc {
                    onset.push(letter.clone());
                } else {
                    coda.push(letter.clone());
                }
            }
            (
                Letter {
                    ipa_type: LetterType::Vowel(_),
                    diacritics: _,
                },
                _,
            ) => {
                if found_nuc {
                    onset.reverse();
                    nucleus.reverse();
                    coda.reverse();
                    out.push(Syllable {
                        onset,
                        nucleus,
                        coda,
                    });
                    onset = vec![];
                    nucleus = vec![letter.clone()];
                    coda = vec![];
                    found_nuc = true;
                } else {
                    nucleus.push(letter.clone());
                    found_nuc = true;
                }
            }
            _ => {
                if found_nuc {
                    onset.push(letter.clone());
                } else {
                    coda.push(letter.clone());
                }
            }
        }
        last = letter;
    }
    if found_nuc {
        onset.reverse();
        nucleus.reverse();
        coda.reverse();
        out.push(Syllable {
            onset,
            nucleus,
            coda,
        });
    }

    out.reverse();
    out
}

#[derive(Debug, Default)]
pub struct Word(Vec<Letter>);

impl TryFrom<&str> for Word {
    // assumes diacritics are always behind their corresponding letters
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut out = vec![];

        for grapheme in UnicodeSegmentation::graphemes(value, true) {
            if grapheme == "/" {
                continue;
            }
            out.push(Letter::try_from(grapheme)?)
        }

        Ok(Self(out))
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word_str = self.0.iter().map(|z| z.to_string()).collect::<String>();

        write!(f, "{}", word_str)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Letter {
    pub ipa_type: LetterType,
    pub diacritics: Option<Vec<Diacritic>>,
}

impl TryFrom<&str> for Letter {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for (special, replace) in REPLACE_LIST {
            if value.contains(special) {
                return Self::try_from(replace);
            }
        }
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
        let dia_str = if let Some(a) = &self.diacritics {
            a.iter().map(|z| z.to_string()).collect::<String>()
        } else {
            "".to_string()
        };
        let a = self.ipa_type.to_string() + &dia_str;
        write!(f, "{}", a)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LetterType {
    PulmonicConsonant(PulmonicConsonant),
    NonPulmonicConsonant,
    Vowel(Vowel),
    Suprasegmental(Suprasegmental),
}

impl TryFrom<&str> for LetterType {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(vow) = Vowel::try_from(value) {
            return Ok(Self::Vowel(vow));
        }
        if let Ok(sup) = Suprasegmental::try_from(value) {
            return Ok(Self::Suprasegmental(sup));
        }
        if let Ok(cons) = PulmonicConsonant::try_from(value) {
            return Ok(Self::PulmonicConsonant(cons));
        }
        Err(anyhow!("can't construct LetterType from {}", value))
    }
}

impl fmt::Display for LetterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LetterType::Suprasegmental(s) => s.to_string(),
                LetterType::PulmonicConsonant(p) => p.to_string(),
                LetterType::Vowel(v) => v.to_string(),
                _ => return Err(fmt::Error),
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
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
#[derive(PartialEq, Clone, Debug)]
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

impl std::fmt::Display for Vowel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(a) = VOWEL_LIST
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
impl TryFrom<&str> for Vowel {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for (i, j) in VOWEL_LIST.iter() {
            if j.iter().all(|x| value.contains(*x)) {
                return Ok(i.clone());
            }
        }
        Err(())
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct ChaoToneLetter {
    contour: Vec<ChaoToneLetterHeight>,
    reversed: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ChaoToneLetterHeight {
    ExtraHigh,
    High,
    Mid,
    Low,
    ExtraLow,
}

#[derive(PartialEq, Clone, Copy, Debug)]
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

#[derive(PartialEq, Debug, Clone)]
pub struct Vowel {
    pub height: VowelHeight,
    pub backness: VowelBackness,
    pub roundedness: VowelRoundedness,
}

#[derive(PartialEq, Debug, Clone)]
pub enum VowelHeight {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

#[derive(PartialEq, Debug, Clone)]
pub enum VowelBackness {
    Front,
    Central,
    Back,
}
#[derive(PartialEq, Debug, Clone)]
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

impl std::fmt::Display for PulmonicConsonant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(a) = PULMONIC_CONSONANT_LIST
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
        for (i, j) in PULMONIC_CONSONANT_LIST.iter() {
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
