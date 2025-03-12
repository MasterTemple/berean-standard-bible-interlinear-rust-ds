use crate::greek::{components::part_of_speech::PartOfSpeech, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrepositionParsing;

impl Into<GreekWordParsing> for PrepositionParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Preposition(self)
    }
}

impl PartOfSpeechParsing for PrepositionParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Preposition
    }

    /// - `Prep`: No components
    fn parse_segments(_segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        Ok(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bsb() -> Result<(), String> {
        assert_eq!(
            GreekWordParsing::parse("Prep")?,
            GreekWordParsing::Preposition(PrepositionParsing)
        );

        Ok(())
    }
}
