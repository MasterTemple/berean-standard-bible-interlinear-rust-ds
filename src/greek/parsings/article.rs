use crate::greek::{components::{case::Case, gender::Gender, number::Number, part_of_speech::PartOfSpeech}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArticleParsing {
    gender: Gender,
    number: Number,
    case: Case,
}

impl Into<GreekWordParsing> for ArticleParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Article(self)
    }
}

impl PartOfSpeechParsing for ArticleParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Article
    }

    /// Case, Gender, Number
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        let mut chars = segments.next()
            .ok_or_else(|| format!("Case, Number, Gender, Comparison not included"))?
            .split_inclusive(|_| true);
        let case: Case = chars.next().ok_or_else(|| format!("Article: Case is required"))?.parse()?;
        let gender: Gender = chars.next().ok_or_else(|| format!("Article: Gender is required"))?.parse()?;
        let number: Number = chars.next().ok_or_else(|| format!("Article: Number is required"))?.parse()?;

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
            GreekWordParsing::parse("Art-AFP")?,
            GreekWordParsing::Article(ArticleParsing {
                case: Case::Accusative,
                gender: Gender::Feminine,
                number: Number::Plural,
            })
        );

        // let pairs = vec![
        //     ("Art-AFP"),
        //     ("Art-AFS"),
        //     ("Art-AMP"),
        //     ("Art-AMS"),
        //     ("Art-ANP"),
        //     ("Art-ANS"),
        //     ("Art-DFP"),
        //     ("Art-DFS"),
        //     ("Art-DMP"),
        //     ("Art-DMS"),
        //     ("Art-DNP"),
        //     ("Art-DNS"),
        //     ("Art-GFP"),
        //     ("Art-GFS"),
        //     ("Art-GMP"),
        //     ("Art-GMS"),
        //     ("Art-GNP"),
        //     ("Art-GNS"),
        //     ("Art-NFP"),
        //     ("Art-NFS"),
        //     ("Art-NMP"),
        //     ("Art-NMS"),
        //     ("Art-NNP"),
        //     ("Art-NNS"),
        //     ("Art-VFP"),
        //     ("Art-VFS"),
        //     ("Art-VMP"),
        //     ("Art-VMS"),
        //     ("Art-VNP"),
        //     ("Art-VNS"),
        // ];

        Ok(())
    }
}
