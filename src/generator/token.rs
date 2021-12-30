use yaml_rust::Yaml;
use crate::generator::{get_condition, get_constant, get_entity, get_entity_tag, get_keyword, get_skip, get_var, slash_comment_enable, string::StringBuilder};
use yaml_rust::yaml::Hash;

const ACCEPT_ENTITY_TAG_PREFIX: &str = "ACCEPT_ENTITY_TAG_PREFIX";

pub fn generate_token(h: &Hash) -> String {
    let mut token = StringBuilder::new();
    token.push_strln("// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //");
    token.push_strln("#[derive(PartialEq)]");
    token.push_strln("#[derive(Debug)]");
    token.push_strln("pub enum Token {");
    token.push_tabln(1, "ILLEGAL,");
    token.push_tabln(1, "EOF,");
    token.push_tabln(1, "CH(char),");
    token.push_tabln(1, "ENDL(char),");
    token.push_tabln(1, "INT(Vec<char>),");
    token.push_tabln(1, "IDENT(Vec<char>),");
    token.push_tabln(1, "ENTITY(Vec<char>),");
    token.push_tabln(1, "STRING(Vec<char>),");
    token.push_tabln(1, "KEYWORD(Vec<char>),");

    if let Some(_) = get_condition(h).get(&Yaml::String(ACCEPT_ENTITY_TAG_PREFIX.to_string())) {
        token.push_tabln(1, "ENTITYTAG(Vec<char>),");
    }

    for (k, _v) in get_constant(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in get_var(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in get_skip(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in get_entity(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in get_entity_tag(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    if slash_comment_enable(h) {
        token.push_tabln(1, "COMMENT(Vec<char>),");
    }

    token.push_strln("}\n");
    token.push_strln("pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {");
    token.push_tabln(
        1,
        "let identifiers: String = identifier.into_iter().collect();",
    );
    token.push_tabln(1, "match &identifiers[..] {");

    for (k, v) in get_constant(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln(&format!(
            "Ok(Token::{}(identifier.to_vec())),",
            k.as_str().unwrap()
        ));
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

    for (k, v) in get_entity(h) {
        if v.as_str().unwrap() != "" {
            token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
            token.push_strln(&format!(
                "Ok(Token::{}(identifier.to_vec())),",
                k.as_str().unwrap()
            ));
        }
    }
    for (k, v) in get_entity_tag(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln(&format!(
            "Ok(Token::{}(identifier.to_vec())),",
            k.as_str().unwrap()
        ));
    }

    for (_k, v) in get_keyword(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln("Ok(Token::KEYWORD(identifier.to_vec())),");
    }

    token.push_tabln(2, "_ => Err(String::from(\"Not a keyword\"))");
    token.push_tabln(1, "}");
    token.push_strln("}");

    token.to_string()
}
