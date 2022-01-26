use crate::generator::{
    double_dash_comment_enable, get_constant, get_entity, get_entity_prefix, get_entity_suffix,
    get_entity_tag, get_keyword, get_var, get_xml_entity_tag, hashtag_comment_enable,
    slash_comment_enable, slash_star_comment_enable, string::StringBuilder, xml_comment_enable,
    ConditionExt,
};
use yaml_rust::yaml::Hash;

const ACCEPT_PREFIX_VAR: &str = "ACCEPT_PREFIX_VAR";
const ACCEPT_ENTITY_TAG_PREFIX: &str = "ACCEPT_ENTITY_TAG_PREFIX";
const ENTITY_TAG_PREFIX_CHAR: &str = "ENTITY_TAG_PREFIX_CHAR";
const ACCEPT_PREFIX_KEYWORD: &str = "ACCEPT_PREFIX_KEYWORD";
const ACCEPT_STRING_ONE_QUOTE: &str = "ACCEPT_STRING_ONE_QUOTE";
const ACCEPT_STRING_DOUBLE_QUOTE: &str = "ACCEPT_STRING_DOUBLE_QUOTE";
const ACCEPT_STRING_EOF: &str = "ACCEPT_STRING_EOF";
const MARK_ENTITY_TAG_SUFFIX: &str = "MARK_ENTITY_TAG_SUFFIX";
const MARK_STRING_ENTITY_TAG: &str = "MARK_STRING_ENTITY_TAG";
const PREFIX_ONE_LINE_COMMENT: &str = "PREFIX_ONE_LINE_COMMENT";
const MARKUP_HEAD: &str = "MARKUP_HEAD";

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

    if h.get_some_condition(MARKUP_HEAD).is_some() {
        token.push_tabln(1, "HEAD(Vec<char>),")
    }

    if slash_star_comment_enable(h)
        || slash_comment_enable(h)
        || h.get_some_condition(ACCEPT_ENTITY_TAG_PREFIX).is_some()
        || h.get_some_condition(ENTITY_TAG_PREFIX_CHAR).is_some()
        || h.get_some_condition(ACCEPT_PREFIX_KEYWORD).is_some()
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

    if h.get_some_condition(ACCEPT_STRING_ONE_QUOTE).is_some()
        || h.get_some_condition(ACCEPT_STRING_DOUBLE_QUOTE).is_some()
        || h.get_some_condition(ACCEPT_STRING_EOF).is_some()
    {
        token.push_tabln(1, "STRING(Vec<char>),");
    }

    if get_constant(h).len() >= 1 {
        token.push_tabln(1, "CONSTANT(Vec<char>),");
    }

    if get_keyword(h).len() >= 1 {
        token.push_tabln(1, "KEYWORD(Vec<char>),");
    }

    if get_entity_tag(h).len() >= 1
        || get_xml_entity_tag(h).len() >= 1
        || h.get_some_condition(MARK_ENTITY_TAG_SUFFIX).is_some()
        || h.get_some_condition(MARK_STRING_ENTITY_TAG).is_some()
    {
        token.push_tabln(1, "ENTITYTAG(Vec<char>),");
    }

    if h.get_some_condition(ACCEPT_PREFIX_VAR).is_some() {
        token.push_tabln(1, "VAR(Vec<char>),");
    }

    for (k, _) in get_var(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    if slash_comment_enable(h)
        || slash_star_comment_enable(h)
        || xml_comment_enable(h)
        || hashtag_comment_enable(h)
        || double_dash_comment_enable(h)
        || h.get_some_condition(PREFIX_ONE_LINE_COMMENT).is_some()
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

    for (_, v) in get_xml_entity_tag(h) {
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
