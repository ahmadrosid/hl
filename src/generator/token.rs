use yaml_rust::yaml::Hash;
use crate::generator;
use crate::generator::string::StringBuilder;

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

    for (k, _v) in generator::get_base(h) {
        token.push_tabln(1, &format!("{}(char),", k.as_str().unwrap()));
    }

    for (k, _v) in generator::get_constant(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in generator::get_var(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in generator::get_skip(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in generator::get_entity(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in generator::get_entity_tag(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    for (k, _v) in generator::get_keyword(h) {
        token.push_tabln(1, &format!("{}(Vec<char>),", k.as_str().unwrap()));
    }

    if generator::slash_comment_enable(h) {
        token.push_tabln(1, "COMMENT(Vec<char>),");
    }

    token.push_strln("}\n");
    token.push_strln("pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {");
    token.push_tabln(1, "let identifiers: String = identifier.into_iter().collect();");
    token.push_tabln(1, "match &identifiers[..] {");

    for (k, v) in generator::get_constant(h) {
        if v.as_str().unwrap() != "" {
            token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
            token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
        }
    }

    for (k, v) in generator::get_var(h) {
        if v.as_str().unwrap() != "" {
            token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
            token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
        }
    }

    for (k, v) in generator::get_entity(h) {
        if v.as_str().unwrap() != "" {
            token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
            token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
        }
    }
    for (k, v) in generator::get_entity_tag(h) {
        token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
        token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
    }

    for (k, v) in generator::get_keyword(h) {
        token.push_str(&format!("\t\t\"{}\" => ", v.as_str().unwrap()));
        token.push_str(&format!("Ok(Token::{}(identifier.to_vec())),\n", k.as_str().unwrap()));
    }

    token.push_str("\t\t_ => Err(String::from(\"Not a keyword\"))\n");
    token.push_str("\t}\n");
    token.push_str("}\n");

    token.to_string()
}
