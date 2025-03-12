use crate::greek::{components::{case::Case, gender::Gender, number::Number, part_of_speech::PartOfSpeech, person::Person}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PersonalPossessivePronounParsing {
    case: Case,
    gender: Option<Gender>,
    person: Option<Person>,
    number: Number,
}

impl Into<GreekWordParsing> for PersonalPossessivePronounParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::PersonalPossessivePronoun(self)
    }
}

impl PartOfSpeechParsing for PersonalPossessivePronounParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::PersonalPossessivePronoun
    }

    /// - `PPro-A1P`: Case, Person, Number
    /// - `PPro-AF1P`: Case, Gender, Person, Number
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        let mut chars = segments.next()
            .ok_or_else(|| format!("Case, Person, Number not included"))?
            .split_inclusive(|_| true).peekable();

        let case: Case = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Case is required"))?.parse()?;
        let gender: Option<Gender> = chars.peek().ok_or_else(|| format!("Personal/Possessive Pronoun: Number is required"))?.parse().ok();
        // only consume if the gender successfully parses
        if gender.is_some() { chars.next(); }

        let person: Option<Person> = chars.peek().ok_or_else(|| format!("Personal/Possessive Pronoun: Number is required"))?.parse().ok();
        if person.is_some() { chars.next(); }
        let number: Number = chars.next().ok_or_else(|| format!("Personal/Possessive Pronoun: Number is required"))?.parse()?;

        Ok(Self {
            case,
            gender,
            person,
            number,
        })
    }

    fn case(&self) -> Option<Case> { Some(self.case) }
    fn gender(&self) -> Option<Gender> { self.gender }
    fn person(&self) -> Option<Person> { self.person }
    fn number(&self) -> Option<Number> { Some(self.number) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bsb() -> Result<(), String> {
        assert_eq!(
            GreekWordParsing::parse("PPro-A1P")?,
            GreekWordParsing::PersonalPossessivePronoun(PersonalPossessivePronounParsing {
                case: Case::Accusative,
                gender: None,
                person: Some(Person::First),
                number: Number::Plural,
            })
        );

        assert_eq!(
            GreekWordParsing::parse("PPro-AF1P")?,
            GreekWordParsing::PersonalPossessivePronoun(PersonalPossessivePronounParsing {
                case: Case::Accusative,
                gender: Some(Gender::Feminine),
                person: Some(Person::First),
                number: Number::Plural,
            })
        );

        assert_eq!(
            GreekWordParsing::parse("PPro-NFS")?,
            GreekWordParsing::PersonalPossessivePronoun(PersonalPossessivePronounParsing {
                case: Case::Nominative,
                gender: Some(Gender::Feminine),
                person: None,
                number: Number::Singular,
            })
        );

        Ok(())
    }
}
