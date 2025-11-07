pub trait LanguageParser {
    fn name(&self) -> &str;
    fn number_to_text(&self, n: u64) -> String;

}
pub struct Chinese {

}
impl LanguageParser for Chinese{
    fn name(&self) -> &str {
        return "Chinese";
    }

    fn number_to_text(&self, n: u64) -> String {
        todo!()
    }
}