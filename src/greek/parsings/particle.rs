use crate::greek::{components::part_of_speech::PartOfSpeech, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParticleParsing;

impl Into<GreekWordParsing> for ParticleParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Particle(self)
    }
}

impl PartOfSpeechParsing for ParticleParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Particle
    }

    /// - `Prtcl`: No components
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
            GreekWordParsing::parse("Prtcl")?,
            GreekWordParsing::Particle(ParticleParsing)
        );

        Ok(())
    }
}
