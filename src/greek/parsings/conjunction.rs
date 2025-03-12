use crate::greek::{components::part_of_speech::PartOfSpeech, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConjunctionParsing;

impl Into<GreekWordParsing> for ConjunctionParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Conjunction(self)
    }
}

impl PartOfSpeechParsing for ConjunctionParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Conjunction
    }

    /// - `Conj`: No components
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
            GreekWordParsing::parse("Conj")?,
            GreekWordParsing::Conjunction(ConjunctionParsing)
        );

        Ok(())
    }
}
