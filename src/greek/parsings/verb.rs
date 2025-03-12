use itertools::Itertools;

use crate::greek::{components::{case::Case, gender::Gender, mood::Mood, number::Number, part_of_speech::PartOfSpeech, person::Person, tense::Tense, voice::Voice}, word::{GreekWordParsing, PartOfSpeechParsing}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerbParsing {
    tense: Option<Tense>,
    mood: Mood,
    voice: Option<Voice>,

    case: Option<Case>,
    gender: Option<Gender>,
    person: Option<Person>,
    number: Option<Number>,
}

pub struct PersonNumber {
    person: Person,
    number: Number,
}

pub enum VerbMoods {
    /// I - Indicative
    /// - V-PI-3S
    /// - V-PIA-1P
    /// - V-LIM/P-3S
    Indicative {
        tense: Tense,
        voice: Option<Voice>,
        person: Person,
        number: Number,
    },
    /// M - Imperative
    /// V-M-2P?
    /// V-M-2S?
    Imperative {
        tense: Tense,
        voice: Voice,
        person: Person,
        number: Number,
    },
    /// S - Subjunctive
    Subjunctive {
        tense: Tense,
        voice: Voice,
        person: Person,
        number: Number,
    },
    /// O - Optative
    Optative {
        tense: Tense,
        voice: Voice,
        person: Person,
        number: Number,
    },
    /// N - Infinitive
    Infinitive {
        tense: Tense,
        voice: Voice,
    },
    /// P - Participle
    /// `V-APA-AFP`: Tense::Aorist, Mood::Participle, Voice::Active - Case::Accusative, Gender::Feminine, Number::Plural
    Participle {
        tense: Tense,
        voice: Voice,
        case: Case,
        gender: Gender,
        number: Number,
    },
}

impl Into<GreekWordParsing> for VerbParsing {
    fn into(self) -> GreekWordParsing {
        GreekWordParsing::Verb(self)
    }
}

impl PartOfSpeechParsing for VerbParsing {
    fn part_of_speech() -> PartOfSpeech {
        PartOfSpeech::Verb
    }

    /// - `RelPro-AFP`: Case, Gender, Number
    fn parse_segments(mut segments: std::str::Split<'_, &str>) -> Result<Self, String> {
        // let first = segments.next().ok_or_else(|| format!("Mood not included"))?;
        let mut chars = segments.next().ok_or_else(|| format!("Mood not included"))?.split_inclusive(|_| true).peekable();

        let tense: Option<Tense> = chars.peek().ok_or_else(|| format!("Verb: Mood is required"))?.parse().ok();
        if tense.is_some() { chars.next(); }
        let mood: Mood = chars.next().ok_or_else(|| format!("Verb: Mood is required"))?.parse()?;
        // M/P is at the end so I can just get the rest
        let voice: Option<Voice> = chars.join("").parse().ok();

        // let second = segments.next();
        let chars = segments.next().map(|seg| seg.split_inclusive(|_| true).peekable());

        // these are always found together, but I don't know Greek to know if it is technically allowable
        let (case, gender, person, number) = match chars {
            Some(mut chars) => {
                let case: Option<Case> = chars.peek().and_then(|c| c.parse().ok());
                // let case: Option<Case> = chars.peek().map(|c| c.parse()).transpose()?;
                if case.is_some() { chars.next(); }

                let person: Option<Person> = chars.peek().and_then(|c| c.parse().ok());
                // let person: Option<Person> = chars.peek().map(|c| c.parse()).transpose()?;
                if person.is_some() { chars.next(); }

                let gender: Option<Gender> = chars.peek().and_then(|c| c.parse().ok());
                // let gender: Option<Gender> = chars.peek().map(|c| c.parse()).transpose()?;
                if gender.is_some() { chars.next(); }

                let number: Option<Number> = chars.peek().and_then(|c| c.parse().ok());
                // let number: Option<Number> = chars.peek().map(|c| c.parse()).transpose()?;
                if person.is_some() { chars.next(); }

                (case, gender, person, number)
            },
            None => (None, None, None, None),
        };

        Ok(Self {
            tense,
            mood,
            voice,
            case,
            gender,
            person,
            number,
        })
    }

    fn case(&self) -> Option<Case> { self.case }
    fn gender(&self) -> Option<Gender> { self.gender }
    fn mood(&self) -> Option<Mood> { Some(self.mood) }
    fn number(&self) -> Option<Number> { self.number }
    fn person(&self) -> Option<Person> { self.person }
    fn tense(&self) -> Option<Tense> { self.tense }
    fn voice(&self) -> Option<Voice> { self.voice }
}

#[cfg(test)]
mod test {
    use core::panic;

    use itertools::Itertools;

    use super::*;

    #[test]
    fn bsb() -> Result<(), String> {

        // V-AIA-1P: Aorist, Indicative, Active - 1 Person, Plural
        assert_eq!(
            GreekWordParsing::parse("V-AIA-1P")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: Some(Tense::Aorist),
                mood: Mood::Indicative,
                voice: Some(Voice::Active),
                case: None,
                gender: None,
                person: Some(Person::First),
                number: Some(Number::Plural),
            })
        );

        // V-ANA: Tense::Aorist, Mood::Infinitive, Voice::Active
        assert_eq!(GreekWordParsing::parse("V-ANA")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: Some(Tense::Aorist),
                mood: Mood::Infinitive,
                voice: Some(Voice::Active),
                case: None,
                gender: None,
                person: None,
                number: None,
            })
        );

        // V-ANM/P: Tense::Aorist, Mood::Infinitive, Voice::Middle/Passive
        assert_eq!(GreekWordParsing::parse("V-ANM/P")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: Some(Tense::Aorist),
                mood: Mood::Infinitive,
                voice: Some(Voice::MiddlePassive),
                case: None,
                gender: None,
                person: None,
                number: None,
            })
        );

        // V-M-2P: Mood::Imperative, Person::Second, Number::Plural
        assert_eq!(GreekWordParsing::parse("V-M-2P")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: None,
                mood: Mood::Imperative,
                voice: None,
                case: None,
                gender: None,
                person: Some(Person::Second),
                number: Some(Number::Plural),
            })
        );

        // V-PI-3S
        assert_eq!(GreekWordParsing::parse("V-PI-3S")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: Some(Tense::Present),
                mood: Mood::Indicative,
                voice: None,
                case: None,
                gender: None,
                person: Some(Person::Third),
                number: Some(Number::Singular),
            })
        );

        // V-APA-AFP
        assert_eq!(GreekWordParsing::parse("V-APA-AFP")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: Some(Tense::Aorist),
                mood: Mood::Participle,
                voice: Some(Voice::Active),
                case: Some(Case::Accusative),
                gender: Some(Gender::Feminine),
                person: None,
                number: Some(Number::Plural),
            })
        );

        // V-APM/P-ANP
        assert_eq!(GreekWordParsing::parse("V-APM/P-ANP")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: Some(Tense::Aorist),
                mood: Mood::Participle,
                voice: Some(Voice::MiddlePassive),
                case: Some(Case::Accusative),
                gender: Some(Gender::Neuter),
                person: None,
                number: Some(Number::Plural),
            })
        );

        // V-IIM/P-1P
        assert_eq!(GreekWordParsing::parse("V-IIM/P-1P")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: Some(Tense::Imperfect),
                mood: Mood::Indicative,
                voice: Some(Voice::MiddlePassive),
                case: None,
                gender: None,
                person: Some(Person::First),
                number: Some(Number::Plural),
            })
        );

        // V-PPM/P-AFP
        assert_eq!(GreekWordParsing::parse("V-PPM/P-AFP")?,
            GreekWordParsing::Verb(VerbParsing {
                tense: Some(Tense::Present),
                mood: Mood::Participle,
                voice: Some(Voice::MiddlePassive),
                case: Some(Case::Accusative),
                gender: Some(Gender::Feminine),
                person: None,
                number: Some(Number::Plural),
            })
        );

        Ok(())
    }

    #[test]
    fn all_verbs() {
        let parsings: &[&str] = &[ "V-AIA-3S", "V-AIP-3S", "V-PPM/P-NMS", "V-IIA-3S", "V-APP-GFS", "V-ANA", "V-PPA-NFS", "V-PPA-NMS", "V-APP-GMS", "V-AMA-2S", "V-ASP-2S", "V-APP-NNS", "V-PIA-3S", "V-FIM-3S", "V-FIA-2S", "V-FIA-3S", "V-RIA-3S", "V-ASP-3S", "V-PPA-GMS", "V-FIA-3P", "V-PPM/P-NNS", "V-APP-NMS", "V-AIM-3P", "V-PPA-NMP", "V-AIA-1P", "V-APA-NMS", "V-IIM/P-3S", "V-PIM/P-3S", "V-AIA-3P", "V-RIM/P-3S", "V-PIA-2S", "V-PPM/P-GMS", "V-APP-NMP", "V-AMA-2P", "V-ASA-2P", "V-ASA-1S", "V-APA-NMP", "V-AIP-3P", "V-APA-GMP", "V-PMA-2S", "V-PNA", "V-AIA-1S", "V-ANP", "V-PIA-3P", "V-APA-GMS", "V-PMM/P-2S", "V-RIA-3P", "V-PPM/P-AFS", "V-FIP-3S", "V-PMA-2P", "V-PPM-NMP", "V-IIM/P-3P", "V-PPM/P-AMP", "V-PPA-GFS", "V-PIA-1P", "V-PIA-1S", "V-PPA-NNS", "V-PIM/P-2S", "V-PPA-ANS", "V-PPM/P-ANS", "V-ASM-3P", "V-PPM/P-DNS", "V-ASA-2S", "V-FIA-1S", "V-IIA-3P", "V-PPM/P-DMP", "V-AIM-3S", "V-PPM/P-AMS", "V-PPA-AMP", "V-M-2P", "V-FIP-3P", "V-FIM-3P", "V-RPM/P-NMP", "V-PIA-2P", "V-ASA-3P", "V-PPM/P-NMP", "V-PMM/P-2P", "V-PNM/P", "V-PPM/P-NFS", "V-AMA-3S", "V-ASA-3S", "V-ASM-3S", "V-AIA-2P", "V-PSA-2S", "V-AMP-2S", "V-FIP-2S", "V-RPM/P-AFS", "V-PMA-3S", "V-PPA-DMS", "V-PPA-AMS", "V-ANM", "V-PPA-GMP", "V-ASM-2P", "V-FIM-2P", "V-ASP-3P", "V-PSA-3S", "V-PSM/P-2P", "V-RPA-NMP", "V-PSM/P-2S", "V-AMM-2S", "V-ASP-2P", "V-AMP-3S", "V-PSA-2P", "V-PIM/P-2P", "V-ASA-1P", "V-ASM-1P", "V-FIP-2P", "V-FIA-2P", "V-RIA-2P", "V-PPA-DMP", "V-PSA-3P", "V-RPM/P-NFS", "V-PIM/P-3P", "V-PPM/P-VMP", "V-LIM/P-3S", "V-AIA-2S", "V-PPA-AFS", "V-APM-GFS", "V-APA-DMS", "V-PIM-1P", "V-PPM/P-GMP", "V-RPM/P-AMS", "V-RPA-NMS", "V-RSA-2P", "V-APA-AMS", "V-APA-NFS", "V-ASM-1S", "V-FIP-1S", "V-PIM/P-1S", "V-APP-GNS", "V-PPA-NNP", "V-AMP-2P", "V-APM-NMS", "V-RPA-ANP", "V-AMM-2P", "V-RPM/P-NNS", "V-PMM-2P", "V-RPM/P-NFP", "V-FIM-1S", "V-PPM/P-DNP", "V-AIM-2P", "V-APM-NFP", "V-FIM-2S", "V-PIM-1S", "V-PSM/P-3S", "V-PPA-VMP", "V-RPM/P-VMP", "V-LIA-2P", "V-IIM-3P", "V-APP-NFS", "V-APA-NNS", "V-APA-NNP", "V-LIA-3P", "V-LIA-3S", "V-RPM/P-ANS", "V-RPM/P-ANP", "V-RPM/P-DMS", "V-APP-DFS", "V-APA-DFS", "V-APM-DNP", "V-PIM-3P", "V-RIA-2S", "V-PPM/P-NNP", "V-AIP-1S", "V-PPA-GNP", "V-AMM-3S", "V-RPA-GMP", "V-RPM/P-VFS", "V-AIP-1P", "V-RPM/P-DNP", "V-RPM/P-NNP", "V-APM-GMS", "V-APM-ANP", "V-PPA-ANP", "V-M-2S", "V-RPA-AMP", "V-APA-DMP", "V-PIM-2P", "V-PIM/P-1P", "V-RPM/P-NMS", "V-AIM-2S", "V-RIA-1P", "V-AIP-2P", "V-PPA-DNS", "V-RPM/P-AMP", "V-RPM/P-DMP", "V-RIA-1S", "V-RPM-AMS", "V-APP-ANS", "V-RPM/P-GMP", "V-PPM/P-DMS", "V-IIM-1P", "V-PPA-VFS", "V-RPA-ANS", "V-PMA-3P", "V-PPA-DFP", "V-PPA-NFP", "V-PPA-DNP", "V-APA-NFP", "V-PPM/P-GFP", "V-IIA-2S", "V-LIA-2S", "V-AIM-1S", "V-IIM-1S", "V-PSA-1S", "V-PSA-1P", "V-IIM/P-1S", "V-IIM-2S", "V-RPA-GMS", "V-RPM/P-GMS", "V-PPA-VMS", "V-FIA-1P", "V-FPA-NMS", "V-APM/P-ANP", "V-PPM/P-NFP", "V-PPA-DFS", "V-APP-NFP", "V-RPM-NMS", "V-PPM/P-AFP", "V-PNM", "V-RMM/P-2S", "V-RNM/P", "V-RPA-NNS", "V-RPA-AMS", "V-APA-AFS", "V-IIM-3S", "V-RPA-NFS", "V-APM-GNS", "V-APA-GFS", "V-RPA-AFS", "V-APM-NMP", "V-IIM/P-2P", "V-IIA-1P", "V-AOA-3S", "V-PPA-GNS", "V-PPM/P-ANP", "V-PPM-NMS", "V-PPM-AMS", "V-RPA-DMP", "V-APM-DMP", "V-RPM/P-GNP", "V-RPA-DMS", "V-AIP-2S", "V-POA-3S", "V-PPM/P-DFS", "V-AOM-3S", "V-APP-AMP", "V-RPM/P-DFS", "V-APP-GNP", "V-IIA-2P", "V-APP-GFP", "V-APA-AMP", "V-RIM/P-3P", "V-AOA-3P", "V-PIM-3S", "V-PSM/P-3P", "V-APP-AMS", "V-IIA-1S", "V-RNA", "V-APA-GNS", "V-PMM-2S", "V-RIM/P-2S", "V-ASP-1P", "V-ASP-1S", "V-AMA-3P", "V-APP-ANP", "V-PSM-3S", "V-APM-AMS", "V-PPM-GMS", "V-PMM/P-3P", "V-PPM/P-GNP", "V-PPM-GNP", "V-PMM/P-3S", "V-FIM/P-2P", "V-ASM-2S", "V-FPM-ANS", "V-APM-AMP", "V-APM-AFS", "V-APP-GMP", "V-APM-ANS", "V-RPA-NFP", "V-PPA-GFP", "V-APM-GFP", "V-RPA-GNP", "V-AIM-1P", "V-LIA-1S", "V-RIM/P-1S", "V-PPM/P-GNS", "V-PSM/P-1S", "V-APA-ANP", "V-PPM-AFS", "V-PSM/P-1P", "V-FIM-1P", "V-RIM/P-2P", "V-RIP-1P", "V-LIM-3P", "V-RPM/P-GFP", "V-APM/P-GFS", "V-FNA", "V-PPM/P-GFS", "V-RPA-DNS", "V-RPA-GFS", "V-PPM-DMS", "V-POM/P-1S", "V-PPM-AMP", "V-IIM/P-2S", "V-PPM-NFP", "V-FNM", "V-RPM/P-DFP", "V-PNP", "V-RMM/P-2P", "V-APA-DFP", "V-RPM/P-AFP", "V-PPM-GMP", "V-APA-AFP", "V-FPA-ANP", "V-IIM/P-1P", "V-APM-GMP", "V-RPM/P-VMS", "V-APA-ANS", "V-POA-3P", "V-RIM-2S", "V-POM/P-3S", "V-AOM-1S", "V-LIM-3S", "V-APP-DMS", "V-POM/P-3P", "V-PPM-ANS", "V-PPP-DMP", "V-PPM/P-VMS", "V-FIP-1P", "V-RPA-NNP", "V-AMP-3P", "V-APP-AFS", "V-PPP-NMP", "V-PMP-3S", "V-PPA-AFP", "V-RPM/P-GNS", "V-PPM-DMP", "V-RSA-1P", "V-PIP-3S", "V-PSM-1S", "V-IIP-3P", "V-ANM/P", "V-RSA-1S", "V-PI-3S", "V-PPM-GFS", "V-RPM/P-GFS", "V-RPM/P-DNS", "V-RIM/P-1P", "V-PIP-1S", "V-PPP-GMS", "V-RIM-3S", "V-PPM-NFS", "V-RMA-2P", "V-PPM-NNS", "V-PMP-2P", "V-AOP-3S", "V-PSM-2S", "V-PPM/P-DFP", "V-RSA-2S", "V-PPP-GMP", "V-APP-DNP", "V-FPP-GNP", "V-APM-GNP", "V-APP-NNP", "V-APM-NFS", "V-IIP-1P", "V-FPA-NMP", "V-PPM-GFP", "V-AMM-3P", "V-PPM-GNS", "V-APP-DNS", "V-POA-2P", "V-APM/P-NMS", "V-APM-NNS", "V-PSM-1P", "V-RPM-NMP" ];

        let errors = parsings.iter().filter_map(|input| {
            let result = GreekWordParsing::parse(input);
            result.is_err().then(|| format!("Code: \"{}\"\nReason: \"{}\"", input, result.unwrap_err()))
        }).collect_vec();

        if errors.len() > 0 {
            panic!("{}\nTotal errors: {}", errors.join("\n\n"), errors.len());
        }
    }
}
