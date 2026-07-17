use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tense {
    Present,
    Past,
    Future,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aspect {
    Simple,
    Progressive,
    Completed,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NounKeyData {
    pub key: String,
    pub category: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct VerbKeyData {
    pub key: String,
    pub category: String,
    pub pattern: VerbPattern,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerbPattern {
    Intransitive,
    Transitive,
    Ditransitive,
}

#[derive(Clone, Debug)]
pub enum Modifier {
    Location(String),
    Time(String),
    Manner(String),
}

#[derive(Clone, Debug)]
pub struct Clause {
    pub subject: String,
    pub verb: String,
    pub object: Option<String>,
    pub recipient: Option<String>,
    pub modifiers: Vec<Modifier>,
    pub tense: Tense,
    pub aspect: Aspect,
    pub negative: bool,
}

impl Clause {
    pub fn new(subject: impl Into<String>, verb: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            verb: verb.into(),
            object: None,
            recipient: None,
            modifiers: Vec::new(),
            tense: Tense::Present,
            aspect: Aspect::Simple,
            negative: false,
        }
    }

    pub fn object(mut self, object: impl Into<String>) -> Self {
        self.object = Some(object.into());
        self
    }

    pub fn recipient(mut self, recipient: impl Into<String>) -> Self {
        self.recipient = Some(recipient.into());
        self
    }

    pub fn location(mut self, location: impl Into<String>) -> Self {
        self.modifiers.push(Modifier::Location(location.into()));
        self
    }

    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.modifiers.push(Modifier::Time(time.into()));
        self
    }

    pub fn manner(mut self, manner: impl Into<String>) -> Self {
        self.modifiers.push(Modifier::Manner(manner.into()));
        self
    }

    pub fn tense(mut self, tense: Tense) -> Self {
        self.tense = tense;
        self
    }

    pub fn aspect(mut self, aspect: Aspect) -> Self {
        self.aspect = aspect;
        self
    }

    pub fn negative(mut self) -> Self {
        self.negative = true;
        self
    }
}

#[derive(Clone, Debug)]
pub struct LangPack {
    pub noun_values: HashMap<String, String>,
    pub verb_values: HashMap<String, String>,
    pub parent_lang: Option<Box<LangPack>>,
}

impl LangPack {
    pub fn noun<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(noun) = self.noun_values.get(key) {
            return noun.as_str();
        }

        if let Some(parent) = &self.parent_lang {
            return parent.noun(key);
        }

        key
    }

    pub fn verb<'a>(&'a self, key: &'a str) -> &'a str {
        if let Some(verb) = self.verb_values.get(key) {
            return verb.as_str();
        }

        if let Some(parent) = &self.parent_lang {
            return parent.verb(key);
        }

        key
    }
}

pub fn load_noun_keys() -> Vec<NounKeyData> {
    serde_json::from_str(include_str!("res/noun_key.json")).unwrap()
}

pub fn load_verb_keys() -> Vec<VerbKeyData> {
    serde_json::from_str(include_str!("res/verb_key.json")).unwrap()
}

pub fn load_json_map(text: &str) -> HashMap<String, String> {
    serde_json::from_str(text).unwrap()
}
