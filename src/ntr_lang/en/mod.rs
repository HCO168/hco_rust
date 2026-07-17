use std::collections::HashMap;

use crate::ntr_lang::core::{
    Aspect,
    Clause,
    LangPack,
    Modifier,
    Tense,
    load_json_map,
};

pub mod regular_past_tense_convert;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct IrregularVerb {
    pub past: String,
    pub past_participle: String,
    pub progressive: String,
    pub present_third_person: String,
}

pub fn load() -> EnglishPack {
    let lang = LangPack {
        noun_values: load_json_map(include_str!("noun_value.json")),
        verb_values: load_json_map(include_str!("verb_value.json")),
        parent_lang: None,
    };

    let irregular_verbs: HashMap<String, IrregularVerb> =
        serde_json::from_str(include_str!("irregular_verbs.json")).unwrap();

    EnglishPack {
        lang,
        irregular_verbs,
    }
}

#[derive(Clone, Debug)]
pub struct EnglishPack {
    pub lang: LangPack,
    pub irregular_verbs: HashMap<String, IrregularVerb>,
}

impl EnglishPack {
    pub fn compose(&self, keys: &[&str]) -> String {
        let tokens: Vec<&str> = keys.iter().map(|k| self.lang.noun(k)).collect();
        tokens.join(" ")
    }

    pub fn compose_sentence(&self, keys: &[&str]) -> String {
        format!("{}.", self.compose(keys))
    }

    pub fn render(&self, clause: &Clause) -> String {
        let subject = self.lang.noun(&clause.subject);
        let verb = self.lang.verb(&clause.verb);

        let verb_phrase = self.render_verb(verb, clause.tense, clause.aspect, clause.negative);

        let mut result = format!("{} {}", subject, verb_phrase);

        if let Some(recipient) = &clause.recipient {
            result.push(' ');
            result.push_str(self.lang.noun(recipient));
        }

        if let Some(object) = &clause.object {
            result.push(' ');
            result.push_str(self.lang.noun(object));
        }

        for modifier in &clause.modifiers {
            match modifier {
                Modifier::Location(place) => {
                    result.push_str(" in ");
                    result.push_str(self.lang.noun(place));
                }
                Modifier::Time(time) => {
                    result.push(' ');
                    result.push_str(time);
                }
                Modifier::Manner(manner) => {
                    result.push(' ');
                    result.push_str(manner);
                }
            }
        }

        result.push('.');
        result
    }

    fn render_verb(
        &self,
        verb: &str,
        tense: Tense,
        aspect: Aspect,
        negative: bool,
    ) -> String {
        if negative {
            return match tense {
                Tense::Present => format!("does not {}", verb),
                Tense::Past => format!("did not {}", verb),
                Tense::Future => format!("will not {}", verb),
            };
        }

        match (tense, aspect) {
            (Tense::Present, Aspect::Simple) => self.present_third_person(verb),
            (Tense::Past, Aspect::Simple) => self.past(verb),
            (Tense::Future, Aspect::Simple) => format!("will {}", verb),

            (Tense::Present, Aspect::Progressive) => {
                format!("is {}", self.progressive(verb))
            }
            (Tense::Past, Aspect::Progressive) => {
                format!("was {}", self.progressive(verb))
            }
            (Tense::Future, Aspect::Progressive) => {
                format!("will be {}", self.progressive(verb))
            }

            (Tense::Present, Aspect::Completed) => {
                format!("has {}", self.past_participle(verb))
            }
            (Tense::Past, Aspect::Completed) => {
                format!("had {}", self.past_participle(verb))
            }
            (Tense::Future, Aspect::Completed) => {
                format!("will have {}", self.past_participle(verb))
            }
        }
    }

    fn past(&self, verb: &str) -> String {
        if let Some(v) = self.irregular_verbs.get(verb) {
            return v.past.clone();
        }

        regular_past_tense_convert::regular_past_tense(verb)
    }

    fn past_participle(&self, verb: &str) -> String {
        if let Some(v) = self.irregular_verbs.get(verb) {
            return v.past_participle.clone();
        }

        regular_past_tense_convert::regular_past_tense(verb)
    }

    fn progressive(&self, verb: &str) -> String {
        if let Some(v) = self.irregular_verbs.get(verb) {
            return v.progressive.clone();
        }

        if verb.ends_with('e') {
            format!("{}ing", &verb[..verb.len() - 1])
        } else {
            format!("{}ing", verb)
        }
    }

    fn present_third_person(&self, verb: &str) -> String {
        if let Some(v) = self.irregular_verbs.get(verb) {
            return v.present_third_person.clone();
        }

        if verb.ends_with('y') {
            format!("{}ies", &verb[..verb.len() - 1])
        } else if verb.ends_with('s')
            || verb.ends_with("sh")
            || verb.ends_with("ch")
            || verb.ends_with('x')
            || verb.ends_with('z')
        {
            format!("{}es", verb)
        } else {
            format!("{}s", verb)
        }
    }
}
