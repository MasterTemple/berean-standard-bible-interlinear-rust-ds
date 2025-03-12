use std::{fmt::Display, str::FromStr};

use super::ComponentCode;
/**
M - Masculine
F - Feminine
N - Neuter
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gender {
    /// M - Masculine
    Masculine,
    /// F - Feminine
    Feminine,
    /// N - Neuter
    Neuter,
}

impl FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "m" => Self::Masculine,
            "f" => Self::Feminine,
            "n" => Self::Neuter,
            _ => Err(format!("Invalid Gender - '{s}'"))?
        })
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ComponentCode for Gender {
    fn code(&self) -> &'static str {
        match self {
            Self::Masculine => "M",
            Self::Feminine => "F",
            Self::Neuter => "N",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::Masculine => "Masculine",
            Self::Feminine => "Feminine",
            Self::Neuter => "Neuter",
        }
    }
}
