use crate::math::digits::Digits;

pub trait LanguageParser {
    fn name() -> &'static str;
    fn number_to_text(&self, n: Digits) -> Result<String,&str>;
}