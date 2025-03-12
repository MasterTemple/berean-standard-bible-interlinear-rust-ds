use std::{fmt::Display, str::FromStr};

use super::ComponentCode;

/**
V - Verb
N - Noun
Adv - Adverb
Adj - Adjective
Art - Article
DPro - Demonstrative Pronoun
IPro - Interrogative / Indefinite Pronoun
PPro - Personal / Possessive Pronoun
RecPro - Reciprocal Pronoun
RelPro - Relative Pronoun
RefPro - Reflexive Pronoun
Prep - Preposition
Conj - Conjunction
I - Interjection
Prtcl - Particle
Heb - Hebrew Word
Aram - Aramaic Word
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartOfSpeech {
    /// V - Verb
    Verb,
    /// N - Noun
    Noun,
    /// Adv - Adverb
    Adverb,
    /// Adj - Adjective
    Adjective,
    /// Art - Article
    Article,
    /// DPro - Demonstrative Pronoun
    DemonstrativePronoun,
    /// IPro - Interrogative / Indefinite Pronoun
    InterrogativeIndefinitePronoun,
    /// PPro - Personal / Possessive Pronoun
    PersonalPossessivePronoun,
    /// RecPro - Reciprocal Pronoun
    ReciprocalPronoun,
    /// RelPro - Relative Pronoun
    RelativePronoun,
    /// RefPro - Reflexive Pronoun
    ReflexivePronoun,
    /// Prep - Preposition
    Preposition,
    /// Conj - Conjunction
    Conjunction,
    /// I - Interjection
    Interjection,
    /// Prtcl - Particle
    Particle,
    /// Heb - Hebrew Word
    HebrewWord,
    /// Aram - Aramaic Word
    AramaicWord,
}

impl FromStr for PartOfSpeech {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "v" => Self::Verb,
            "n" => Self::Noun,
            "adv" => Self::Adverb,
            "adj" => Self::Adjective,
            "art" => Self::Article,
            "dpro" => Self::DemonstrativePronoun,
            "ipro" => Self::InterrogativeIndefinitePronoun,
            "ppro" => Self::PersonalPossessivePronoun,
            "recpro" => Self::ReciprocalPronoun,
            "relpro" => Self::RelativePronoun,
            "refpro" => Self::ReflexivePronoun,
            "prep" => Self::Preposition,
            "conj" => Self::Conjunction,
            "i" => Self::Interjection,
            "prtcl" => Self::Particle,
            "heb" => Self::HebrewWord,
            "aram" => Self::AramaicWord,
            _ => Err(format!("Invalid Part of Speech - '{s}'"))?
        })
    }
}

impl Display for PartOfSpeech {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl ComponentCode for PartOfSpeech {
    fn code(&self) -> &'static str {
        match self {
            Self::Verb => "V",
            Self::Noun => "N",
            Self::Adverb => "Adv",
            Self::Adjective => "Adj",
            Self::Article => "Art",
            Self::DemonstrativePronoun => "DPro",
            Self::InterrogativeIndefinitePronoun => "IPro",
            Self::PersonalPossessivePronoun => "PPro",
            Self::ReciprocalPronoun => "RecPro",
            Self::RelativePronoun => "RelPro",
            Self::ReflexivePronoun => "RefPro",
            Self::Preposition => "Prep",
            Self::Conjunction => "Conj",
            Self::Interjection => "I",
            Self::Particle => "Prtcl",
            Self::HebrewWord => "Heb",
            Self::AramaicWord => "Aram",
        }
    }

    fn code_name(&self) -> &'static str {
        match self {
            Self::Verb => "Verb",
            Self::Noun => "Noun",
            Self::Adverb => "Adverb",
            Self::Adjective => "Adjective",
            Self::Article => "Article",
            Self::DemonstrativePronoun => "Demonstrative Pronoun",
            Self::InterrogativeIndefinitePronoun => "Interrogative / Indefinite Pronoun",
            Self::PersonalPossessivePronoun => "Personal / Possessive Pronoun",
            Self::ReciprocalPronoun => "Reciprocal Pronoun",
            Self::RelativePronoun => "Relative Pronoun",
            Self::ReflexivePronoun => "Reflexive Pronoun",
            Self::Preposition => "Preposition",
            Self::Conjunction => "Conjunction",
            Self::Interjection => "Interjection",
            Self::Particle => "Particle",
            Self::HebrewWord => "Hebrew Word",
            Self::AramaicWord => "Aramaic Word",
        }
    }
}
