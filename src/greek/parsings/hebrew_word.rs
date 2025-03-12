use crate::greek::{components::part_of_speech::PartOfSpeech, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HebrewWordParsing;

impl Into<GreekWordParsing> for HebrewWordParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::HebrewWord(self)
    }
}

impl PartOfSpeechParsing for HebrewWordParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::HebrewWord
    }

    /// - `Heb`: No components
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
            GreekWordParsing::parse("Heb")?,
            GreekWordParsing::HebrewWord(HebrewWordParsing)
        );

        Ok(())
    }
}
