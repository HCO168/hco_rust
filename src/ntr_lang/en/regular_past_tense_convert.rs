pub fn regular_past_tense(verb: &str) -> String {
    if verb.ends_with('e') {
        format!("{}d", verb)
    } else if verb.ends_with('y') {
        format!("{}ied", &verb[..verb.len() - 1])
    } else {
        format!("{}ed", verb)
    }
}
