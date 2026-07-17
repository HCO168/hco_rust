# Natural Language Mod Explanation
## Sentences
Components:
* optional logical modifier
* A Subject 
* A Verb with Appropriate Parameter

Example:
* She(&Alice).fly(location=sky)
  * Translation: She (Alice) flies in the sky. 
* For all x:Person(filter={body_height>length(200,centimeter)}),And(x.possess(Respect(from:others)),_.consider(x,tall)) 
  * TL: All people who were taller than 200 centimeter have respect from others and was considered tall. 
## Word

### Nouns
words that represent a thing, a concept, or an events (sentence was considered events)

Example:
* Person
* Isolation
* Universe
* Alice fly in the sky
### Verb
words that represent a moving action, like a function

Example:
* A.hit(object2,VerbModifiers)
* A.give(recipient,object,other modifiers)

### Nouns Modifier
Modify nouns(like adjective), expression is good

Example:
* tall (Vague adjective)
* open (Status for physical objects, precise)
* hard-working (vague)
* height>200cm (precise)
### Verb Modifier
Modify verbs(like adverb)

Example: 
* location=sky (kind of vague)
* duration=small (vague)
* duration=2 to 3 second (precise)
* with:respect (vague)
* carefully (vague)
* repeat=4 times (precise)

## Translation
* Use table for basic noun,verb, and modifiers.
* Use function for sentences or complicate components.

## Categories
Each word need to fall in a category to reduce name overlapping

Example:
* creature: turkey (Meleagris gallopavo)
* social_entity: Turkey
* invention: Box
* shape: Box
* 
### Table
| key             | English     | Chinese |
|-----------------|-------------|---------|
| info.table      | table sheet | 表格      |
| invention.table | table       | 桌子      |
```rust like
fight<Language.zh_cn>(self,other,mod){
  format!("{}{}{}",self.into(Language.zh_cn),modify_verb(verb,mod),other)
}
modify_verb(verb,mod){
    modify_verb_by_time(verb,mod);
    modify_verb_by_vague_adv(verb,mod);
    //...
}
modify_verb_by_time<zh_cn>(verb,mod){
    if(mod.time.is_specific){
        return mod.time+verb
    }
    if(mod.time.happened){
        return verb+"了"
    }
    //other cases...
}
modify_verb_by_time<en>{
    //Modify the verb according to english grammar using mod.time
    //require irregular verb list and verb modify function like past_tence(verb)
}
```

## Website Localization Keys (Current Practice)

Use stable key namespaces so UI text and sentence generation can coexist.

- `ui.*`: UI labels and actions
  - examples: `ui.user`, `ui.username`, `ui.password`, `ui.login`
- `web.*`: web domain entities
  - examples: `web.file`, `web.page`, `web.requested_file`
- `connector.*`: connectors for phrase composition
  - examples: `connector.or`, `connector.is`
- `state.*`: status words
  - examples: `state.invalid`, `state.not_found`, `state.expired`
- `action.*`: action words
  - examples: `action.save`, `action.load`, `action.try_again`
- `verb.*`: composable actions/states for clause generation
  - example: `verb.exist`

Language inheritance fallback:

- Each language pack has a `parent_lang` field.
- Lookup order: current language -> parent language (recursive) -> raw key.
- Current default setup: `zh_cn` falls back to `en` for missing entries.

Example (dynamic sentence):

- clause: `Clause::new("web.requested_file", "verb.exist").negative()`
- English: `the file you requested does not exist.`
- Chinese: `你请求的文件不存在。`

Example (key-based phrase composition):

- keys: `[ui.username, connector.or, ui.password, connector.is, state.invalid]`
- English (`compose_sentence`): `username or password is invalid.`
- Chinese (`compose_sentence`): `用户名或密码无效。`
