use serde::Deserialize;
use std::{cmp, error::Error, fmt};
use toml::value::Datetime;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
enum Level {
    #[serde(rename = "0-1")]
    L0_1,
    #[serde(rename = "0-2")]
    L0_2,
    #[serde(rename = "0-3")]
    L0_3,
    #[serde(rename = "0-4")]
    L0_4,
    #[serde(rename = "0-5")]
    L0_5,
    #[serde(rename = "1-1")]
    L1_1,
    #[serde(rename = "1-2")]
    L1_2,
    #[serde(rename = "1-3")]
    L1_3,
    #[serde(rename = "1-4")]
    L1_4,
    #[serde(rename = "2-1")]
    L2_1,
    #[serde(rename = "2-2")]
    L2_2,
    #[serde(rename = "2-3")]
    L2_3,
    #[serde(rename = "2-4")]
    L2_4,
    #[serde(rename = "3-1")]
    L3_1,
    #[serde(rename = "3-2")]
    L3_2,
    #[allow(non_camel_case_types)] // reason = "Prime levels should be consistent with the rest"
    #[serde(rename = "P-1")]
    LP_1,
    #[serde(rename = "4-1")]
    L4_1,
    #[serde(rename = "4-2")]
    L4_2,
    #[serde(rename = "4-3")]
    L4_3,
    #[serde(rename = "4-4")]
    L4_4,
    #[serde(rename = "5-1")]
    L5_1,
    #[serde(rename = "5-2")]
    L5_2,
    #[serde(rename = "5-3")]
    L5_3,
    #[serde(rename = "5-4")]
    L5_4,
    #[serde(rename = "6-1")]
    L6_1,
    #[serde(rename = "6-2")]
    L6_2,
    #[allow(non_camel_case_types)] // reason = "Prime levels should be consistent with the rest"
    #[serde(rename = "P-2")]
    LP_2,
    #[serde(rename = "7-1")]
    L7_1,
    #[serde(rename = "7-2")]
    L7_2,
    #[serde(rename = "7-3")]
    L7_3,
    #[serde(rename = "7-4")]
    L7_4,
    // #[serde(rename = "8-1")]
    // L8_1,
    // #[serde(rename = "8-2")]
    // L8_2,
    // #[serde(rename = "8-3")]
    // L8_3,
    // #[serde(rename = "8-4")]
    // L8_4,
    // #[serde(rename = "9-1")]
    // L9_1,
    // #[serde(rename = "9-2")]
    // L9_2,
    // #[allow(non_camel_case_types)] // reason = "Prime levels should be consistent with the rest"
    // #[serde(rename = "P-3")]
    // LP_3,
    Custom(String),
}

impl fmt::Display for Level {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::L0_1 => write!(f, "0-1: Into The Fire"),
            Self::L0_2 => write!(f, "0-2: The Meatgrinder"),
            Self::L0_3 => write!(f, "0-3: Double Down"),
            Self::L0_4 => write!(f, "0-4: A One Machine Army"),
            Self::L0_5 => write!(f, "0-5: Cerberus"),
            Self::L1_1 => write!(f, "1-1: Heart Of The Sunrise"),
            Self::L1_2 => write!(f, "1-2: The Burning World"),
            Self::L1_3 => write!(f, "1-3: Halls Of Sacred Remains"),
            Self::L1_4 => write!(f, "1-4: Clair De Lune"),
            Self::L2_1 => write!(f, "2-1: Bridgerunner"),
            Self::L2_2 => write!(f, "2-2: Death At 20,000 Volts"),
            Self::L2_3 => write!(f, "2-3: Sheer Heart Attack"),
            Self::L2_4 => write!(f, "2-4: Court Of The Corpse King"),
            Self::L3_1 => write!(f, "3-1: Belly Of The Beast"),
            Self::L3_2 => write!(f, "3-2: In The Flesh"),
            Self::LP_1 => write!(f, "P-1: Soul Survivor"),
            Self::L4_1 => write!(f, "4-1: Slaves To Power"),
            Self::L4_2 => write!(f, "4-2: God Damn The Sun"),
            Self::L4_3 => write!(f, "4-3: A Shot In The Dark"),
            Self::L4_4 => write!(f, "4-4: Clair De Soleil"),
            Self::L5_1 => write!(f, "5-1: In The Wake Of Poseidon"),
            Self::L5_2 => write!(f, "5-2: Waves Of The Starless Sea"),
            Self::L5_3 => write!(f, "5-3: Ship Of Fools"),
            Self::L5_4 => write!(f, "5-4: Leviathan"),
            Self::L6_1 => write!(f, "6-1: Cry For The Weeper"),
            Self::L6_2 => write!(f, "6-2: Aesthetics Of Hate"),
            Self::LP_2 => write!(f, "P-2: Wait Of The World"),
            Self::L7_1 => write!(f, "7-1: Garden Of Forking Paths"),
            Self::L7_2 => write!(f, "7-2: Light Up The Night"),
            Self::L7_3 => write!(f, "7-3: No Sound, No Memory"),
            Self::L7_4 => write!(f, "7-4: ...Like Antennas To Heaven"),
            // Self::L8_1 => write!(f, "8-1"),
            // Self::L8_2 => write!(f, "8-2"),
            // Self::L8_3 => write!(f, "8-3"),
            // Self::L8_4 => write!(f, "8-4"),
            // Self::L9_1 => write!(f, "9-1"),
            // Self::L9_2 => write!(f, "9-2"),
            // Self::LP_3 => write!(f, "P-3"),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
enum Layer {
    #[serde(alias = "Mouth Of Hell")]
    MouthOfHell,
    Limbo,
    Lust,
    Gluttony,
    Greed,
    Wrath,
    Heresy,
    Violence,
    // Fraud,
    // Treachery
}

impl fmt::Display for Layer {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[allow(clippy::wildcard_enum_match_arm)] // reason = "intended"
        match self {
            Self::MouthOfHell => write!(f, "Mouth Of Hell"),
            _ => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Deserialize)]
#[repr(u8)]
enum Act {
    #[serde(
        alias = "Act1",
        alias = "Act 1",
        alias = "ActI",
        alias = "Act I",
        alias = "InfiniteHyperdeath",
        alias = "Infinite Hyperdeath"
    )]
    I,
    #[serde(
        alias = "Act2",
        alias = "Act 2",
        alias = "ActII",
        alias = "Act II",
        alias = "ImperfectHatred",
        alias = "Imperfect Hatred"
    )]
    II,
    // #[serde(
    //     alias = "Act3",
    //     alias = "Act 3",
    //     alias = "ActIII",
    //     alias = "Act III",
    //     alias = "GodfistSuicide",
    //     alias = "Godfist Suicide"
    // )]
    // III,
}

