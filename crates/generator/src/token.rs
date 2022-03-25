use crate::parser::{
    get_constant, get_entity, get_entity_tag, get_keyword, get_var, get_xml_entity_tag,
};

use crate::string::StringBuilder;
use yaml_rust::yaml::Hash;

pub fn generate_token(h: &Hash) -> String {
    let mut token = StringBuilder::new();
    token.push_strln("// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //");
    token.push_strln("use crate::lexers::Token;\n");
    token.push_strln("pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {");
    token.push_tabln(1, "let id: String = identifier.into_iter().collect();");
    token.push_tabln(1, "match &id[..] {");

    for v in get_constant(h) {
        token.push_tab(2, &format!("\"{}\" => ", v));
        token.push_strln("Ok(Token::CONSTANT(identifier.clone())),");
    }

    for (_, v) in get_var(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln("Ok(Token::VAR(identifier.clone())),");
    }

    for v in get_entity(h) {
        token.push_tab(2, &format!("\"{}\" => ", v));
        token.push_strln("Ok(Token::ENTITY(identifier.clone())),");
    }

    for (_, v) in get_entity_tag(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln("Ok(Token::ENTITYTAG(identifier.clone())),");
    }

    for v in get_xml_entity_tag(h) {
        token.push_tab(2, &format!("\"{}\" => ", v));
        token.push_strln("Ok(Token::ENTITYTAG(identifier.clone())),");
    }

    let keyword = get_keyword(h);
    if !keyword.is_empty() {
        for (i, v) in keyword.iter().enumerate() {
            token.push_str(&format!("\"{}\"", v));
            if i < keyword.len() - 1 {
                token.push_str("|");
            }
        }
        token.push_strln(" => Ok(Token::KEYWORD(identifier.clone())),");
    }

    token.push_tabln(2, "_ => Err(String::from(\"Not a keyword\")),");
    token.push_tabln(1, "}");
    token.push_strln("}");

    token.to_string()
}
