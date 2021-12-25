use yaml_rust::yaml::Hash;
use crate::generator::{
    string::StringBuilder,
    get_base,
    get_constant,
    get_entity,
    get_entity_tag,
    get_var,
    get_skip,
    get_keyword,
    slash_comment_enable
};

pub fn generate_token(h: &Hash) -> String {
    let mut token = StringBuilder::new();
    token.push_strln("// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ---- //");
    token.push_strln("#[derive(PartialEq)]");
    token.push_strln("#[derive(Debug)]");
    token.push_strln("pub enum Token {");
    token.push_tabln(1, "ILLEGAL,");
    token.push_tabln(1, "EOF,");
    token.push_tabln(1, "CH(char),");
    token.push_tabln(1, "ENDL(char),");

    for (k, _v) in get_base(h) {
        token.push_tabln(1, &format!("{}(char),", k.as_str().unwrap()));
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

    for (k, _v) in get_keyword(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    if slash_comment_enable(h) {
        token.push_tabln(1, "COMMENT(Vec<char>),");
    }

    token.push_strln("}\n");
    token.push_strln("pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {");
    token.push_tabln(1, "let identifiers: String = identifier.into_iter().collect();");
    token.push_tabln(1, "match &identifiers[..] {");

    for (k, v) in get_constant(h) {
        if v.as_str().unwrap() != "" {
            token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
            token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
        }
    }

    for (k, v) in get_var(h) {
        if v.as_str().unwrap() != "" {
            token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
            token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
        }
    }

    for (k, v) in get_entity(h) {
        if v.as_str().unwrap() != "" {
            token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
            token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
        }
    }
    for (k, v) in get_entity_tag(h) {
        token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
        token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
    }

    for (k, v) in get_keyword(h) {
        token.push_tab(2,&format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln(&format!("Ok(Token::{}(identifier.to_vec())),", k.as_str().unwrap()));
    }

    token.push_tabln(2, "_ => Err(String::from(\"Not a keyword\"))");
    token.push_tabln(1, "}");
    token.push_strln("}");

    token.to_string()
}
