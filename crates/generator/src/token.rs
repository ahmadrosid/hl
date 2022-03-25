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

    for (_, v) in get_var(h) {
        token.push_tab(2, &format!("\"{}\" => ", v.as_str().unwrap()));
        token.push_strln("Ok(Token::VAR(identifier.clone())),");
    }

    write(&get_constant(h), &mut token, "CONSTANT");
    write(&get_entity(h), &mut token, "ENTITY");
    write(&get_keyword(h), &mut token, "KEYWORD");

    let mut entity_tag = get_entity_tag(h);
    entity_tag.extend(get_xml_entity_tag(h));
    write(&entity_tag, &mut token, "ENTITYTAG");

    token.push_tabln(2, "_ => Err(String::from(\"Not a keyword\")),");
    token.push_tabln(1, "}");
    token.push_strln("}");

    token.to_string()
}

fn write(data: &Vec<&str>, token: &mut StringBuilder, key: &str) {
    if !data.is_empty() {
        for (i, v) in data.iter().enumerate() {
            token.push_str(&format!("\"{}\"", v));
            if i < data.len() - 1 {
                token.push_str("|");
            }
        }
        token.push_strln(&" => Ok(Token::{}(identifier.clone())),".replace("{}", key));
    }
}
