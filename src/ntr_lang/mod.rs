pub mod lang;
pub mod chinese;

pub use lang::*;
pub use chinese::*;

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    ZhCn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WordKey {
    PersonAlice,
    PersonBob,
    PlaceSky,
    ObjectBook,
    AnimalTurkey,
    CountryTurkey,
    VerbFly,
    VerbGive,
    VerbLike,
}

#[derive(Clone, Copy, Debug)]
pub enum WordCategory {
    Noun,
    Verb,
    Place,
}

#[derive(Clone, Debug)]
pub struct Word {
    pub key: WordKey,
    pub category: WordCategory,
    pub en: &'static str,
    pub zh_cn: &'static str,
}

impl Word {
    pub fn text(&self, lang: Language) -> &'static str {
        match lang {
            Language::En => self.en,
            Language::ZhCn => self.zh_cn,
        }
    }
}

pub struct Dictionary {
    words: HashMap<WordKey, Word>,
}

impl Dictionary {
    pub fn new() -> Self {
        let mut words = HashMap::new();

        words.insert(WordKey::PersonAlice, Word {
            key: WordKey::PersonAlice,
            category: WordCategory::Noun,
            en: "Alice",
            zh_cn: "Alice",
        });

        words.insert(WordKey::PersonBob, Word {
            key: WordKey::PersonBob,
            category: WordCategory::Noun,
            en: "Bob",
            zh_cn: "Bob",
        });

        words.insert(WordKey::PlaceSky, Word {
            key: WordKey::PlaceSky,
            category: WordCategory::Place,
            en: "the sky",
            zh_cn: "天空",
        });

        words.insert(WordKey::ObjectBook, Word {
            key: WordKey::ObjectBook,
            category: WordCategory::Noun,
            en: "a book",
            zh_cn: "一本书",
        });

        words.insert(WordKey::AnimalTurkey, Word {
            key: WordKey::AnimalTurkey,
            category: WordCategory::Noun,
            en: "turkey",
            zh_cn: "火鸡",
        });

        words.insert(WordKey::CountryTurkey, Word {
            key: WordKey::CountryTurkey,
            category: WordCategory::Noun,
            en: "Turkey",
            zh_cn: "土耳其",
        });

        words.insert(WordKey::VerbFly, Word {
            key: WordKey::VerbFly,
            category: WordCategory::Verb,
            en: "fly",
            zh_cn: "飞",
        });

        words.insert(WordKey::VerbGive, Word {
            key: WordKey::VerbGive,
            category: WordCategory::Verb,
            en: "give",
            zh_cn: "给",
        });

        words.insert(WordKey::VerbLike, Word {
            key: WordKey::VerbLike,
            category: WordCategory::Verb,
            en: "like",
            zh_cn: "喜欢",
        });

        Self { words }
    }

