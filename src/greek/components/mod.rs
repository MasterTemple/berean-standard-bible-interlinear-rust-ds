pub mod case;
pub mod comparison;
pub mod gender;
pub mod mood;
pub mod number;
pub mod part_of_speech;
pub mod person;
pub mod tense;
pub mod voice;

pub trait ComponentCode {
    fn code(&self) -> &'static str;
    fn code_name(&self) -> &'static str;
}
