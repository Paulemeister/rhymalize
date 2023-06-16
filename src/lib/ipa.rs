#![allow(dead_code)]

pub struct Letter {
    ipa_type: Type,
    diacritics: Vec<Diacritic>,
}

enum Type {
    Consonant(Consonant),
    Vowel(Vowel),
    Suprasegmental(Suprasegmental),
}

enum Diacritic {
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

enum Suprasegmental {
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

struct ChaoToneLetter {
    contour: Vec<ChaoToneLetterHeight>,
    reversed: bool,
}

enum ChaoToneLetterHeight {
    ExtraHigh,
    High,
    Mid,
    Low,
    ExtraLow,
}

enum PitchDiacritic {
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

struct Vowel {
    height: VowelHeight,
    backness: VowelBackness,
    roundedness: VowelRoundedness,
}

enum VowelHeight {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

enum VowelBackness {
    Front,
    Central,
    Back,
}

enum VowelRoundedness {
    Unrounded,
    Rounded,
}

enum Consonant {
    Pulmonic(PulmonicConsonant),
}

struct PulmonicConsonant {
    manner: PulmonicConsonantManner,
    place: ConsonantPlace,
}

enum PulmonicConsonantManner {
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

enum ConsonantPlace {
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
