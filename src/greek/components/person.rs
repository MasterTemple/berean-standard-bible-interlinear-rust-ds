use std::{fmt::Display, str::FromStr};

use super::ComponentCode;

/**
1 - 1st Person
2 - 2nd Person
3 - 3rd Person
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Person {
    /// 1 - 1st Person
    First,
    /// 2 - 2nd Person
    Second,
    /// 3 - 3rd Person
    Third
}

impl FromStr for Person {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "1" => Self::First,
            "2" => Self::Second,
            "3" => Self::Third,
            _ => Err(format!("Invalid Person - '{s}'"))?
        })
    }

}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ComponentCode for Person {
    fn code(&self) -> &'static str {
        match self {
            Self::First => "1",
            Self::Second => "2",
            Self::Third => "3",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::First => "1st Person",
            Self::Second => "2nd Person",
            Self::Third => "3rd Person",
        }
    }
}
