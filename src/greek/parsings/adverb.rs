use std::ops::{Deref, DerefMut};

use crate::greek::{components::{case::Case, comparison::Comparison, gender::Gender, number::Number, part_of_speech::PartOfSpeech}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdverbParsing {
    comparison: Option<Comparison>
}

impl Into<GreekWordParsing> for AdverbParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Adverb(self)
    }
}

impl PartOfSpeechParsing for AdverbParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Adverb
    }

    /// - `Adv`: No components
    /// - `Adv-C`: Comparison
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        let Some(segment) = segments.next() else { return Ok(Self { comparison: None }) };
        let mut chars = segment
            .split_inclusive(|_| true);

        let comparison: Option<Comparison> = chars.next().map(|c| c.parse()).transpose()?;

        Ok(Self {
            comparison,
        })
    }

    fn comparison(&self) -> Option<Comparison> { self.comparison }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bsb() -> Result<(), String> {
        assert_eq!(
            GreekWordParsing::parse("Adv")?,
            GreekWordParsing::Adverb(AdverbParsing { comparison: None })
        );

        assert_eq!(
            GreekWordParsing::parse("Adv-C")?,
            GreekWordParsing::Adverb(AdverbParsing { comparison: Some(Comparison::Comparative) })
        );

        Ok(())
    }
}
