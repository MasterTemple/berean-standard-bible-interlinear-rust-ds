use std::{fmt::Display, str::FromStr};

use super::ComponentCode;
/**
P - Present
I - Imperfect
F - Future
A - Aorist
R - Perfect
L - Pluperfect
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tense {
    /// P - Present
    Present,
    /// I - Imperfect
    Imperfect,
    /// F - Future
    Future,
    /// A - Aorist
    Aorist,
    /// R - Perfect
    Perfect,
    /// L - Pluperfect
    Pluperfect,
}

impl FromStr for Tense {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "p" => Self::Present,
            "i" => Self::Imperfect,
            "f" => Self::Future,
            "a" => Self::Aorist,
            "r" => Self::Perfect,
            "l" => Self::Pluperfect,
            _ => Err(format!("Invalid Tense - '{s}'"))?
        })
    }
}

impl Display for Tense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ComponentCode for Tense {
    fn code(&self) -> &'static str {
        match self {
            Self::Present => "P",
            Self::Imperfect => "I",
            Self::Future => "F",
            Self::Aorist => "A",
            Self::Perfect => "R",
            Self::Pluperfect => "L",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::Present => "Present",
            Self::Imperfect => "Imperfect",
            Self::Future => "Future",
            Self::Aorist => "Aorist",
            Self::Perfect => "Perfect",
            Self::Pluperfect => "Pluperfect",
        }
    }
}
