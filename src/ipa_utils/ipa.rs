#![allow(dead_code)]

use core::fmt;

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
    if value.contains('\u{0329}') || value.contains('\u{030D}') {
        diacs.push(Diacritic::Syllabic)
    }
    if value.contains('\u{032F}') || value.contains('\u{0311}') {
        diacs.push(Diacritic::NonSyllabic)
    }
    if value.contains('\u{02B0}') {
        diacs.push(Diacritic::Aspirated)
    }
    if value.contains('\u{031A}') {
        diacs.push(Diacritic::NoAudibleRelease)
    }
    if value.contains('\u{207F}') {
        diacs.push(Diacritic::NasalRelease)
    }
    if value.contains('\u{02E1}') {
        diacs.push(Diacritic::LateralRelease)
    }
    if value.contains('\u{1DBF}') {
        diacs.push(Diacritic::VoicelessDentalFricativeRelease)
    }
    if value.contains('\u{02E3}') {
        diacs.push(Diacritic::VoicelessVelarFricativeRelease)
    }
    if value.contains('\u{1D4A}') {
        diacs.push(Diacritic::MidCentralVowelRelease)
    }
    // attention: used for specific consonants as well. duplicate in diacritics?
    if value.contains('\u{0325}') || value.contains('\u{030A}') {
        diacs.push(Diacritic::Voiceless)
    }
    if value.contains('\u{032C}') {
        diacs.push(Diacritic::Voiced)
    }
    if value.contains('\u{0324}') {
        diacs.push(Diacritic::BreathyVoiced)
    }
    if value.contains('\u{0330}') {
        diacs.push(Diacritic::CreakyVoiced)
    }
    if value.contains('\u{032A}') || value.contains('\u{0346}') {
        diacs.push(Diacritic::Dental)
    }
    if value.contains('\u{033C}') {
        diacs.push(Diacritic::Linguolabial)
    }
    if value.contains('\u{033A}') {
        diacs.push(Diacritic::Apical)
    }
    if value.contains('\u{033B}') {
        diacs.push(Diacritic::Laminal)
    }
    if value.contains('\u{031F}') {
        // has other
        diacs.push(Diacritic::Advanced)
    }
    if value.contains('\u{0320}') || value.contains('\u{0304}') {
        diacs.push(Diacritic::Retracted)
    }
    if value.contains('\u{0308}') {
        diacs.push(Diacritic::Centralized)
    }
    if value.contains('\u{033D}') {
        diacs.push(Diacritic::MidCentralized)
    }
    if value.contains('\u{031D}') || value.contains('\u{02D4}') {
        diacs.push(Diacritic::Raised)
    }
    if value.contains('\u{031E}') || value.contains('\u{02D5}') {
        diacs.push(Diacritic::Lowered)
    }
    if value.contains('\u{0339}') || value.contains('\u{0357}') {
        diacs.push(Diacritic::MoreRounded)
    }
    if value.contains('\u{031C}') || value.contains('\u{0351}') {
        diacs.push(Diacritic::LessRounded)
    }
    if value.contains('\u{02B7}') {
        diacs.push(Diacritic::Labialized)
    }
    if value.contains('\u{02B2}') {
        diacs.push(Diacritic::Palatalized)
    }
    if value.contains('\u{02E0}') {
        diacs.push(Diacritic::Velarized)
    }
    if value.contains('\u{0334}') {
        diacs.push(Diacritic::VelarizedOrPharyngealized)
    }
    if value.contains('\u{02E4}') {
        diacs.push(Diacritic::Pharyngealized)
    }
    if value.contains('\u{0318}') || value.contains('\u{AB6A}') {
        diacs.push(Diacritic::AdvancedTongueRoot)
    }
    if value.contains('\u{0319}') || value.contains('\u{AB6B}') {
        diacs.push(Diacritic::RetractedTongueRoot)
    }
    if value.contains('\u{0303}') {
        diacs.push(Diacritic::Nasalized)
    }
    if value.contains('\u{02DE}') {
        diacs.push(Diacritic::Rhoticity)
    }

    diacs
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
                    ConsonantVoicing::Voiceless => "\u{033C}\u{03B8}",
                },
                ConsonantPlace::Dental => match v {
                    ConsonantVoicing::Voiced => "\u{00F0}",
                    ConsonantVoicing::Voiceless => "\u{03B8}",
                },
                ConsonantPlace::Aveolar => match v {
                    ConsonantVoicing::Voiced => "\u{00F0}\u{0331}",
                    ConsonantVoicing::Voiceless => "\u{0331}\u{03B8}", // small theta has higher codepoint, so adjacent combining chars will combine to the right
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
