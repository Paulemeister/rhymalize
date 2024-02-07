#![allow(dead_code)]

use std::path::Display;

use reqwest::header::ALT_SVC;

pub struct Letter {
    ipa_type: LetterType,
    diacritics: Option<Vec<Diacritic>>,
}

pub enum LetterType {
    Consonant(Consonant),
    Vowel(Vowel),
    Suprasegmental(Suprasegmental),
}

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

pub struct ChaoToneLetter {
    contour: Vec<ChaoToneLetterHeight>,
    reversed: bool,
}

pub enum ChaoToneLetterHeight {
    ExtraHigh,
    High,
    Mid,
    Low,
    ExtraLow,
}

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

pub enum VowelHeight {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

pub enum VowelBackness {
    Front,
    Central,
    Back,
}

pub enum VowelRoundedness {
    Unrounded,
    Rounded,
}

pub enum Consonant {
    Pulmonic(PulmonicConsonant),
}

pub struct PulmonicConsonant {
    pub manner: PulmonicConsonantManner,
    pub place: ConsonantPlace,
    pub voicing: ConsonantVoicing,
    // exception, as sometimes voiceless consonant has different symbol
    // otherwise it should be a diacritic
}

impl std::fmt::Display for PulmonicConsonant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let PulmonicConsonant {
            manner: m,
            place: p,
            voicing: v,
        } = self;