    pub fn get(&self, key: WordKey) -> &Word {
        self.words.get(&key).unwrap()
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

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

#[derive(Clone, Debug)]
pub enum Modifier {
    Location(WordKey),
    Time(&'static str),
    Manner(&'static str),
}

#[derive(Clone, Debug)]
pub struct Clause {
    subject: WordKey,
    verb: WordKey,
    object: Option<WordKey>,
    recipient: Option<WordKey>,
    modifiers: Vec<Modifier>,
    tense: Tense,
    aspect: Aspect,
    negative: bool,
}

impl Clause {
    pub fn new(subject: WordKey, verb: WordKey) -> Self {
        Self {
            subject,
            verb,
            object: None,
            recipient: None,
            modifiers: Vec::new(),
            tense: Tense::Present,
            aspect: Aspect::Simple,
            negative: false,
        }
    }

    pub fn object(mut self, object: WordKey) -> Self {
        self.object = Some(object);
        self
    }

    pub fn recipient(mut self, recipient: WordKey) -> Self {
        self.recipient = Some(recipient);
        self
    }

    pub fn location(mut self, location: WordKey) -> Self {
        self.modifiers.push(Modifier::Location(location));
        self
    }

    pub fn time(mut self, time: &'static str) -> Self {
        self.modifiers.push(Modifier::Time(time));
        self
    }

    pub fn manner(mut self, manner: &'static str) -> Self {
        self.modifiers.push(Modifier::Manner(manner));
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

    pub fn render(&self, dict: &Dictionary, lang: Language) -> String {
        match lang {
            Language::En => self.render_en(dict),
            Language::ZhCn => self.render_zh_cn(dict),
        }
    }

    fn render_en(&self, dict: &Dictionary) -> String {
        let subject = dict.get(self.subject).text(Language::En);
        let verb = dict.get(self.verb).text(Language::En);

        let verb_phrase = render_english_verb(verb, self.tense, self.aspect, self.negative);

        let mut result = format!("{} {}", subject, verb_phrase);

        if let Some(recipient) = self.recipient {
            result.push(' ');
            result.push_str(dict.get(recipient).text(Language::En));
        }

        if let Some(object) = self.object {
            result.push(' ');
            result.push_str(dict.get(object).text(Language::En));
        }

        for modifier in &self.modifiers {
            match modifier {
                Modifier::Location(place) => {
                    result.push_str(" in ");
                    result.push_str(dict.get(*place).text(Language::En));
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

    fn render_zh_cn(&self, dict: &Dictionary) -> String {
        // 特判 give，不然中文语序会很怪
        if self.verb == WordKey::VerbGive {
            return self.render_zh_cn_give(dict);
        }

        let subject = dict.get(self.subject).text(Language::ZhCn);
        let verb = dict.get(self.verb).text(Language::ZhCn);

        let mut result = String::new();

        for modifier in &self.modifiers {
            if let Modifier::Time(time) = modifier {
                result.push_str(time);
                result.push('，');
            }
        }

        result.push_str(subject);

        for modifier in &self.modifiers {
            match modifier {
                Modifier::Location(place) => {
                    result.push('在');
                    result.push_str(dict.get(*place).text(Language::ZhCn));
                }
                Modifier::Manner(manner) => {
                    result.push_str(manner);
                    result.push('地');
                }
                Modifier::Time(_) => {}
            }
        }

        if self.negative {
            match self.tense {
                Tense::Past => result.push('没'),
                _ => result.push('不'),
            }
        }

        match self.aspect {
            Aspect::Progressive => {
                if !self.negative {
                    result.push_str("正在");
                }
            }
            _ => {}
        }

        result.push_str(verb);

        if let Some(object) = self.object {
            result.push_str(dict.get(object).text(Language::ZhCn));
        }

        match self.aspect {
            Aspect::Completed => {
                if !self.negative {
                    result.push('了');
                }
            }
            Aspect::Simple => {
                if self.tense == Tense::Past && !self.negative {
                    result.push('了');
                }
            }
            Aspect::Progressive => {}
        }

        result.push('。');
        result
    }

    fn render_zh_cn_give(&self, dict: &Dictionary) -> String {
        let subject = dict.get(self.subject).text(Language::ZhCn);

        let mut result = String::new();

        for modifier in &self.modifiers {
            if let Modifier::Time(time) = modifier {
                result.push_str(time);
                result.push('，');
            }
        }

        result.push_str(subject);

        if self.negative {
            match self.tense {
                Tense::Past => result.push('没'),
                _ => result.push('不'),
            }
        }

        result.push('给');

        if self.tense == Tense::Past && !self.negative {
            result.push('了');
        }

        if let Some(recipient) = self.recipient {
            result.push_str(dict.get(recipient).text(Language::ZhCn));
        }

        if let Some(object) = self.object {
            result.push_str(dict.get(object).text(Language::ZhCn));
        }

        result.push('。');
        result
    }
}

fn render_english_verb(
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
        (Tense::Present, Aspect::Simple) => present_third_person(verb),
        (Tense::Past, Aspect::Simple) => past_tense(verb),
        (Tense::Future, Aspect::Simple) => format!("will {}", verb),

        (Tense::Present, Aspect::Progressive) => format!("is {}", progressive_form(verb)),
        (Tense::Past, Aspect::Progressive) => format!("was {}", progressive_form(verb)),
        (Tense::Future, Aspect::Progressive) => format!("will be {}", progressive_form(verb)),

        (Tense::Present, Aspect::Completed) => format!("has {}", past_participle(verb)),
        (Tense::Past, Aspect::Completed) => format!("had {}", past_participle(verb)),
        (Tense::Future, Aspect::Completed) => format!("will have {}", past_participle(verb)),
    }
}

fn present_third_person(verb: &str) -> String {
    match verb {
        "fly" => "flies".to_string(),
        "give" => "gives".to_string(),
        "like" => "likes".to_string(),
        _ => format!("{}s", verb),
    }
}

fn past_tense(verb: &str) -> String {
    match verb {
        "fly" => "flew".to_string(),
        "give" => "gave".to_string(),
        "like" => "liked".to_string(),
        _ => format!("{}ed", verb),
    }
}

fn past_participle(verb: &str) -> String {
    match verb {
        "fly" => "flown".to_string(),
        "give" => "given".to_string(),
        "like" => "liked".to_string(),
        _ => format!("{}ed", verb),
    }
}

fn progressive_form(verb: &str) -> String {
    match verb {
        "fly" => "flying".to_string(),
        "give" => "giving".to_string(),
        "like" => "liking".to_string(),
        _ => format!("{}ing", verb),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_fly_sentence() {
        let dict = Dictionary::new();

        let clause = Clause::new(WordKey::PersonAlice, WordKey::VerbFly)
            .location(WordKey::PlaceSky);

        assert_eq!(
            clause.render(&dict, Language::En),
            "Alice flies in the sky."
        );

        assert_eq!(
            clause.render(&dict, Language::ZhCn),
            "Alice在天空飞。"
        );
    }

    #[test]
    fn render_give_sentence() {
        let dict = Dictionary::new();

        let clause = Clause::new(WordKey::PersonAlice, WordKey::VerbGive)
            .recipient(WordKey::PersonBob)
            .object(WordKey::ObjectBook)
            .tense(Tense::Past);

        assert_eq!(
            clause.render(&dict, Language::En),
            "Alice gave Bob a book."
        );

        assert_eq!(
            clause.render(&dict, Language::ZhCn),
            "Alice给了Bob一本书。"
        );
    }

    #[test]
    fn render_negative_sentence() {
        let dict = Dictionary::new();

        let clause = Clause::new(WordKey::PersonBob, WordKey::VerbLike)
            .object(WordKey::AnimalTurkey)
            .negative();

        assert_eq!(
            clause.render(&dict, Language::En),
            "Bob does not like turkey."
        );

        assert_eq!(
            clause.render(&dict, Language::ZhCn),
            "Bob不喜欢火鸡。"
        );
    }

    #[test]
    fn render_future_progressive_sentence() {
        let dict = Dictionary::new();

        let clause = Clause::new(WordKey::PersonAlice, WordKey::VerbFly)
            .location(WordKey::PlaceSky)
            .tense(Tense::Future)
            .aspect(Aspect::Progressive);

        assert_eq!(
            clause.render(&dict, Language::En),
            "Alice will be flying in the sky."
        );

        assert_eq!(
            clause.render(&dict, Language::ZhCn),
            "Alice在天空正在飞。"
        );
    }
}