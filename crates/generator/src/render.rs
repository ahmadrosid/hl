use crate::parser::{
    get_constant, get_constant_prefix, get_constant_suffix, get_entity, get_entity_prefix,
    get_entity_suffix, get_entity_tag, get_keyword, get_multi_line_comment, get_var,
    get_var_prefix, get_var_suffix, get_xml_entity_tag, ConditionExt,
};
use crate::string::StringBuilder;
use yaml_rust::yaml::Hash;

const ACCEPT_PREFIX_VAR: &str = "ACCEPT_PREFIX_VAR";
const ENCODE_LT: &str = "ENCODE_LT";
const ENCODE_LT_STRING: &str = "ENCODE_LT_STRING";
const ENCODE_GT: &str = "ENCODE_GT";
const ENCODE_GT_STRING: &str = "ENCODE_GT_STRING";
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
const PREFIX_ONE_LINE_COMMENT_BEFORE_NEWLINE: &str = "PREFIX_ONE_LINE_COMMENT_BEFORE_NEWLINE";
const MARKUP_HEAD: &str = "MARKUP_HEAD";
const IGNORE_INTEGER: &str = "IGNORE_INTEGER";
const MARK_AS_KEYWORD_IN_SCOPE: &str = "MARK_AS_KEYWORD_IN_SCOPE";

pub fn generate_html(h: &Hash) -> String {
    let mut html = StringBuilder::new();
    html.push_strln("\n");
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
    html.push_tabln(2, "if token == Token::EOF {");
    html.push_tabln(3, "html.push_str(\"</td></tr>\\n\");");
    html.push_tabln(3, "break;");
    html.push_tabln(2, "}\n");

    html.push_tabln(2, "match token {");

    if h.check_condition(IGNORE_INTEGER).is_none() {
        html.push_str(include_str!("stub/render/html_token_int.stub"));
    }

    write_token_identifier(&mut html);

    if h.check_condition(ACCEPT_STRING_ONE_QUOTE).is_some()
        || h.check_condition(ACCEPT_STRING_DOUBLE_QUOTE).is_some()
        || h.check_condition(ACCEPT_STRING_EOF).is_some()
    {
        write_token_string(&mut html, h);
    }

    if h.check_condition(ACCEPT_ENTITY_TAG_PREFIX).is_some()
        || h.check_condition(ENTITY_TAG_PREFIX_CHAR).is_some()
        || h.check_condition(ACCEPT_PREFIX_KEYWORD).is_some()
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

    if get_constant(h).len() >= 1
        || get_constant_suffix(h).len() >= 1
        || get_constant_prefix(h).len() >= 1
    {
        html.push_str(include_str!("stub/render/html_token_constant.stub"));
    }

    if get_keyword(h).len() >= 1 || h.check_condition(MARK_AS_KEYWORD_IN_SCOPE).is_some() {
        write_token_keyword(&mut html);
    }

    if get_entity_tag(h).len() >= 1
        || get_xml_entity_tag(h).len() >= 1
        || h.check_condition(MARK_ENTITY_TAG_SUFFIX).is_some()
        || h.check_condition(MARK_STRING_ENTITY_TAG).is_some()
    {
        html.push_str(include_str!("stub/render/html_token_entity_tag.stub"));
    }

    if h.check_condition(ACCEPT_PREFIX_VAR).is_some() {
        html.push_str(include_str!("stub/render/html_token_var.stub"));
    }

    if h.check_condition(MARKUP_HEAD).is_some() {
        write_token_head(&mut html);
    }

    if get_multi_line_comment(h).len() > 1
        || h.check_condition(PREFIX_ONE_LINE_COMMENT).is_some()
        || h.check_condition(PREFIX_ONE_LINE_COMMENT_BEFORE_NEWLINE)
            .is_some()
    {
        html.push_str(include_str!("stub/render/html_token_comment.stub"));
    }

    if get_var(h).len() >= 1 || get_var_suffix(h).len() >= 1 || get_var_prefix(h).len() >= 1 {
        html.push_tabln(3, "Token::VAR(value) => {");
        html.push_tabln(
            4,
            "html.push_str(&format!(\"<span class=\\\"hl-v\\\">{}</span>\", \
                value.iter().collect::<String>()));",
        );
        html.push_tabln(3, "}");
    }

    html.push_str(include_str!("stub/render/html_token_endl.stub"));

    html.push_tabln(3, "_ => {");
    if let Some(prefix) = h.check_condition(ENCODE_LT) {
        if let Some(encode) = h.check_condition(ENCODE_LT_STRING) {
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
    if let Some(prefix) = h.check_condition(ENCODE_GT) {
        if let Some(encode) = h.check_condition(ENCODE_GT_STRING) {
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

fn write_token_head(html: &mut StringBuilder) {
    html.push_tabln(3, "Token::HEAD(value) => {");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-mh\\\">{}</span>\", \
        value.iter().collect::<String>()));",
    );
    html.push_tabln(3, "}");
}

fn write_token_identifier(html: &mut StringBuilder) {
    html.push_tabln(3, "Token::IDENT(value) => {");
    html.push_tabln(4, "html.push_str(&value.iter().collect::<String>());");
    html.push_tabln(3, "}");
}

fn write_token_entity(html: &mut StringBuilder) {
    html.push_tabln(3, "Token::ENTITY(value) => {");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-en\\\">{}</span>\", \
        value.iter().collect::<String>()));",
    );
    html.push_tabln(3, "}");
}

fn write_token_keyword(html: &mut StringBuilder) {
    html.push_tabln(3, "Token::KEYWORD(value) => {");
    html.push_tabln(
        4,
        "html.push_str(&format!(\"<span class=\\\"hl-k\\\">{}</span>\", \
        value.iter().collect::<String>()));",
    );
    html.push_tabln(3, "}");
}

fn write_token_ch(html: &mut StringBuilder, h: &Hash) {
    html.push_tabln(3, "Token::CH(value) => {");
    if let Some(prefix) = h.check_condition(ENCODE_LT) {
        if let Some(encode) = h.check_condition(ENCODE_LT_STRING) {
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
    html.push_tabln(3, "Token::STRING(value) => {");
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
    if h.check_condition(ACCEPT_STRING_EOF).is_some() {
        html.push_tabln(
            4,
            r#"s = s.replace("&lt;&lt;","<span class=\"hl-k\">&lt;&lt;</span>");"#,
        );
        html.push_tabln(
            4,
            r#"s = s.replace("EOF","<span class=\"hl-k\">EOF</span>");"#,
        );
    }
    if h.check_condition(RENDER_MULTI_LINE_STRING).is_some() {
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
