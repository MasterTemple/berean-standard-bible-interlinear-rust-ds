use crate::greek::{components::{case::Case, gender::Gender, number::Number, part_of_speech::PartOfSpeech, person::Person}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReflexivePronounParsing {
    case: Case,
    gender: Gender,
    person: Person,
    number: Number,
}

impl Into<GreekWordParsing> for ReflexivePronounParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::ReflexivePronoun(self)
    }
}

impl PartOfSpeechParsing for ReflexivePronounParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::ReflexivePronoun
    }

    /// - `RefPro-AF3P`: Case, Gender, Person, Number
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        let mut chars = segments.next()
            .ok_or_else(|| format!("Case, Gender, Person, Number not included"))?
            .split_inclusive(|_| true).peekable();

        let case: Case = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Case is required"))?.parse()?;
        let gender: Gender = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Gender is required"))?.parse()?;
        let person: Person = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Person is required"))?.parse()?;
        let number: Number = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Number is required"))?.parse()?;

        Ok(Self {
            case,
            gender,
            person,
            number,
        })
    }

    fn case(&self) -> Option<Case> { Some(self.case) }
    fn gender(&self) -> Option<Gender> { Some(self.gender) }
    fn person(&self) -> Option<Person> { Some(self.person) }
    fn number(&self) -> Option<Number> { Some(self.number) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bsb() -> Result<(), String> {
        assert_eq!(
            GreekWordParsing::parse("RefPro-AF3P")?,
            GreekWordParsing::ReflexivePronoun(ReflexivePronounParsing {
                case: Case::Accusative,
                gender: Gender::Feminine,
                person: Person::Third,
                number: Number::Plural,
            })
        );

        Ok(())
    }
}
