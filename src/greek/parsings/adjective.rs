use std::ops::{Deref, DerefMut};

use crate::greek::{components::{case::Case, comparison::Comparison, gender::Gender, number::Number, part_of_speech::PartOfSpeech}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdjectiveData {
    gender: Gender,
    number: Number,
    case: Case,
    comparison: Option<Comparison>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdjectiveParsing(Option<AdjectiveData>);

impl Deref for AdjectiveParsing {
    type Target = Option<AdjectiveData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AdjectiveParsing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<GreekWordParsing> for AdjectiveParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Adjective(self)
    }
}

impl PartOfSpeechParsing for AdjectiveParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Adjective
    }

    /// - `Adj`: No components
    /// - `Adj-AFP`: Case, Gender, Number
    /// - `Adj-AFP-C`: Case, Gender, Number, Comparison
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        let Some(segment) = segments.next() else { return Ok(Self(None)) };
        let mut chars = segment
            .split_inclusive(|_| true);

        let case: Case = chars.next().ok_or_else(|| format!("Adjective: Case is required"))?.parse()?;
        let gender: Gender = chars.next().ok_or_else(|| format!("Adjective: Gender is required"))?.parse()?;
        let number: Number = chars.next().ok_or_else(|| format!("Adjective: Number is required"))?.parse()?;

        let comparison: Option<Comparison> = segments.next().map(|c| c.parse()).transpose()?;

        Ok(Self(Some(AdjectiveData {
            gender,
            number,
            case,
            comparison,
        })))
    }

    fn case(&self) -> Option<Case> { self.map(|s| s.case) }
    fn comparison(&self) -> Option<Comparison> { self.and_then(|s| s.comparison) }
    fn gender(&self) -> Option<Gender> { self.map(|s| s.gender) }
    fn number(&self) -> Option<Number> { self.map(|s| s.number) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bsb() -> Result<(), String> {
        assert_eq!(
            GreekWordParsing::parse("Adj")?,
            GreekWordParsing::Adjective(AdjectiveParsing(None))
        );

        assert_eq!(
            GreekWordParsing::parse("Adj-AFP")?,
            GreekWordParsing::Adjective(AdjectiveParsing(Some(AdjectiveData {
                case: Case::Accusative,
                gender: Gender::Feminine,
                number: Number::Plural,
                comparison: None
            })))
        );

        assert_eq!(
            GreekWordParsing::parse("Adj-AFP-C")?,
            GreekWordParsing::Adjective(AdjectiveParsing(Some(AdjectiveData {
                case: Case::Accusative,
                gender: Gender::Feminine,
                number: Number::Plural,
                comparison: Some(Comparison::Comparative)
            })))
        );

        Ok(())
    }
}
