use std::{fmt::Display, str::FromStr};

use super::ComponentCode;
/**
A - Active
M - Middle
P - Passive
M/P - Middle or Passive
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Voice {
    /// A - Active
    Active,
    /// M - Middle
    Middle,
    /// P - Passive
    Passive,
    /// M/P - Middle or Passive
    MiddlePassive,
}

impl FromStr for Voice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "a" => Self::Active,
            "m" => Self::Middle,
            "p" => Self::Passive,
            "m/p" => Self::MiddlePassive,
            _ => Err(format!("Invalid Voice - '{s}'"))?
        })
    }
}

impl Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ComponentCode for Voice {
    fn code(&self) -> &'static str {
        match self {
            Self::Active => "A",
            Self::Middle => "M",
            Self::Passive => "P",
            Self::MiddlePassive => "M/P",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Middle => "Middle",
            Self::Passive => "Passive",
            Self::MiddlePassive => "Middle or Passive",
        }
    }
}
