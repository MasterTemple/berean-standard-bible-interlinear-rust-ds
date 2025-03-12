use crate::greek::{components::part_of_speech::PartOfSpeech, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct InterjectionParsing;

impl Into<GreekWordParsing> for InterjectionParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Interjection(self)
    }
}

impl PartOfSpeechParsing for InterjectionParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Interjection
    }

    /// - `I`: No components
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
            GreekWordParsing::parse("I")?,
            GreekWordParsing::Interjection(InterjectionParsing)
        );

        Ok(())
    }
}
