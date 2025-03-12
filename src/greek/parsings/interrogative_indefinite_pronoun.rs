use crate::greek::{components::{case::Case, gender::Gender, number::Number, part_of_speech::PartOfSpeech}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InterrogativeIndefinitePronounParsing {
    gender: Gender,
    number: Number,
    case: Case,
}

impl Into<GreekWordParsing> for InterrogativeIndefinitePronounParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::InterrogativeIndefinitePronoun(self)
    }
}

impl PartOfSpeechParsing for InterrogativeIndefinitePronounParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::InterrogativeIndefinitePronoun
    }

    /// Case, Gender, Number
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        let mut chars = segments.next()
            .ok_or_else(|| format!("Case, Number, Gender not included"))?
            .split_inclusive(|_| true);
        let case: Case = chars.next().ok_or_else(|| format!("InterrogativePronoun: Case is required"))?.parse()?;
        let gender: Gender = chars.next().ok_or_else(|| format!("InterrogativePronoun: Gender is required"))?.parse()?;
        let number: Number = chars.next().ok_or_else(|| format!("InterrogativePronoun: Number is required"))?.parse()?;

        Ok(Self {
            gender,
            number,
            case,
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
            GreekWordParsing::parse("IPro-AFP")?,
            GreekWordParsing::InterrogativeIndefinitePronoun(InterrogativeIndefinitePronounParsing {
                case: Case::Accusative,
                gender: Gender::Feminine,
                number: Number::Plural,
            })
        );

        Ok(())
    }
}