        let str = match m {
            PulmonicConsonantManner::Nasal => match p {
                ConsonantPlace::Bilabial => match v {
                    ConsonantVoicing::Voiced => "\u{006D}",
                    ConsonantVoicing::Voiceless => "\u{006D}\u{0325}",
                },
                ConsonantPlace::Labiodental => match v {
                    ConsonantVoicing::Voiced => "\u{0271}",
                    ConsonantVoicing::Voiceless => "\u{0271}\u{030A}",
                },
                ConsonantPlace::Linguolabial => match v {
                    ConsonantVoicing::Voiced => "\u{006E}\u{033C}",
                    ConsonantVoicing::Voiceless => return Err(std::fmt::Error),
                },
                ConsonantPlace::Aveolar => match v {
                    ConsonantVoicing::Voiced => "\u{006E}",
                    ConsonantVoicing::Voiceless => "\u{006E}\u{0325}",
                },
                ConsonantPlace::Retroflex => match v {
                    ConsonantVoicing::Voiced => "\u{0273}",
                    ConsonantVoicing::Voiceless => "\u{0273}\u{030A}",
                },
                ConsonantPlace::Palatal => match v {
                    ConsonantVoicing::Voiced => "\u{0272}",
                    ConsonantVoicing::Voiceless => "\u{0272}\u{030A}",
                },
                ConsonantPlace::Velar => match v {
                    ConsonantVoicing::Voiced => "\u{014B}",
                    ConsonantVoicing::Voiceless => "\u{014B}\u{030A}",
                },
                ConsonantPlace::Uvular => match v {
                    ConsonantVoicing::Voiced => "\u{0274}",
                    ConsonantVoicing::Voiceless => "\u{0274}\u{0325}",
                },
                _ => return Err(std::fmt::Error),
            },
            PulmonicConsonantManner::Plosive => match p {
                ConsonantPlace::Bilabial => match v {
                    ConsonantVoicing::Voiced => "\u{0062}",
                    ConsonantVoicing::Voiceless => "\u{0070}",
                },
                ConsonantPlace::Labiodental => match v {
                    ConsonantVoicing::Voiced => "\u{0062}\u{032A}",
                    ConsonantVoicing::Voiceless => "\u{0070}\u{032A}",
                },
                ConsonantPlace::Linguolabial => match v {
                    ConsonantVoicing::Voiced => "\u{0064}\u{033C}",
                    ConsonantVoicing::Voiceless => "\u{0074}\u{033C}",
                },
                ConsonantPlace::Aveolar => match v {
                    ConsonantVoicing::Voiced => "\u{0064}",
                    ConsonantVoicing::Voiceless => "\u{0074}",
                },
                ConsonantPlace::Retroflex => match v {
                    ConsonantVoicing::Voiced => "\u{0256}",
                    ConsonantVoicing::Voiceless => "\u{0288}",
                },
                ConsonantPlace::Palatal => match v {
                    ConsonantVoicing::Voiced => "\u{025F}",
                    ConsonantVoicing::Voiceless => "\u{0063}",
                },
                ConsonantPlace::Velar => match v {
                    ConsonantVoicing::Voiced => "\u{0261}",
                    ConsonantVoicing::Voiceless => "\u{006B}",
                },
                ConsonantPlace::Uvular => match v {
                    ConsonantVoicing::Voiced => "\u{0262}",
                    ConsonantVoicing::Voiceless => "\u{0071}",
                },
                ConsonantPlace::Pharyngeal => match v {
                    ConsonantVoicing::Voiced => return Err(std::fmt::Error),
                    ConsonantVoicing::Voiceless => "\u{02A1}",
                },
                ConsonantPlace::Glottal => match v {
                    ConsonantVoicing::Voiced => return Err(std::fmt::Error),
                    ConsonantVoicing::Voiceless => "\u{0294}",
                },
                _ => return Err(std::fmt::Error),
            },
            PulmonicConsonantManner::SibilantFricative => match p {
                ConsonantPlace::Aveolar => match v {
                    ConsonantVoicing::Voiced => "\u{007A}",
                    ConsonantVoicing::Voiceless => "\u{0073}",
                },
                ConsonantPlace::Postalveolar => match v {
                    ConsonantVoicing::Voiced => "\u{0292}",
                    ConsonantVoicing::Voiceless => "\u{0283}",
                },
                ConsonantPlace::Retroflex => match v {
                    ConsonantVoicing::Voiced => "\u{0290}",
                    ConsonantVoicing::Voiceless => "\u{0282}",
                },
                ConsonantPlace::Palatal => match v {
                    ConsonantVoicing::Voiced => "\u{0291}",
                    ConsonantVoicing::Voiceless => "\u{0255}",
                },
                _ => return Err(std::fmt::Error),
            },
            PulmonicConsonantManner::NonSibilantFricative => match p {
                ConsonantPlace::Bilabial => match v {
                    ConsonantVoicing::Voiced => "\u{03B2}",
                    ConsonantVoicing::Voiceless => "\u{0278}",
                },
                ConsonantPlace::Labiodental => match v {
                    ConsonantVoicing::Voiced => "\u{0076}",
                    ConsonantVoicing::Voiceless => "\u{0066}",
                },
                ConsonantPlace::Linguolabial => match v {
                    ConsonantVoicing::Voiced => "\u{00F0}\u{033C}",
                    ConsonantVoicing::Voiceless => "\u{03B8}\u{033C}",
                },
                ConsonantPlace::Dental => match v {
                    ConsonantVoicing::Voiced => "\u{00F0}",
                    ConsonantVoicing::Voiceless => "\u{03B8}",
                },
                ConsonantPlace::Aveolar => match v {
                    ConsonantVoicing::Voiced => "\u{00F0}\u{0331}",
                    ConsonantVoicing::Voiceless => "\u{03B8}\u{0331}",
                },
                ConsonantPlace::Postalveolar => match v {
                    ConsonantVoicing::Voiced => "\u{0279}\u{0331}\u{02D4}",
                    ConsonantVoicing::Voiceless => "\u{030A}\u{0279}\u{0331}\u{02D4}",
                },
                ConsonantPlace::Retroflex => match v {
                    ConsonantVoicing::Voiced => "\u{027B}\u{02D4}",
                    ConsonantVoicing::Voiceless => "\u{027B}\u{030A}\u{02D4}",
                },
                ConsonantPlace::Palatal => match v {
                    ConsonantVoicing::Voiced => "\u{029D}",
                    ConsonantVoicing::Voiceless => "\u{00E7}",
                },
                ConsonantPlace::Velar => match v {
                    ConsonantVoicing::Voiced => "\u{0263}",
                    ConsonantVoicing::Voiceless => "\u{0078}",
                },
                ConsonantPlace::Uvular => match v {
                    ConsonantVoicing::Voiced => "\u{0281}",
                    ConsonantVoicing::Voiceless => "\u{03C7}",
                },
                ConsonantPlace::Pharyngeal => match v {
                    ConsonantVoicing::Voiced => "\u{0295}",
                    ConsonantVoicing::Voiceless => "\u{0127}",
                },
                ConsonantPlace::Glottal => match v {
                    ConsonantVoicing::Voiced => "\u{0266}",
                    ConsonantVoicing::Voiceless => "\u{0068}",
                },
            },
            PulmonicConsonantManner::Approximant => match p {
                ConsonantPlace::Labiodental => match v {
                    ConsonantVoicing::Voiced => "\u{028B}",
                    ConsonantVoicing::Voiceless => return Err(std::fmt::Error),
                },
                ConsonantPlace::Aveolar => match v {
                    ConsonantVoicing::Voiced => "\u{0279}",
                    ConsonantVoicing::Voiceless => return Err(std::fmt::Error),
                },
                ConsonantPlace::Retroflex => match v {
                    ConsonantVoicing::Voiced => "\u{027B}",
                    ConsonantVoicing::Voiceless => return Err(std::fmt::Error),
                },
                ConsonantPlace::Palatal => match v {
                    ConsonantVoicing::Voiced => "\u{006A}",
                    ConsonantVoicing::Voiceless => return Err(std::fmt::Error),
                },
                ConsonantPlace::Velar => match v {
                    ConsonantVoicing::Voiced => "\u{0270}",
                    ConsonantVoicing::Voiceless => return Err(std::fmt::Error),
                },
                ConsonantPlace::Glottal => match v {
                    ConsonantVoicing::Voiced => "\u{0294}\u{0330}",
                    ConsonantVoicing::Voiceless => return Err(std::fmt::Error),
                },
                _ => return Err(std::fmt::Error),
            },
            _ => "#",
        };

        write!(f, "{str}")
    }
}

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

pub enum ConsonantVoicing {
    Voiced,
    Voiceless,
}
