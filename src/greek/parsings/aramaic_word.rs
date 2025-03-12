use crate::greek::{components::part_of_speech::PartOfSpeech, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AramaicWordParsing;

impl Into<GreekWordParsing> for AramaicWordParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::AramaicWord(self)
    }
}

impl PartOfSpeechParsing for AramaicWordParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::AramaicWord
    }

    /// - `Aram`: No components
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
            GreekWordParsing::parse("Aram")?,
            GreekWordParsing::AramaicWord(AramaicWordParsing)
        );

        Ok(())
    }
}
