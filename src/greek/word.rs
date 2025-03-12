use std::{fmt::Display, str::{FromStr, Split}};

use crate::greek::{components::{case::Case, comparison::Comparison, gender::Gender, mood::Mood, number::Number, part_of_speech::PartOfSpeech, person::Person, tense::Tense, voice::Voice}, parsings::{adjective::AdjectiveParsing, adverb::AdverbParsing, aramaic_word::AramaicWordParsing, article::ArticleParsing, conjunction::ConjunctionParsing, demonstrative_pronoun::DemonstrativePronounParsing, hebrew_word::HebrewWordParsing, interjection::InterjectionParsing, interrogative_indefinite_pronoun::InterrogativeIndefinitePronounParsing, noun::NounParsing, particle::ParticleParsing, personal_possessive_pronoun::PersonalPossessivePronounParsing, preposition::PrepositionParsing, reciprocal_pronoun::ReciprocalPronounParsing, reflexive_pronoun::ReflexivePronounParsing, relative_pronoun::RelativePronounParsing, verb::VerbParsing}};

pub trait PartOfSpeechParsing: Sized {
    fn part_of_speech() -> PartOfSpeech;
    fn get_part_of_speech(&self) -> PartOfSpeech {
        Self::part_of_speech()
    }
    fn parse_segments(segments: Split<'_, &str>) -> Result<Self, String>;

    fn case(&self) -> Option<Case> { None }
    fn comparison(&self) -> Option<Comparison> { None }
    fn gender(&self) -> Option<Gender> { None }
    fn mood(&self) -> Option<Mood> { None }
    fn number(&self) -> Option<Number> { None }
    fn person(&self) -> Option<Person> { None }
    fn tense(&self) -> Option<Tense> { None }
    fn voice(&self) -> Option<Voice> { None }
}

/// Resources:
///
/// Format: `Part of Speech  –  Person, Tense, Mood, Voice  –  Case, Number, Gender, Comparison`
///
/// Bible hub has a different order
/// - https://biblehub.com/abbrev.htm
/// - https://biblehub.com/grammar/
/// - https://accordancefiles2.com/helpfiles/14-Win/win14/content/topics/04_gswa/greek_tag_code_tables.htm
/// - https://help.olivetree.com/hc/en-us/articles/360004615912-NA28-Parsings-Guide
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GreekWordParsing {
    Adjective(AdjectiveParsing),
    Adverb(AdverbParsing),
    AramaicWord(AramaicWordParsing),
    Article(ArticleParsing),
    Conjunction(ConjunctionParsing),
    DemonstrativePronoun(DemonstrativePronounParsing),
    HebrewWord(HebrewWordParsing),
    Interjection(InterjectionParsing),
    /// Not sure what this is.
    /// Maybe Indeclinable?
    Indec,
    /// Not sure what this is.
    IntPrtcl,
    InterrogativeIndefinitePronoun(InterrogativeIndefinitePronounParsing),
    Noun(NounParsing),
    Particle(ParticleParsing),
    PersonalPossessivePronoun(PersonalPossessivePronounParsing),
    Preposition(PrepositionParsing),
    ReciprocalPronoun(ReciprocalPronounParsing),
    ReflexivePronoun(ReflexivePronounParsing),
    RelativePronoun(RelativePronounParsing),
    Verb(VerbParsing),
}

impl GreekWordParsing {
    pub fn parse(input: &str) -> Result<GreekWordParsing, String> {
        let mut segments = input.split("-");
        let part_of_speech = segments.next().ok_or_else(|| format!("Part of Speech not included"))?;
        match part_of_speech {
            "Indec" => return Ok(GreekWordParsing::Indec),
            "IntPrtcl" => return Ok(GreekWordParsing::IntPrtcl),
            _ => ()
        };

        let part_of_speech: PartOfSpeech = part_of_speech.parse()?;
        Ok(match part_of_speech {
            PartOfSpeech::Adjective => AdjectiveParsing::parse_segments(segments)?.into(),
            PartOfSpeech::Adverb => AdverbParsing::parse_segments(segments)?.into(),
            PartOfSpeech::AramaicWord => AramaicWordParsing.into(),
            PartOfSpeech::Article => ArticleParsing::parse_segments(segments)?.into(),
            PartOfSpeech::Conjunction => ConjunctionParsing.into(),
            PartOfSpeech::DemonstrativePronoun => DemonstrativePronounParsing::parse_segments(segments)?.into(),
            PartOfSpeech::HebrewWord => HebrewWordParsing.into(),
            PartOfSpeech::Interjection => InterjectionParsing.into(),
            PartOfSpeech::InterrogativeIndefinitePronoun => InterrogativeIndefinitePronounParsing::parse_segments(segments)?.into(),
            PartOfSpeech::Noun => NounParsing::parse_segments(segments)?.into(),
            PartOfSpeech::Particle => ParticleParsing::parse_segments(segments)?.into(),
            PartOfSpeech::PersonalPossessivePronoun => PersonalPossessivePronounParsing::parse_segments(segments)?.into(),
            PartOfSpeech::Preposition => PrepositionParsing::parse_segments(segments)?.into(),
            PartOfSpeech::ReciprocalPronoun => ReciprocalPronounParsing::parse_segments(segments)?.into(),
            PartOfSpeech::ReflexivePronoun => ReflexivePronounParsing::parse_segments(segments)?.into(),
            PartOfSpeech::RelativePronoun => RelativePronounParsing::parse_segments(segments)?.into(),
            PartOfSpeech::Verb => VerbParsing::parse_segments(segments)?.into(),
        })
    }

