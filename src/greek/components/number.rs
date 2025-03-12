use std::{fmt::Display, str::FromStr};

use super::ComponentCode;
/**
S - Singular
P - Plural
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Number {
    /// S - Singular
    Singular,
    /// P - Plural
    Plural,
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "s" => Self::Singular,
            "p" => Self::Plural,
            _ => Err(format!("Invalid Number - '{s}'"))?
        })
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ComponentCode for Number {
    fn code(&self) -> &'static str {
        match self {
            Self::Singular => "S",
            Self::Plural => "P",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::Singular => "Singular",
            Self::Plural => "Plural",
        }
    }
}
