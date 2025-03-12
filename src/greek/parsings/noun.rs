use std::ops::{Deref, DerefMut};

use crate::greek::{components::{case::Case, comparison::Comparison, gender::Gender, number::Number, part_of_speech::PartOfSpeech}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NounData {
    gender: Gender,
    number: Number,
    case: Case,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NounParsing(Option<NounData>);

impl Deref for NounParsing {
    type Target = Option<NounData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for NounParsing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<GreekWordParsing> for NounParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Noun(self)
    }
}

impl PartOfSpeechParsing for NounParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Noun
    }

    /// - `N`: No components
    /// - `N-AFP`: Case, Gender, Number
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        let Some(segment) = segments.next() else { return Ok(Self(None)) };
        let mut chars = segment
            .split_inclusive(|_| true);

        let case: Case = chars.next().ok_or_else(|| format!("Noun: Case is required"))?.parse()?;
        let gender: Gender = chars.next().ok_or_else(|| format!("Noun: Gender is required"))?.parse()?;
        let number: Number = chars.next().ok_or_else(|| format!("Noun: Number is required"))?.parse()?;

        Ok(Self(Some(NounData {
            gender,
            number,
            case,
        })))
    }

    fn case(&self) -> Option<Case> { self.map(|s| s.case) }
    fn gender(&self) -> Option<Gender> { self.map(|s| s.gender) }
    fn number(&self) -> Option<Number> { self.map(|s| s.number) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bsb() -> Result<(), String> {
        assert_eq!(
            GreekWordParsing::parse("N")?,
            GreekWordParsing::Noun(NounParsing(None))
        );

        assert_eq!(
            GreekWordParsing::parse("N-AFP")?,
            GreekWordParsing::Noun(NounParsing(Some(NounData {
                case: Case::Accusative,
                gender: Gender::Feminine,
                number: Number::Plural,
            })))
        );

        Ok(())
    }
}
