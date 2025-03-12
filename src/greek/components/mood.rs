use std::{fmt::Display, str::FromStr};

use super::ComponentCode;
/**
I - Indicative
M - Imperative
S - Subjunctive
O - Optative
N - Infinitive
P - Participle
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mood {
    /// I - Indicative
    Indicative,
    /// M - Imperative
    Imperative,
    /// S - Subjunctive
    Subjunctive,
    /// O - Optative
    Optative,
    /// N - Infinitive
    Infinitive,
    /// P - Participle
    Participle,
}

impl FromStr for Mood {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "i" => Self::Indicative,
            "m" => Self::Imperative,
            "s" => Self::Subjunctive,
            "o" => Self::Optative,
            "n" => Self::Infinitive,
            "p" => Self::Participle,
            _ => Err(format!("Invalid Mood - '{s}'"))?
        })
    }
}

impl Display for Mood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ComponentCode for Mood {
    fn code(&self) -> &'static str {
        match self {
            Self::Indicative => "I",
            Self::Imperative => "M",
            Self::Subjunctive => "S",
            Self::Optative => "O",
            Self::Infinitive => "N",
            Self::Participle => "P",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::Indicative => "Indicative",
            Self::Imperative => "Imperative",
            Self::Subjunctive => "Subjunctive",
            Self::Optative => "Optative",
            Self::Infinitive => "Infinitive",
            Self::Participle => "Participle",
        }
    }
}
