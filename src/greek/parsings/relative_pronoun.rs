use crate::greek::{components::{case::Case, gender::Gender, number::Number, part_of_speech::PartOfSpeech, person::Person}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RelativePronounParsing {
    case: Case,
    gender: Gender,
    number: Number,
}

impl Into<GreekWordParsing> for RelativePronounParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::RelativePronoun(self)
    }
}

impl PartOfSpeechParsing for RelativePronounParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::RelativePronoun
    }

    /// - `RelPro-AFP`: Case, Gender, Number
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        let mut chars = segments.next()
            .ok_or_else(|| format!("Case, Gender, Number not included"))?
            .split_inclusive(|_| true).peekable();

        let case: Case = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Case is required"))?.parse()?;
        let gender: Gender = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Gender is required"))?.parse()?;
        let number: Number = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Number is required"))?.parse()?;

        Ok(Self {
            case,
            gender,
            number,
        })
    }

    fn case(&self) -> Option<Case> { Some(self.case) }
    fn gender(&self) -> Option<Gender> { Some(self.gender) }
    fn number(&self) -> Option<Number> { Some(self.number) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bsb() -> Result<(), String> {
        assert_eq!(
            GreekWordParsing::parse("RelPro-AFP")?,
            GreekWordParsing::RelativePronoun(RelativePronounParsing {
                case: Case::Accusative,
                gender: Gender::Feminine,
                number: Number::Plural,
            })
        );

        Ok(())
    }
}
