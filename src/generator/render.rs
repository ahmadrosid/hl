use crate::generator::{get_condition, get_entity, get_keyword, get_entity_tag, get_var, slash_comment_enable, slash_star_comment_enable, string::StringBuilder, xml_comment_enable};
use yaml_rust::yaml::Hash;
use yaml_rust::yaml::Yaml;

const ACCEPT_PREFIX_VAR: &str = "ACCEPT_PREFIX_VAR";
const ENCODE_LT: &str = "ENCODE_LT";
const ENCODE_LT_STRING: &str = "ENCODE_LT_STRING";

pub fn generate_render_html(h: &Hash, name: String) -> String {
    let mut html = StringBuilder::new();
    html.push_strln("// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //");
    html.push_strln(&format!("use crate::lexers::{}::Lexer;", name));
    html.push_strln(&format!("use crate::lexers::{}::token;\n", name));

    html.push_strln("pub fn render_html(input: Vec<char>) -> String {");
    html.push_tabln(1, "let mut l = Lexer::new(input);");
    html.push_tabln(1, "l.read_char();");
    html.push_tabln(1, "let mut html = String::new();");
    html.push_tabln(1, "let mut line = 1;");
    html.push_tabln(
        1,
        "html.push_str(\"<table class=\\\"highlight-table\\\">\\n\");",
    );
    html.push_tabln(1, "html.push_str(\"<tbody>\\n\");");
    html.push_tabln(1, "html.push_str(\"<tr>\");");
    html.push_tabln(1, "html.push_str(&format!(");
    html.push_tabln(
        2,
        "\"<td class=\\\"hl-num\\\" data-line=\\\"{}\\\"></td><td>\",",
    );
    html.push_tabln(2, "line");
    html.push_tabln(1, "));\n");

    html.push_tabln(1, "loop {");
    html.push_tabln(2, "let token = l.next_token();");
    html.push_tabln(2, "if token == token::Token::EOF {");
    html.push_tabln(3, "html.push_str(\"</td></tr>\\n\");");
    html.push_tabln(3, "break;");
    html.push_tabln(2, "}\n");

    html.push_tabln(2, "match token {");
    html.push_tabln(3, "token::Token::CH(value) => {");

    if let Some(prefix) = get_condition(h).get(&Yaml::String(ENCODE_LT.to_string())) {
        if let Some(encode) = get_condition(h).get(&Yaml::String(ENCODE_LT_STRING.to_string())) {
            html.push_tabln(4, &format!("if value == '{}' {{", prefix.as_str().unwrap()));
            html.push_tabln(
                5,
                &format!("html.push_str(\"{}\");", encode.as_str().unwrap()),
            );
            html.push_tabln(4, "} else {");
            html.push_tabln(4, "html.push(value);");
            html.push_tabln(4, "}");
        } else {
            html.push_tabln(4, "html.push(value);");
        }
    } else {
        html.push_tabln(4, "html.push(value);");
    }
    html.push_tabln(3, "}");

    write_token_string(&mut html);
    write_token_integer(&mut html);
    write_token_identifier(&mut html);
    write_token_entity(&mut html);
    write_token_constant(&mut html);

    if get_keyword(h).len() >= 1 {
        write_token_keyword(&mut html);
    }

    if get_entity_tag(h).len() >= 1 {
        write_token_entity_tag(&mut html);
    }

    if let Some(_) = get_condition(h).get(&Yaml::String(ACCEPT_PREFIX_VAR.to_string())) {
        write_token_var_identifier(&mut html);
    }

    if slash_comment_enable(h) || slash_star_comment_enable(h) || xml_comment_enable(h) {
        write_token_comment(&mut html);
    }

    for (k, _v) in get_entity(h) {
        html.push_tabln(
            3,
            &format!("token::Token::{}(value) => {{", k.as_str().unwrap()),
        );
        html.push_tabln(
            4,
            "html.push_str(&format!(\"<span class=\\\"hl-en\\\">{}</span>\", \
                value.iter().collect::<String>()));",
        );
        html.push_tabln(3, "}");
    }

    for (k, _v) in get_var(h) {
        html.push_tabln(
            3,
            &format!("token::Token::{}(value) => {{", k.as_str().unwrap()),
        );
        html.push_tabln(
            4,
            "html.push_str(&format!(\"<span class=\\\"hl-v\\\">{}</span>\", \
                value.iter().collect::<String>()));",
        );
        html.push_tabln(3, "}");
    }

    html.push_tabln(3, "token::Token::ENDL(_) => {");
    html.push_tabln(4, "line = line + 1;");
    html.push_tabln(4, "html.push_str(\"</td></tr>\\n\");");
    html.push_tabln(4, "html.push_str(&format!(");
    html.push_tabln(
        5,
        "\"<tr><td class=\\\"hl-num\\\" data-line=\\\"{}\\\"></td><td>\",",
    );
    html.push_tabln(5, "line");
    html.push_tabln(4, "));");
    html.push_tabln(3, "}");

    html.push_tabln(3, "_ => {");
    html.push_tabln(4, "html.push(l.ch);");
    html.push_tabln(4, "l.read_char();");
    html.push_tabln(3, "}");
    html.push_tabln(2, "}");
    html.push_tabln(1, "}\n");

    html.push_tabln(1, "html.push_str(\"</tbody>\\n\");");
    html.push_tabln(1, "html.push_str(\"</table>\");");
    html.push_tabln(1, "html");
    html.push_strln("}");
    html.to_string()
}

