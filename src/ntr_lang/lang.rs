use std::debug_assert;
use crate::dprintln;
use crate::math::digits::Digits;

#[derive(Debug)]
pub enum NumberConvertError{
    InvalidNumber,
    NumberOutOfRange,
}
pub trait NumToLangParser {
    fn number_to_text(&self, n: Digits) -> Result<String,NumberConvertError>;
}
pub trait LanguageParser {
    fn name() -> &'static str;
}