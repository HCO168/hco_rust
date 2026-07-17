pub mod lang;
pub mod chinese;

pub mod core;
pub mod en;
pub mod zh_cn;

pub use lang::*;
pub use chinese::*;
pub use core::*;

#[cfg(test)]
mod tests {
    use super::{
        Aspect,
        Clause,
        Tense,
    };

    #[test]
    fn render_english_and_chinese_transfer_sentence() {
        let en = super::en::load();
        let zh = super::zh_cn::load();

        let clause = Clause::new("person.alice", "verb.transfer")
            .recipient("person.bob")
            .object("object.book")
            .tense(Tense::Past);

        assert_eq!(
            en.render(&clause),
            "Alice gave Bob a book."
        );

        assert_eq!(
            zh.render(&clause),
            "Alice给了Bob一本书。"
        );
    }

    #[test]
    fn render_fly_sentence() {
        let en = super::en::load();
        let zh = super::zh_cn::load();

        let clause = Clause::new("person.alice", "verb.fly")
            .location("place.sky")
            .tense(Tense::Future)
            .aspect(Aspect::Progressive);

        assert_eq!(
            en.render(&clause),
            "Alice will be flying in the sky."
        );

        assert_eq!(
            zh.render(&clause),
            "Alice正在在天空中飞。"
        );
    }

    #[test]
    fn render_negative_sentence() {
        let en = super::en::load();
        let zh = super::zh_cn::load();

        let clause = Clause::new("person.bob", "verb.like")
            .object("animal.turkey")
            .negative();

        assert_eq!(
            en.render(&clause),
            "Bob does not like turkey."
        );

        assert_eq!(
            zh.render(&clause),
            "Bob不喜欢火鸡。"
        );
    }

    #[test]
    fn load_key_files() {
        let noun_keys = super::core::load_noun_keys();
        let verb_keys = super::core::load_verb_keys();

        assert!(noun_keys.iter().any(|x| x.key == "person.alice"));
        assert!(verb_keys.iter().any(|x| x.key == "verb.transfer"));
        assert!(noun_keys.iter().any(|x| x.key == "ui.username"));
        assert!(noun_keys.iter().any(|x| x.key == "state.invalid"));
        assert!(noun_keys.iter().any(|x| x.key == "connector.or"));
        assert!(noun_keys.iter().any(|x| x.key == "ui.remember_me"));
        assert!(verb_keys.iter().any(|x| x.key == "verb.exist"));
    }

    #[test]
    fn render_requested_file_not_found_sentence() {
        let en = super::en::load();
        let zh = super::zh_cn::load();

        let clause = Clause::new("web.requested_file", "verb.exist").negative();

        assert_eq!(
            en.render(&clause),
            "the file you requested does not exist."
        );

        assert_eq!(
            zh.render(&clause),
            "你请求的文件不存在。"
        );
    }

    #[test]
    fn website_localization_terms_are_available() {
        let en = super::en::load();
        let zh = super::zh_cn::load();

        assert_eq!(en.lang.noun("ui.username"), "username");
        assert_eq!(zh.lang.noun("ui.username"), "用户名");
        assert_eq!(en.lang.noun("ui.password"), "password");
        assert_eq!(zh.lang.noun("ui.password"), "密码");
        assert_eq!(en.lang.noun("ui.remember_me"), "Remember me");
        assert_eq!(zh.lang.noun("ui.remember_me"), "Remember me");
        assert_eq!(en.lang.noun("state.invalid"), "invalid");
        assert_eq!(zh.lang.noun("state.invalid"), "无效");
    }

    #[test]
    fn compose_username_or_password_invalid_sentence() {
        let en = super::en::load();
        let zh = super::zh_cn::load();

        let keys = [
            "ui.username",
            "connector.or",
            "ui.password",
            "connector.is",
            "state.invalid",
        ];

        assert_eq!(
            en.compose_sentence(&keys),
            "username or password is invalid."
        );
        assert_eq!(
            zh.compose_sentence(&keys),
            "用户名或密码无效。"
        );
    }
}
