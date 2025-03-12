use std::{fmt::Display, str::FromStr};

use super::ComponentCode;
/**
N - Nominative
V - Vocative
A - Accusative
G - Genitive
D - Dative
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Case {
    /// N - Nominative
    Nominative,
    /// V - Vocative
    Vocative,
    /// A - Accusative
    Accusative,
    /// G - Genitive
    Genitive,
    /// D - Dative
    Dative,
}

impl FromStr for Case {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "n" => Self::Nominative,
            "v" => Self::Vocative,
            "a" => Self::Accusative,
            "g" => Self::Genitive,
            "d" => Self::Dative,
            _ => Err(format!("Invalid Case - '{s}'"))?
        })
    }
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ComponentCode for Case {
    fn code(&self) -> &'static str {
        match self {
            Self::Nominative => "N",
            Self::Vocative => "V",
            Self::Accusative => "A",
            Self::Genitive => "G",
            Self::Dative => "D",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::Nominative => "Nominative",
            Self::Vocative => "Vocative",
            Self::Accusative => "Accusative",
            Self::Genitive => "Genitive",
            Self::Dative => "Dative",
        }
    }
}
