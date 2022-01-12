use crate::generator::{
    double_dash_comment_enable, get_constant, get_entity, get_entity_prefix, get_entity_suffix,
    get_entity_tag, get_keyword, get_var, hashtag_comment_enable, slash_comment_enable,
    slash_star_comment_enable, string::StringBuilder, xml_comment_enable, ConditionExt,
};
use yaml_rust::yaml::Hash;

const ACCEPT_PREFIX_VAR: &str = "ACCEPT_PREFIX_VAR";
const ENCODE_LT: &str = "ENCODE_LT";
const ENCODE_LT_STRING: &str = "ENCODE_LT_STRING";
const RENDER_MULTI_LINE_STRING: &str = "RENDER_MULTI_LINE_STRING";
const ACCEPT_ENTITY_TAG_PREFIX: &str = "ACCEPT_ENTITY_TAG_PREFIX";
const ENTITY_TAG_PREFIX_CHAR: &str = "ENTITY_TAG_PREFIX_CHAR";
const ACCEPT_PREFIX_KEYWORD: &str = "ACCEPT_PREFIX_KEYWORD";
const ACCEPT_STRING_ONE_QUOTE: &str = "ACCEPT_STRING_ONE_QUOTE";
const ACCEPT_STRING_DOUBLE_QUOTE: &str = "ACCEPT_STRING_DOUBLE_QUOTE";
const ACCEPT_STRING_EOF: &str = "ACCEPT_STRING_EOF";
const MARK_ENTITY_TAG_SUFFIX: &str = "MARK_ENTITY_TAG_SUFFIX";
const MARK_STRING_ENTITY_TAG: &str = "MARK_STRING_ENTITY_TAG";
const PREFIX_ONE_LINE_COMMENT: &str = "PREFIX_ONE_LINE_COMMENT";

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

    write_token_integer(&mut html);
    write_token_identifier(&mut html);

    if h.get_some_condition(ACCEPT_STRING_ONE_QUOTE).is_some()
        || h.get_some_condition(ACCEPT_STRING_DOUBLE_QUOTE).is_some()
        || h.get_some_condition(ACCEPT_STRING_EOF).is_some()
    {
        write_token_string(&mut html, h);
    }

    if slash_star_comment_enable(h)
        || slash_comment_enable(h)
        || h.get_some_condition(ACCEPT_ENTITY_TAG_PREFIX).is_some()
        || h.get_some_condition(ENTITY_TAG_PREFIX_CHAR).is_some()
        || h.get_some_condition(ACCEPT_PREFIX_KEYWORD).is_some()
    {
        write_token_ch(&mut html, h);
    }

    if get_entity(h).len() >= 1
        || get_entity_tag(h).len() >= 1
        || get_entity_prefix(h).len() >= 1
        || get_entity_suffix(h).len() >= 1
    {
        write_token_entity(&mut html);
    }

    if get_constant(h).len() >= 1 {
        write_token_constant(&mut html);
    }

    if get_keyword(h).len() >= 1 {
        write_token_keyword(&mut html);
    }

    if get_entity_tag(h).len() >= 1
        || h.get_some_condition(MARK_ENTITY_TAG_SUFFIX).is_some()
        || h.get_some_condition(MARK_STRING_ENTITY_TAG).is_some()
    {
        write_token_entity_tag(&mut html);
    }

    if h.get_some_condition(ACCEPT_PREFIX_VAR).is_some() {
        write_token_var_identifier(&mut html);
    }

    if slash_comment_enable(h)
        || slash_star_comment_enable(h)
        || xml_comment_enable(h)
        || hashtag_comment_enable(h)
        || double_dash_comment_enable(h)
        || h.get_some_condition(PREFIX_ONE_LINE_COMMENT).is_some()
    {
        write_token_comment(&mut html);
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
        r#""<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>","#,
    );
    html.push_tabln(5, "line");
    html.push_tabln(4, "));");
    html.push_tabln(3, "}");

    html.push_tabln(3, "_ => {");
    if let Some(prefix) = h.get_some_condition(ENCODE_LT) {
        if let Some(encode) = h.get_some_condition(ENCODE_LT_STRING) {
            html.push_tabln(4, &format!("if l.ch == '{}' {{", prefix.as_str().unwrap()));
            html.push_tabln(
                5,
                &format!("html.push_str(\"{}\");", encode.as_str().unwrap()),
            );
            html.push_tabln(5, "l.read_char();");
            html.push_tabln(5, "continue;");
            html.push_tabln(4, "}");
        }
    }
    html.push_tabln(4, "html.push(l.ch);");
    html.push_tabln(4, "l.read_char();");
    html.push_tabln(3, "}");
    html.push_tabln(2, "}");
    html.push_tabln(1, "}\n");

    html.push_tabln(1, r#"html.push_str("</tbody>\n");"#);
    html.push_tabln(1, r#"html.push_str("</table>");"#);
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
    html.push_tabln(4, r#"let split = lines.split("\n");"#);
    html.push_tabln(
        4,
        "let split_len = split.clone().collect::<Vec<&str>>().len();",
    );
    html.push_tabln(4, "let mut index = 0;");
    html.push_tabln(4, "for val in split {");
    html.push_tabln(5, "if val.len() > 1 {");
    html.push_tabln(
        6,
        r#"html.push_str(&format!("<span class=\"hl-cmt\">{}</span>", val));"#,
    );
    html.push_tabln(5, "}");
    html.push_tabln(5, "index = index + 1;");
    html.push_tabln(5, "if index != split_len {");
    html.push_tabln(6, "line = line + 1;");
    html.push_tabln(6, "html.push_str(\"</td></tr>\\n\");");
    html.push_tabln(6, "html.push_str(&format!(");
    html.push_tabln(
        7,
        r#""<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>","#,
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

fn write_token_ch(html: &mut StringBuilder, h: &Hash) {
    html.push_tabln(3, "token::Token::CH(value) => {");
    if let Some(prefix) = h.get_some_condition(ENCODE_LT) {
        if let Some(encode) = h.get_some_condition(ENCODE_LT_STRING) {
            html.push_tabln(4, &format!("if value == '{}' {{", prefix.as_str().unwrap()));
            html.push_tabln(
                5,
                &format!("html.push_str(\"{}\");", encode.as_str().unwrap()),
            );
            html.push_tabln(4, "} else {");
            html.push_tabln(5, "html.push(value);");
            html.push_tabln(4, "}");
        } else {
            html.push_tabln(4, "html.push(value);");
        }
    } else {
        html.push_tabln(4, "html.push(value);");
    }
    html.push_tabln(3, "}");
}

fn write_token_string(html: &mut StringBuilder, h: &Hash) {
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
    if h.get_some_condition(ACCEPT_STRING_EOF).is_some() {
        html.push_tabln(
            4,
            r#"s = s.replace("&lt;&lt;","<span class=\"hl-k\">&lt;&lt;</span>");"#,
        );
        html.push_tabln(
            4,
            r#"s = s.replace("EOF","<span class=\"hl-k\">EOF</span>");"#,
        );
    }
    if h.get_some_condition(RENDER_MULTI_LINE_STRING).is_some() {
        html.push_tabln(4, r#"let split = s.split("\n");"#);
        html.push_tabln(
            4,
            "let split_len = split.clone().collect::<Vec<&str>>().len();",
        );
        html.push_tabln(4, "let mut index = 0;");
        html.push_tabln(4, "for val in split {");
        html.push_tabln(
            5,
            r#"html.push_str(&format!("<span class=\"hl-s\">{}</span>", val));"#,
        );
        html.push_tabln(5, "index = index + 1;");
        html.push_tabln(5, "if index != split_len {");
        html.push_tabln(6, "line = line + 1;");
        html.push_tabln(6, "html.push_str(\"</td></tr>\\n\");");
        html.push_tabln(6, "html.push_str(&format!(");
        html.push_tabln(
            7,
            r#""<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>","#,
        );
        html.push_tabln(7, "line");
        html.push_tabln(6, "));");
        html.push_tabln(5, "}");
        html.push_tabln(4, "}");
    } else {
        html.push_tabln(
            4,
            r#"html.push_str(&format!("<span class=\"hl-s\">{}</span>", s));"#,
        );
    }

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
