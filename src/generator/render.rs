use yaml_rust::yaml::Hash;
use crate::generator;
use crate::generator::string::StringBuilder;

pub fn generate_render_html(h: &Hash, name: String) -> String {
    let mut html = StringBuilder::new();
    html.push_strln("// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ---- //");
    html.push_strln(&format!("use crate::lexers::{}::Lexer;", name));
    html.push_strln(&format!("use crate::lexers::{}::token;\n", name));

    html.push_strln("pub fn render_html(input: Vec<char>) -> String {");
    html.push_tabln(1, "let mut l = Lexer::new(input);");
    html.push_tabln(1, "l.read_char();");
    html.push_tabln(1, "let mut html = String::new();");
    html.push_tabln(1, "let mut line = 1;");
    html.push_tabln(1, "html.push_str(\"<table class=\\\"highlight-table\\\">\\n\");");
    html.push_tabln(1, "html.push_str(\"<tbody>\\n\");");
    html.push_tabln(1, "html.push_str(\"<tr>\");");
    html.push_tabln(1, "html.push_str(&format!(");
    html.push_tabln(2, "\"<td class=\\\"hl-num\\\" data-line=\\\"{}\\\"></td><td>\",");
    html.push_tabln(2, "line\n\t));\n");

    html.push_tabln(1, "loop {");
    html.push_tabln(2, "let token = l.next_token();");
    html.push_tabln(2, "if token == token::Token::EOF {");
    html.push_tabln(3, "html.push_str(\"</td></tr>\\n\");");
    html.push_tabln(3, "break;");
    html.push_tabln(2, "}\n");

    html.push_tabln(2, "match token {");
    for (k, _v) in generator::get_base(h) {
        let key = k.as_str().unwrap();
        if key != "ENDL" {
            html.push_tabln(3, &format!("token::Token::{}(value) => {{", key));
            html.push_tabln(4, "html.push(value);");
            html.push_tabln(3, "}");
        }
    }

    html.push_tabln(3, "token::Token::CH(value) => {");
    html.push_tabln(4, "html.push(value);");
    html.push_tabln(3, "}");

    if generator::slash_comment_enable(h) {
        html.push_tabln(3, "token::Token::COMMENT(value) => {");
        html.push_tabln(4, "let lines = value.iter().collect::<String>();");
        html.push_tabln(4, "let split = lines.split(\"\\n\");");
        html.push_tabln(4, "let split_len = split.clone().collect::<Vec<&str>>().len();");
        html.push_tabln(4, "let mut index = 0;");
        html.push_tabln(4, "for val in split {");
        html.push_tabln(5, "html.push_str(&format!(\"<span class=\\\"hl-cmt\\\">{}</span>\", val));");
        html.push_tabln(5, "index = index + 1;");
        html.push_tabln(5, "if index != split_len {");
        html.push_tabln(6, "line = line + 1;");
        html.push_tabln(6, "html.push_str(\"</td></tr>\\n\");");
        html.push_tabln(6, "html.push_str(&format!(");
        html.push_tabln(7, "\"<tr><td class=\\\"hl-num\\\" data-line=\\\"{}\\\"></td><td>\",");
        html.push_tabln(7, "line");
        html.push_tabln(6, "));");
        html.push_tabln(5, "}");
        html.push_tabln(4, "}");
        html.push_tabln(3, "}");
    }

    for (k, _v) in generator::get_skip(h) {
        html.push_tabln(3, &format!("token::Token::{}(value) => {{", k.as_str().unwrap()));
        html.push_tabln(4, "html.push_str(&value.iter().collect::<String>());");
        html.push_tabln(3, "}");
    }

    for (k, _v) in generator::get_entity(h) {
        html.push_tabln(3,&format!("token::Token::{}(value) => {{", k.as_str().unwrap()));
        html.push_tabln(4, "html.push_str(&format!(\"<span class=\\\"hl-en\\\">{}</span>\", \
        value.iter().collect::<String>()));");
        html.push_tabln(3, "}");
    }

    for (k, _v) in generator::get_entity_tag(h) {
        html.push_tabln(3,&format!("token::Token::{}(value) => {{", k.as_str().unwrap()));
        html.push_tabln(4, "html.push_str(&format!(\"<span class=\\\"hl-ent\\\">{}</span>\", \
        value.iter().collect::<String>()));");
        html.push_tabln(3, "}");
    }

    for (k, _v) in generator::get_constant(h) {
        html.push_tabln(3,&format!("token::Token::{}(value) => {{", k.as_str().unwrap()));
        html.push_tabln(4, "html.push_str(&format!(\"<span class=\\\"hl-c\\\">{}</span>\", \
        value.iter().collect::<String>()));");
        html.push_tabln(3, "}");
    }

    for (k, _v) in generator::get_var(h) {
        html.push_tabln(3,&format!("token::Token::{}(value) => {{", k.as_str().unwrap()));
        html.push_tabln(4, "html.push_str(&format!(\"<span class=\\\"hl-v\\\">{}</span>\", \
        value.iter().collect::<String>()));");
        html.push_tabln(3, "}");
    }

    for (k, _v) in generator::get_keyword(h) {
        html.push_tabln(3,&format!("token::Token::{}(value) => {{", k.as_str().unwrap()));
        html.push_tabln(4, "html.push_str(&format!(\"<span class=\\\"hl-k\\\">{}</span>\", \
        value.iter().collect::<String>()));");
        html.push_tabln(3, "}");
    }

    html.push_str("\t\t\ttoken::Token::ENDL(_) => {\n\t\t\t\t\
        line = line + 1;\n\t\t\t\t\
        html.push_str(\"</td></tr>\\n\");\n\t\t\t\t\
        html.push_str(&format!(\n\t\t\t\t\t\
            \"<tr><td class=\\\"hl-num\\\" data-line=\\\"{}\\\"></td><td>\",\n\t\t\t\t\t\
            line\n\t\t\t\t\
        ));\n\t\t\t\
    }\n\t\t\t\
    _ => {\n\t\t\t\t\
    html.push(l.ch);\n\t\t\t\t\
    l.read_char();\n\t\t\t\
    }\n");

    html.push_tabln(2, "}");
    html.push_tabln(1, "}\n");

    html.push_tabln(1, "html.push_str(\"</tbody>\\n\");");
    html.push_tabln(1, "html.push_str(\"</table>\");");
    html.push_tabln(1, "html");
    html.push_strln("}");
    html.to_string()
}
