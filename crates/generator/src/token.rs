use crate::parser::{
    get_constant, get_entity, get_entity_tag, get_keyword, get_var, get_xml_entity_tag,
};

use yaml_rust::yaml::Hash;

pub fn generate_token(h: &Hash) -> String {
    let mut token = String::new();
    token.push_str("\npub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {");
    token.push_str("let id: String = identifier.into_iter().collect();");
    token.push_str("match &id[..] {\n");

    write(&get_constant(h), &mut token, "CONSTANT");
    write(&get_var(h), &mut token, "VAR");
    write(&get_entity(h), &mut token, "ENTITY");
    write(&get_keyword(h), &mut token, "KEYWORD");

    let mut entity_tag = get_entity_tag(h);
    entity_tag.extend(get_xml_entity_tag(h));
    write(&entity_tag, &mut token, "ENTITYTAG");

    token.push_str("_ => Err(String::from(\"Not a keyword\")),");
    token.push_str("}");
    token.push_str("}");

    token.to_string()
}

fn write(data: &Vec<&str>, token: &mut String, key: &str) {
    if !data.is_empty() {
        for (i, v) in data.iter().enumerate() {
            token.push_str(&format!("\"{}\"", v));
            if i < data.len() - 1 {
                token.push_str("|");
            }
        }
        token.push_str(&" => Ok(Token::{}(identifier.clone())),".replace("{}", key));
    }
}
