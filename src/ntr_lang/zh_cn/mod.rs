use crate::ntr_lang::core::{
    Aspect,
    Clause,
    LangPack,
    Modifier,
    Tense,
    load_json_map,
};

pub fn load() -> ChinesePack {
    let parent_lang = LangPack {
        noun_values: load_json_map(include_str!("../en/noun_value.json")),
        verb_values: load_json_map(include_str!("../en/verb_value.json")),
        parent_lang: None,
    };

    let lang = LangPack {
        noun_values: load_json_map(include_str!("noun_value.json")),
        verb_values: load_json_map(include_str!("verb_value.json")),
        parent_lang: Some(Box::new(parent_lang)),
    };
    let location_suffix = load_json_map(include_str!("location_suffix.json"));

    ChinesePack { lang, location_suffix }
}

#[derive(Clone, Debug)]
pub struct ChinesePack {
    pub lang: LangPack,
    pub location_suffix: std::collections::HashMap<String, String>,
}

impl ChinesePack {
    pub fn compose(&self, keys: &[&str]) -> String {
        let tokens: Vec<&str> = keys.iter().map(|k| self.lang.noun(k)).collect();
        tokens.concat()
    }

    pub fn compose_sentence(&self, keys: &[&str]) -> String {
        format!("{}。", self.compose(keys))
    }

    pub fn render(&self, clause: &Clause) -> String {
        if clause.verb == "verb.transfer" {
            return self.render_transfer(clause);
        }

        let subject = self.lang.noun(&clause.subject);
        let verb = self.lang.verb(&clause.verb);

        let mut result = String::new();

        for modifier in &clause.modifiers {
            if let Modifier::Time(time) = modifier {
                result.push_str(time);
                result.push('，');
            }
        }

        result.push_str(subject);

        if clause.aspect == Aspect::Progressive && !clause.negative {
            result.push_str("正在");
        }

        for modifier in &clause.modifiers {
            match modifier {
                Modifier::Location(place) => {
                    result.push('在');
                    result.push_str(self.lang.noun(place));
                    if let Some(suffix) = self.location_suffix.get(place) {
                        result.push_str(suffix);
                    }
                }
                Modifier::Manner(manner) => {
                    result.push_str(manner);
                    result.push('地');
                }
                Modifier::Time(_) => {}
            }
        }

        if clause.negative {
            match clause.tense {
                Tense::Past => result.push('没'),
                _ => result.push('不'),
            }
        }

        result.push_str(verb);

        if let Some(object) = &clause.object {
            result.push_str(self.lang.noun(object));
        }

        if should_add_le(clause) {
            result.push('了');
        }

        result.push('。');
        result
    }

    fn render_transfer(&self, clause: &Clause) -> String {
        let subject = self.lang.noun(&clause.subject);

        let mut result = String::new();

        for modifier in &clause.modifiers {
            if let Modifier::Time(time) = modifier {
                result.push_str(time);
                result.push('，');
            }
        }

        result.push_str(subject);

        if clause.negative {
            match clause.tense {
                Tense::Past => result.push('没'),
                _ => result.push('不'),
            }
        }

        result.push('给');

        if should_add_le(clause) {
            result.push('了');
        }

        if let Some(recipient) = &clause.recipient {
            result.push_str(self.lang.noun(recipient));
        }

        if let Some(object) = &clause.object {
            result.push_str(self.lang.noun(object));
        }

        result.push('。');
        result
    }
}

fn should_add_le(clause: &Clause) -> bool {
    if clause.negative {
        return false;
    }

    clause.tense == Tense::Past || clause.aspect == Aspect::Completed
}
