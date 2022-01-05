use crate::generator::{
    get_condition, get_constant, get_entity, get_entity_prefix, get_entity_suffix, get_entity_tag,
    get_keyword, get_var, hashtag_comment_enable, slash_comment_enable, slash_star_comment_enable,
    string::StringBuilder, xml_comment_enable,
};
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

const ACCEPT_PREFIX_VAR: &str = "ACCEPT_PREFIX_VAR";
const ACCEPT_ENTITY_TAG_PREFIX: &str = "ACCEPT_ENTITY_TAG_PREFIX";
const ENTITY_TAG_PREFIX_CHAR: &str = "ENTITY_TAG_PREFIX_CHAR";
const ACCEPT_PREFIX_KEYWORD: &str = "ACCEPT_PREFIX_KEYWORD";
const ACCEPT_STRING_ONE_QUOTE: &str = "ACCEPT_STRING_ONE_QUOTE";
const ACCEPT_STRING_DOUBLE_QUOTE: &str = "ACCEPT_STRING_DOUBLE_QUOTE";
const ACCEPT_STRING_EOF: &str = "ACCEPT_STRING_EOF";

pub fn generate_token(h: &Hash) -> String {
    let mut token = StringBuilder::new();
    token.push_strln("// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //");
    token.push_strln("#[derive(PartialEq, Debug)]");
    token.push_strln("pub enum Token {");
    token.push_tabln(1, "ILLEGAL,");
    token.push_tabln(1, "EOF,");
    token.push_tabln(1, "ENDL(char),");
    token.push_tabln(1, "INT(Vec<char>),");
    token.push_tabln(1, "IDENT(Vec<char>),");

    if slash_comment_enable(h)
        || slash_star_comment_enable(h)
        || get_condition(h)
            .get(&Yaml::String(ACCEPT_ENTITY_TAG_PREFIX.to_string()))
            .is_some()
        || get_condition(h)
            .get(&Yaml::String(ENTITY_TAG_PREFIX_CHAR.to_string()))
            .is_some()
        || get_condition(h)
            .get(&Yaml::String(ACCEPT_PREFIX_KEYWORD.to_string()))
            .is_some()
    {
        token.push_tabln(1, "CH(char),");
    }

    if get_entity(h).len() >= 1
        || get_entity_tag(h).len() >= 1
        || get_entity_prefix(h).len() >= 1
        || get_entity_suffix(h).len() >= 1
    {
        token.push_tabln(1, "ENTITY(Vec<char>),");
    }

    if get_condition(h)
        .get(&Yaml::String(ACCEPT_STRING_ONE_QUOTE.to_string()))
        .is_some()
        || get_condition(h)
            .get(&Yaml::String(ACCEPT_STRING_DOUBLE_QUOTE.to_string()))
            .is_some()
        || get_condition(h)
            .get(&Yaml::String(ACCEPT_STRING_EOF.to_string()))
            .is_some()
    {
        token.push_tabln(1, "STRING(Vec<char>),");
    }

    if get_constant(h).len() >= 1 {
        token.push_tabln(1, "CONSTANT(Vec<char>),");
    }

    if get_keyword(h).len() >= 1 {
        token.push_tabln(1, "KEYWORD(Vec<char>),");
    }

    if get_entity_tag(h).len() >= 1 {
        token.push_tabln(1, "ENTITYTAG(Vec<char>),");
    }

    if let Some(_) = get_condition(h).get(&Yaml::String(ACCEPT_PREFIX_VAR.to_string())) {
        token.push_tabln(1, "VAR(Vec<char>),");
    }

    for (k, _) in get_var(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    if slash_comment_enable(h)
        || slash_star_comment_enable(h)
        || xml_comment_enable(h)
        || hashtag_comment_enable(h)
    {
        token.push_tabln(1, "COMMENT(Vec<char>),");
    }

    token.push_strln("}\n");
    token.push_strln("pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {");
    token.push_tabln(1, "let id: String = identifier.into_iter().collect();");
    token.push_tabln(1, "match &id[..] {");

    for (_, v) in get_constant(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln("Ok(Token::CONSTANT(identifier.to_vec())),");
    }

    for (k, v) in get_var(h) {
        if v.as_str().unwrap() != "" {
            token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
            token.push_strln(&format!(
                "Ok(Token::{}(identifier.to_vec())),",
                k.as_str().unwrap()
            ));
        }
    }

    for (_, v) in get_entity(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln("Ok(Token::ENTITY(identifier.to_vec())),");
    }

    for (_, v) in get_entity_tag(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln("Ok(Token::ENTITYTAG(identifier.to_vec())),");
    }

    for (_, v) in get_keyword(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln("Ok(Token::KEYWORD(identifier.to_vec())),");
    }

    token.push_tabln(2, "_ => Err(String::from(\"Not a keyword\")),");
    token.push_tabln(1, "}");
    token.push_strln("}");

    token.to_string()
}