    pub fn part_of_speech(&self) -> Option<PartOfSpeech> {
        Some(match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.get_part_of_speech(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.get_part_of_speech(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.get_part_of_speech(),
            GreekWordParsing::Article(article_parsing) => article_parsing.get_part_of_speech(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.get_part_of_speech(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.get_part_of_speech(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.get_part_of_speech(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.get_part_of_speech(),
            GreekWordParsing::Indec => None?,
            GreekWordParsing::IntPrtcl => None?,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.get_part_of_speech(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.get_part_of_speech(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.get_part_of_speech(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.get_part_of_speech(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.get_part_of_speech(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.get_part_of_speech(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.get_part_of_speech(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.get_part_of_speech(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.get_part_of_speech(),
        })
    }


    pub fn case(&self) -> Option<Case> {
        match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.case(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.case(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.case(),
            GreekWordParsing::Article(article_parsing) => article_parsing.case(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.case(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.case(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.case(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.case(),
            GreekWordParsing::Indec => None,
            GreekWordParsing::IntPrtcl => None,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.case(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.case(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.case(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.case(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.case(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.case(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.case(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.case(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.case(),
        }
    }

    pub fn comparison(&self) -> Option<Comparison> {
        match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.comparison(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.comparison(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.comparison(),
            GreekWordParsing::Article(article_parsing) => article_parsing.comparison(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.comparison(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.comparison(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.comparison(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.comparison(),
            GreekWordParsing::Indec => None,
            GreekWordParsing::IntPrtcl => None,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.comparison(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.comparison(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.comparison(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.comparison(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.comparison(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.comparison(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.comparison(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.comparison(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.comparison(),
        }
    }

    pub fn gender(&self) -> Option<Gender> {
        match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.gender(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.gender(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.gender(),
            GreekWordParsing::Article(article_parsing) => article_parsing.gender(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.gender(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.gender(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.gender(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.gender(),
            GreekWordParsing::Indec => None,
            GreekWordParsing::IntPrtcl => None,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.gender(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.gender(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.gender(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.gender(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.gender(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.gender(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.gender(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.gender(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.gender(),
        }
    }

    pub fn mood(&self) -> Option<Mood> {
        match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.mood(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.mood(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.mood(),
            GreekWordParsing::Article(article_parsing) => article_parsing.mood(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.mood(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.mood(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.mood(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.mood(),
            GreekWordParsing::Indec => None,
            GreekWordParsing::IntPrtcl => None,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.mood(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.mood(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.mood(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.mood(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.mood(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.mood(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.mood(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.mood(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.mood(),
        }
    }

    pub fn number(&self) -> Option<Number> {
        match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.number(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.number(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.number(),
            GreekWordParsing::Article(article_parsing) => article_parsing.number(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.number(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.number(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.number(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.number(),
            GreekWordParsing::Indec => None,
            GreekWordParsing::IntPrtcl => None,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.number(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.number(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.number(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.number(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.number(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.number(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.number(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.number(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.number(),
        }
    }

    pub fn person(&self) -> Option<Person> {
        match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.person(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.person(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.person(),
            GreekWordParsing::Article(article_parsing) => article_parsing.person(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.person(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.person(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.person(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.person(),
            GreekWordParsing::Indec => None,
            GreekWordParsing::IntPrtcl => None,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.person(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.person(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.person(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.person(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.person(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.person(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.person(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.person(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.person(),
        }
    }

    pub fn tense(&self) -> Option<Tense> {
        match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.tense(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.tense(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.tense(),
            GreekWordParsing::Article(article_parsing) => article_parsing.tense(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.tense(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.tense(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.tense(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.tense(),
            GreekWordParsing::Indec => None,
            GreekWordParsing::IntPrtcl => None,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.tense(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.tense(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.tense(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.tense(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.tense(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.tense(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.tense(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.tense(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.tense(),
        }
    }

    pub fn voice(&self) -> Option<Voice> {
        match self {
            GreekWordParsing::Adjective(adjective_parsing) => adjective_parsing.voice(),
            GreekWordParsing::Adverb(adverb_parsing) => adverb_parsing.voice(),
            GreekWordParsing::AramaicWord(aramaic_word_parsing) => aramaic_word_parsing.voice(),
            GreekWordParsing::Article(article_parsing) => article_parsing.voice(),
            GreekWordParsing::Conjunction(conjunction_parsing) => conjunction_parsing.voice(),
            GreekWordParsing::DemonstrativePronoun(demonstrative_pronoun_parsing) => demonstrative_pronoun_parsing.voice(),
            GreekWordParsing::HebrewWord(hebrew_word_parsing) => hebrew_word_parsing.voice(),
            GreekWordParsing::Interjection(interjection_parsing) => interjection_parsing.voice(),
            GreekWordParsing::Indec => None,
            GreekWordParsing::IntPrtcl => None,
            GreekWordParsing::InterrogativeIndefinitePronoun(interrogative_indefinite_pronoun_parsing) => interrogative_indefinite_pronoun_parsing.voice(),
            GreekWordParsing::Noun(noun_parsing) => noun_parsing.voice(),
            GreekWordParsing::Particle(particle_parsing) => particle_parsing.voice(),
            GreekWordParsing::PersonalPossessivePronoun(personal_possessive_pronoun_parsing) => personal_possessive_pronoun_parsing.voice(),
            GreekWordParsing::Preposition(preposition_parsing) => preposition_parsing.voice(),
            GreekWordParsing::ReciprocalPronoun(reciprocal_pronoun_parsing) => reciprocal_pronoun_parsing.voice(),
            GreekWordParsing::ReflexivePronoun(reflexive_pronoun_parsing) => reflexive_pronoun_parsing.voice(),
            GreekWordParsing::RelativePronoun(relative_pronoun_parsing) => relative_pronoun_parsing.voice(),
            GreekWordParsing::Verb(verb_parsing) => verb_parsing.voice(),
        }
    }
}






















#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn all_codes() {
        let codes = &["N-NFS", "N-GFS", "N-GMS", "N-NMS", "V-AIA-3S", "Art-AMS", "N-AMS", "Conj", "Art-AMP", "PPro-GM3S", "N-AMP", "Prep", "Art-GFS", "Art-GMS", "Art-AFS", "N-AFS", "RelPro-GFS", "V-AIP-3S", "Art-NMS", "V-PPM/P-NMS", "Adj-NFP", "Art-NFP", "N-NFP", "Adv", "Art-NFS", "V-IIA-3S", "V-APP-GFS", "Art-DMS", "N-DMS", "PPro-AM3P", "V-ANA", "V-PPA-NFS", "N-DFS", "Adj-GNS", "N-GNS", "PPro-GF3S", "V-PPA-NMS", "Adj-NMS", "PPro-AF3S", "V-APP-GMS", "DPro-ANP", "V-AMA-2S", "PPro-DM3S", "N-ANS", "N-VMS", "V-ASP-2S", "PPro-G2S", "Art-NNS", "V-APP-NNS", "PPro-DF3S", "V-PIA-3S", "V-FIM-3S", "V-FIA-2S", "Art-ANS", "PPro-NM3S", "V-FIA-3S", "Art-GFP", "PPro-GM3P", "N-GFP", "Adj-NNS", "DPro-NNS", "V-RIA-3S", "V-ASP-3S", "V-PPA-GMS", "V-FIA-3P", "RelPro-NNS", "V-PPM/P-NNS", "PPro-G1P", "V-APP-NMS", "RelPro-GMS", "N-DFP", "N-NMP", "V-AIM-3P", "N-ANP", "V-PPA-NMP", "Art-GMP", "Adj-GMP", "V-AIA-1P", "Art-DFS", "V-APA-NMS", "Adj-NFS", "Adj-AMP", "V-IIM/P-3S", "V-PIM/P-3S", "Art-NMP", "V-AIA-3P", "V-RIM/P-3S", "PPro-N2S", "N-VFS", "V-PIA-2S", "Adj-NFS-S", "Art-DMP", "N-DMP", "RelPro-NMS", "PPro-G1S", "V-PPM/P-GMS", "V-APP-NMP", "V-AMA-2P", "Art-GNS", "V-ASA-2P", "PPro-D1S", "PPro-N1S", "V-ASA-1S", "V-APA-NMP", "V-AIP-3P", "RelPro-AMS", "N-NNS", "Adj-AFS", "Adj-GFS", "V-APA-GMP", "V-PMA-2S", "Prtcl", "PPro-D2S", "V-PNA", "PPro-AN3S", "V-AIA-1S", "N-GMP", "Adj-DNP", "Art-DNP", "N-DNP", "Adj-GMS", "Art-ANP", "V-ANP", "V-PIA-3P", "V-APA-GMS", "V-PMM/P-2S", "V-RIA-3P", "V-PPM/P-AFS", "V-FIP-3S", "Art-DFP", "DPro-DFP", "Adj-DFS", "V-PMA-2P", "DPro-NMS", "Adj-AFP", "N-AFP", "Art-AFP", "PPro-AM3S", "N-NNP", "V-PPM-NMP", "V-IIM/P-3P", "V-PPM/P-AMP", "PPro-DM3P", "N-VNP", "IPro-NMS", "PPro-D2P", "V-PPA-GFS", "Adj-AMS", "RefPro-DM3P", "V-PIA-1P", "V-PIA-1S", "DPro-GMP", "Art-GNP", "N-GNP", "V-PPA-NNS", "PPro-A2P", "N-DNS", "Adj-NMS-C", "Adj-DNS", "V-PIM/P-2S", "PPro-A1S", "PPro-D1P", "V-PPA-ANS", "V-PPM/P-ANS", "RelPro-DMS", "DPro-NMP", "V-ASM-3P", "Adj-DMS", "V-PPM/P-DNS", "PPro-AM2S", "PPro-A2S", "V-ASA-2S", "Adj-ANS", "PPro-GF3P", "Adj-ANP", "V-FIA-1S", "V-IIA-3P", "V-PPM/P-DMP", "V-AIM-3S", "V-PPM/P-AMS", "V-PPA-AMP", "V-M-2P", "Art-DNS", "Adj-DFP", "Adj-NMP", "PPro-NM3P", "V-FIP-3P", "V-FIM-3P", "V-RPM/P-NMP", "V-PIA-2P", "V-ASA-3P", "V-PPM/P-NMP", "PPro-G2P", "V-PMM/P-2P", "PPro-N2P", "IPro-DNS", "V-PNM/P", "V-PPM/P-NFS", "Adj-DMP", "V-AMA-3S", "Heb", "V-ASA-3S", "Adj-NNP", "V-ASM-3S", "Adj-GFP-S", "DPro-GFP", "Adj-NMS-S", "Adj-ANS-C", "V-AIA-2P", "Adj-VMS", "V-PSA-2S", "IPro-ANS", "Adv-S", "V-AMP-2S", "V-FIP-2S", "V-RPM/P-AFS", "V-PMA-3S", "DPro-GNP", "V-PPA-DMS", "V-PPA-AMS", "V-ANM", "V-PPA-GMP", "V-ASM-2P", "IPro-AMS", "IntPrtcl", "V-FIM-2P", "V-ASP-3P", "V-PSA-3S", "V-PSM/P-2P", "V-RPA-NMP", "V-PSM/P-2S", "V-AMM-2S", "V-ASP-2P", "RelPro-GNP", "Art-VMS", "V-AMP-3S", "PPro-N1P", "PPro-A1P", "V-PSA-2P", "IPro-NNS", "V-PIM/P-2P", "DPro-ANS", "Adj-NNS-C", "PPro-AN3P", "PPro-GN3P", "Adj-VMP", "V-ASA-1P", "V-ASM-1P", "Art-NNP", "Adj-GNP", "DPro-NNP", "RefPro-GF3S", "RelPro-DNS", "V-FIP-2P", "PPro-DM2S", "V-FIA-2P", "V-RIA-2P", "V-PPA-DMP", "RelPro-ANP", "V-PSA-3P", "V-RPM/P-NFS", "RelPro-NMP", "V-PIM/P-3P", "DPro-DFS", "Art-VMP", "V-PPM/P-VMP", "DPro-AMP", "V-LIM/P-3S", "RelPro-ANS", "PPro-AM1S", "DPro-DMS", "DPro-AFS", "V-AIA-2S", "V-PPA-AFS", "V-APM-GFS", "RefPro-GM3P", "V-APA-DMS", "V-PIM-1P", "DPro-GFS", "V-PPM/P-GMP", "V-RPM/P-AMS", "N-VNS", "I", "IPro-NMP", "V-RPA-NMS", "V-RSA-2P", "V-APA-AMS", "PPro-GN3S", "V-APA-NFS", "RefPro-DF3S", "V-ASM-1S", "V-FIP-1S", "DPro-NFS", "V-PIM/P-1S", "V-APP-GNS", "V-PPA-NNP", "V-AMP-2P", "V-APM-NMS", "V-RPA-ANP", "RelPro-AFS", "V-AMM-2P", "V-RPM/P-NNS", "V-PMM-2P", "V-RPM/P-NFP", "V-FIM-1S", "Adj-AMS-C", "V-PPM/P-DNP", "RelPro-NNP", "V-AIM-2P", "RelPro-DFP", "Adj-NFP-S", "V-APM-NFP", "V-FIM-2S", "V-PIM-1S", "V-PSM/P-3S", "V-PPA-VMP", "V-RPM/P-VMP", "V-LIA-2P", "V-IIM-3P", "V-APP-NFS", "RefPro-AM3S", "IPro-DMS", "V-APA-NNS", "RefPro-GN3S", "Adj-ANP-C", "V-APA-NNP", "DPro-GMS", "Adj-NNP-C", "V-LIA-3P", "IPro-NFS", "V-LIA-3S", "DPro-DMP", "V-RPM/P-ANS", "RefPro-DM3S", "V-RPM/P-ANP", "V-RPM/P-DMS", "DPro-AMS", "V-APP-DFS", "V-APA-DFS", "DPro-AFP", "V-APM-DNP", "V-PIM-3P", "N-VMP", "V-RIA-2S", "V-PPM/P-NNP", "V-AIP-1S", "V-PPA-GNP", "IPro-AMP", "IPro-AFP", "V-AMM-3S", "V-RPA-GMP", "Adj-VFS", "V-RPM/P-VFS", "V-AIP-1P", "DPro-DNS", "IPro-GMP", "PPro-DN3S", "V-RPM/P-DNP", "V-RPM/P-NNP", "RelPro-GNS", "PPro-AN1S", "V-APM-GMS", "V-APM-ANP", "DPro-GNS", "RelPro-DMP", "RefPro-AM3P", "PPro-DN3P", "V-PPA-ANP", "V-M-2S", "V-RPA-AMP", "V-APA-DMP", "PPro-AN2S", "PPro-DN1P", "V-PIM-2P", "V-PIM/P-1P", "PPro-NN1S", "V-RPM/P-NMS", "Adj-DNP-S", "V-AIM-2S", "IPro-DFS", "V-RIA-1P", "V-AIP-2P", "Adj-AMP-C", "V-PPA-DNS", "V-RPM/P-AMP", "V-RPM/P-DMP", "V-RIA-1S", "RelPro-AMP", "V-RPM-AMS", "IPro-GMS", "V-APP-ANS", "V-RPM/P-GMP", "V-PPM/P-DMS", "V-IIM-1P", "Art-VFS", "V-PPA-VFS", "PPro-GF2S", "RecPro-AMP", "V-RPA-ANS", "V-PMA-3P", "V-PPA-DFP", "RelPro-NFS", "DPro-NFP", "V-PPA-NFP", "RelPro-DFS", "V-PPA-DNP", "RelPro-NFP", "V-APA-NFP", "RefPro-GF3P", "RefPro-DF3P", "V-PPM/P-GFP", "V-IIA-2S", "V-LIA-2S", "V-AIM-1S", "RecPro-GMP", "V-IIM-1S", "Adj-GMP-S", "PPro-NFS", "V-PSA-1S", "V-PSA-1P", "Adj-GFP", "V-IIM/P-1S", "V-IIM-2S", "V-RPA-GMS", "V-RPM/P-GMS", "IPro-ANP", "V-PPA-VMS", "V-FIA-1P", "V-FPA-NMS", "V-APM/P-ANP", "Adv-C", "V-PPM/P-NFP", "Adj-NFS-C", "V-PPA-DFS", "V-APP-NFP", "PPro-DF3P", "V-RPM-NMS", "V-PPM/P-AFP", "V-PNM", "PPro-NM2P", "RefPro-AF3S", "V-RMM/P-2S", "V-RNM/P", "Adj-GMS-S", "Art-VNS", "Adj-VNS", "V-RPA-NNS", "V-RPA-AMS", "PPro-AM2P", "V-APA-AFS", "V-IIM-3S", "V-RPA-NFS", "V-APM-GNS", "V-APA-GFS", "IPro-GFP", "PPro-AM1P", "V-RPA-AFS", "V-APM-NMP", "V-IIM/P-2P", "V-IIA-1P", "RecPro-DMP", "PPro-NF3S", "V-AOA-3S", "V-PPA-GNS", "IPro-NFP", "V-PPM/P-ANP", "V-PPM-NMS", "V-PPM-AMS", "V-RPA-DMP", "RefPro-AF3P", "PPro-AF3P", "V-APM-DMP", "V-RPM/P-GNP", "V-RPA-DMS", "Adj-VMS-S", "RelPro-GMP", "V-AIP-2S", "V-POA-3S", "V-PPM/P-DFS", "V-AOM-3S", "V-APP-AMP", "RelPro-DNP", "RefPro-GM3S", "V-RPM/P-DFS", "V-APP-GNP", "V-IIA-2P", "V-APP-GFP", "V-APA-AMP", "IPro-GFS", "V-RIM/P-3P", "V-AOA-3P", "PPro-NF2P", "PPro-AN2P", "V-PIM-3S", "V-PSM/P-3P", "V-APP-AMS", "V-IIA-1S", "V-RNA", "IPro-AFS", "PPro-NN3S", "V-APA-GNS", "Adj-GNP-C", "Adj-AFP-C", "V-PMM-2S", "Adj-ANS-S", "Adj", "V-RIM/P-2S", "V-ASP-1P", "V-ASP-1S", "PPro-NN1P", "PPro-NN2P", "Adj-NMP-C", "Adj-DNS-S", "DPro-DNP", "V-AMA-3P", "V-APP-ANP", "V-PSM-3S", "V-APM-AMS", "RelPro-GFP", "V-PPM-GMS", "V-PMM/P-3P", "V-PPM/P-GNP", "V-PPM-GNP", "PPro-AF1S", "V-PMM/P-3S", "V-FIM/P-2P", "V-ASM-2S", "IPro-GNS", "PPro-NN2S", "V-FPM-ANS", "V-APM-AMP", "V-APM-AFS", "N-VFP", "V-APP-GMP", "V-APM-ANS", "V-RPA-NFP", "V-PPA-GFP", "V-APM-GFP", "V-RPA-GNP", "V-AIM-1P", "PPro-GM2S", "V-LIA-1S", "V-RIM/P-1S", "Indec", "PPro-NF1S", "V-PPM/P-GNS", "V-PSM/P-1S", "PPro-AF2S", "PPro-GM1S", "PPro-NN3P", "V-APA-ANP", "V-PPM-AFS", "V-PSM/P-1P", "V-FIM-1P", "PPro-NM1S", "V-RIM/P-2P", "Adj-GMP-C", "PPro-DM2P", "PPro-DM1S", "V-RIP-1P", "V-LIM-3P", "IPro-NNP", "PPro-AN1P", "PPro-GN1P", "PPro-AF1P", "PPro-DF1S", "Adj-AFS-C", "PPro-NM2S", "V-RPM/P-GFP", "V-APM/P-GFS", "V-FNA", "V-PPM/P-GFS", "PPro-DF1P", "Adj-DMP-C", "V-RPA-DNS", "V-RPA-GFS", "PPro-DF2S", "V-PPM-DMS", "V-POM/P-1S", "V-PPM-AMP", "V-IIM/P-2S", "V-PPM-NFP", "V-FNM", "V-RPM/P-DFP", "V-PNP", "V-RMM/P-2P", "V-APA-DFP", "V-RPM/P-AFP", "Adj-ANP-S", "V-PPM-GMP", "V-APA-AFP", "V-FPA-ANP", "V-IIM/P-1P", "V-APM-GMP", "V-RPM/P-VMS", "V-APA-ANS", "Adj-DMS-S", "Adj-NFP-C", "V-POA-3P", "V-RIM-2S", "V-POM/P-3S", "Adj-AFS-S", "PPro-GF1P", "V-AOM-1S", "V-LIM-3S", "V-APP-DMS", "V-POM/P-3P", "PPro-GF2P", "IPro-GNP", "V-PPM-ANS", "V-PPP-DMP", "V-PPM/P-VMS", "PPro-DN1S", "V-FIP-1P", "V-RPA-NNP", "Adj-DMS-C", "PPro-GF1S", "V-AMP-3P", "V-APP-AFS", "V-PPP-NMP", "V-PMP-3S", "Adj-VNP", "Art-VNP", "V-PPA-AFP", "V-RPM/P-GNS", "V-PPM-DMP", "V-RSA-1P", "Adj-GNP-S", "V-PIP-3S", "V-PSM-1S", "V-IIP-3P", "V-ANM/P", "V-RSA-1S", "V-PI-3S", "PPro-AF2P", "V-PPM-GFS", "Adj-DFS-C", "V-RPM/P-GFS", "V-RPM/P-DNS", "V-RIM/P-1P", "V-PIP-1S", "V-PPP-GMS", "V-RIM-3S", "IPro-DMP", "V-PPM-NFS", "RecPro-DNP", "PPro-DF2P", "V-RMA-2P", "Art-VFP", "V-PPM-NNS", "V-PMP-2P", "V-AOP-3S", "RelPro-AFP", "V-PSM-2S", "V-PPM/P-DFP", "V-RSA-2S", "PPro-DM1P", "V-PPP-GMP", "PPro-NM1P", "V-APP-DNP", "Adj-GFS-C", "V-FPP-GNP", "Adj-GMS-C", "Adj-DFP-C", "V-APM-GNP", "V-APP-NNP", "V-APM-NFS", "V-IIP-1P", "V-FPA-NMP", "Adj-GNS-S", "V-PPM-GFP", "V-AMM-3P", "V-PPM-GNS", "V-APP-DNS", "Adj-DNS-C", "V-POA-2P", "V-APM/P-NMS", "Adj-VMP-C", "V-APM-NNS", "PPro-NF1P", "V-PSM-1P", "RefPro-AN3P", "RefPro-GN3P", "Adj-DFS-S", "N", "V-RPM-NMP"];

        let errors = codes.iter().filter_map(|input| {
            let result = GreekWordParsing::parse(input);
            result.is_err().then(|| format!("Code: \"{}\"\nReason: \"{}\"", input, result.unwrap_err()))
        }).collect_vec();

        if errors.len() > 0 {
            panic!("{}\nTotal errors: {}", errors.join("\n\n"), errors.len());
        }
    }
}