impl fmt::Display for Act {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I => write!(f, "Act 1: Infinite Hyperdeath"),
            Self::II => write!(f, "Act 2: Imperfect Hatred"),
            // Self::III => write!(f, "Act 3: Godfist Suicide")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub enum Track {
    Fullgame,
    #[allow(private_interfaces)] // reason = "enum must be nameable but non-constructable"
    Act(Act),
    #[allow(private_interfaces)] // reason = "enum must be nameable but non-constructable"
    Layer(Layer),
    #[allow(private_interfaces)] // reason = "enum must be nameable but non-constructable"
    Level(Level),
}

impl Track {
    pub const fn shallow_cmp(&self, other: &Self) -> cmp::Ordering {
        use cmp::Ordering;
        match (self, other) {
            (Self::Fullgame, Self::Fullgame)
            | (Self::Act(_), Self::Act(_))
            | (Self::Layer(_), Self::Layer(_))
            | (Self::Level(_), Self::Level(_)) => Ordering::Equal,
            (Self::Fullgame, _) => Ordering::Greater,
            (_, Self::Fullgame) => Ordering::Less,
            (Self::Act(_), _) => Ordering::Greater,
            (_, Self::Act(_)) => Ordering::Less,
            (Self::Layer(_), _) => Ordering::Greater,
            (_, Self::Layer(_)) => Ordering::Less,
        }
    }
}

impl PartialOrd for Track {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Track {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Self::Fullgame, Self::Fullgame) => cmp::Ordering::Equal,
            (Self::Fullgame, _) => cmp::Ordering::Greater,
            (_, Self::Fullgame) => cmp::Ordering::Less,
            (Self::Act(a), Self::Act(b)) => cmp::Ordering::Equal.then(a.cmp(b)),
            (Self::Act(_), _) => cmp::Ordering::Greater,
            (_, Self::Act(_)) => cmp::Ordering::Less,
            (Self::Layer(a), Self::Layer(b)) => cmp::Ordering::Equal.then(a.cmp(b)),
            (Self::Layer(_), _) => cmp::Ordering::Greater,
            (_, Self::Layer(_)) => cmp::Ordering::Less,
            (Self::Level(a), Self::Level(b)) => cmp::Ordering::Equal.then(a.cmp(b)),
            // _ => unreachable!(),
        }
    }
}

impl fmt::Display for Track {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Level(l) => l.fmt(f),
            Self::Layer(l) => l.fmt(f),
            Self::Act(l) => l.fmt(f),
            Self::Fullgame => write!(f, "Fullgame"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Category {
    P,
    Any,
    NoMo,
}

impl fmt::Display for Category {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Difficulty {
    Harmless,
    Lenient,
    Standard,
    Violent,
    Brutal,
    #[serde(rename = "Ultrakill Must Die", alias = "UKMD")]
    UltrakillMustDie,
}

impl fmt::Display for Difficulty {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

type Patch = String;

#[derive(Debug, Clone)]
pub struct Run {
    pub runner: String,
    pub track: Track,
    pub igt_ms: u32,
    pub category: Category,
    pub submission_date: Datetime,
    pub difficulty: Difficulty,
    pub patch_release_date: Patch,
    pub proof: String,
}

mod deserialization;

/// # Errors
/// Errors on parsing errors or on an empty set of runs
// TODO: move away from Box<dyn Error> once the deserialization::parse_toml function does
pub fn load_runs() -> Result<Vec<Run>, Box<dyn Error>> {
    // I have no idea how to read files at runtime
    // HACK: embed the file in the binary
    match deserialization::parse_toml(std::include_str!("../assets/run_data.toml")) {
        Ok(v) if v.is_empty() => Err("No Runs".into()),
        v => v,
    }
    // TODO: add caching
}
