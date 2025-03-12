use std::{fmt::Display, str::FromStr};

use super::ComponentCode;
/**
C - Comparative
S - Superlative
*/
// Also called Degree
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Comparison {
    /// C - Comparative
    Comparative,
    /// S - Superlative
    Superlative,
}

impl FromStr for Comparison {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "c" => Self::Comparative,
            "s" => Self::Superlative,
            _ => Err(format!("Invalid Comparison - '{s}'"))?
        })
    }
}

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}


impl ComponentCode for Comparison {
    fn code(&self) -> &'static str {
        match self {
            Self::Comparative => "C",
            Self::Superlative => "S",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::Comparative => "Comparative",
            Self::Superlative => "Superlative",
        }
    }
}