fn write_token_comment(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::COMMENT(value) => {");
    html.push_tabln(4, "let mut lines = String::new();");
    html.push_tabln(4, "for ch in value {");
    html.push_tabln(5, "if ch == '<' {");
    html.push_tabln(6, "lines.push_str(\"&lt;\");");
    html.push_tabln(5, "} else if ch == '>' {");
    html.push_tabln(6, "lines.push_str(\"&gt;\");");
    html.push_tabln(5, "} else {");
    html.push_tabln(6, "lines.push(ch);");
    html.push_tabln(5, "}");
    html.push_tabln(4, "}");
    html.push_tabln(4, "let split = lines.split(\"\\n\");");
    html.push_tabln(
        4,
        "let split_len = split.clone().collect::<Vec<&str>>().len();",
    );
    html.push_tabln(4, "let mut index = 0;");
    html.push_tabln(4, "for val in split {");
    html.push_tabln(
        5,
        "html.push_str(&format!(\"<span class=\\\"hl-cmt\\\">{}</span>\", val));",
    );
    html.push_tabln(5, "index = index + 1;");
    html.push_tabln(5, "if index != split_len {");
    html.push_tabln(6, "line = line + 1;");
    html.push_tabln(6, "html.push_str(\"</td></tr>\\n\");");
    html.push_tabln(6, "html.push_str(&format!(");
    html.push_tabln(
        7,
        "\"<tr><td class=\\\"hl-num\\\" data-line=\\\"{}\\\"></td><td>\",",
    );
    html.push_tabln(7, "line");
    html.push_tabln(6, "));");
    html.push_tabln(5, "}");
    html.push_tabln(4, "}");
    html.push_tabln(3, "}");
}

fn write_token_identifier(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::IDENT(value) => {");
    html.push_tabln(4, "html.push_str(&value.iter().collect::<String>());");
    html.push_tabln(3, "}");
}

fn write_token_entity(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::ENTITY(value) => {");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-en\\\">{}</span>\", \
        value.iter().collect::<String>()));",
    );
    html.push_tabln(3, "}");
}

fn write_token_keyword(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::KEYWORD(value) => {");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-k\\\">{}</span>\", \
        value.iter().collect::<String>()));",
    );
    html.push_tabln(3, "}");
}

fn write_token_var_identifier(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::VAR(value) => {");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-vid\\\">{}</span>\", \
        value.iter().collect::<String>()));",
    );
    html.push_tabln(3, "}");
}

fn write_token_constant(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::CONSTANT(value) => {");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-c\\\">{}</span>\", \
        value.iter().collect::<String>()));",
    );
    html.push_tabln(3, "}");
}

fn write_token_string(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::STRING(value) => {");
    html.push_tabln(4, "let mut s = String::new();");
    html.push_tabln(4, "for ch in value {");
    html.push_tabln(5, "if ch == '<' {");
    html.push_tabln(6, "s.push_str(\"&lt;\");");
    html.push_tabln(5, "} else if ch == '>' {");
    html.push_tabln(6, "s.push_str(\"&gt;\");");
    html.push_tabln(5, "} else {");
    html.push_tabln(6, "s.push(ch);");
    html.push_tabln(5, "}");
    html.push_tabln(4, "}");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-s\\\">{}</span>\", s));",
    );
    html.push_tabln(3, "}");
}

fn write_token_entity_tag(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::ENTITYTAG(value) => {");
    html.push_tabln(4, "let mut s = String::new();");
    html.push_tabln(4, "for ch in value {");
    html.push_tabln(5, "if ch == '<' {");
    html.push_tabln(6, "s.push_str(\"&lt;\");");
    html.push_tabln(5, "} else if ch == '>' {");
    html.push_tabln(6, "s.push_str(\"&gt;\");");
    html.push_tabln(5, "} else {");
    html.push_tabln(6, "s.push(ch);");
    html.push_tabln(5, "}");
    html.push_tabln(4, "}");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-ent\\\">{}</span>\", s));",
    );
    html.push_tabln(3, "}");
}

fn write_token_integer(html: &mut StringBuilder) {
    html.push_tabln(3, "token::Token::INT(value) => {");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-c\\\">{}</span>\", \
        value.iter().collect::<String>()));",
    );
    html.push_tabln(3, "}");
}
