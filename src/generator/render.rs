use yaml_rust::yaml::Hash;
use crate::generator;
use crate::generator::string::StringBuilder;

pub fn generate_render_html(h: &Hash, name: String) -> String {
    let mut html = StringBuilder::new();
    html.push_strln("// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ----");
    html.push_str(&format!("use crate::lexers::{}::Lexer;\n", name));
    html.push_str(&format!("use crate::lexers::{}::token;\n\n", name));
    html.push_str("pub fn render_html(input: Vec<char>) -> String {\n");
    html.push_str("\tlet mut l = Lexer::new(input);\n");
    html.push_str("\tl.read_char();\n");
    html.push_str("\tlet mut html = String::new();\n");
    html.push_str("\tlet mut line = 1;\n");
    html.push_str("\thtml.push_str(\"<table class=\\\"highlight-table\\\">\\n\");\n");
    html.push_str("\thtml.push_str(\"<tbody>\\n\");\n");
    html.push_str("\thtml.push_str(\"<tr>\");\n");
    html.push_str("\thtml.push_str(&format!(\n");
    html.push_str("\t\t\"<td class=\\\"hl-num\\\" data-line=\\\"{}\\\"></td><td>\",\n");
    html.push_str("\t\tline\n\t));\n\n");
    html.push_str("\tloop {\n");
    html.push_str("\t\tlet token = l.next_token();\n");
    html.push_str("\t\tif token == token::Token::EOF {\n");
    html.push_str("\t\t\thtml.push_str(\"</td></tr>\\n\");\n");
    html.push_str("\t\t\tbreak;\n\t\t}\n\n");
    html.push_str("\t\tmatch token {\n");

    for (k, _v) in generator::get_base(h) {
        let key = k.as_str().unwrap();
        if key != "ENDL" {
            html.push_str("\t\t\t");
            html.push_str(&format!("token::Token::{}(value)", key));
            html.push_str(" => {\n");
            html.push_str("\t\t\t\thtml.push(value);\n");
            html.push_str("\t\t\t}\n");
        }
    }

    html.push_str("\t\t\ttoken::Token::CH(value) => {\n");
    html.push_str("\t\t\t\thtml.push(value);\n");
    html.push_str("\t\t\t}\n");

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
        html.push_str("\t\t\t");
        html.push_str(&format!("token::Token::{}(value)", k.as_str().unwrap()));
        html.push_str(" => {\n");
        html.push_str("\t\t\t\thtml.push_str(&value.iter().collect::<String>());\n");
        html.push_str("\t\t\t}\n");
    }

    for (k, _v) in generator::get_entity(h) {
        html.push_str("\t\t\t");
        html.push_str(&format!("token::Token::{}(value)", k.as_str().unwrap()));
        html.push_str(" => {\n");
        html.push_str("\t\t\t\thtml.push_str(&format!(\"<span class=\\\"hl-en\\\">{}</span>\", \
        value.iter().collect::<String>()));\n");
        html.push_str("\t\t\t}\n");
    }

    for (k, _v) in generator::get_entity_tag(h) {
        html.push_str("\t\t\t");
        html.push_str(&format!("token::Token::{}(value)", k.as_str().unwrap()));
        html.push_str(" => {\n");
        html.push_str("\t\t\t\thtml.push_str(&format!(\"<span class=\\\"hl-ent\\\">{}</span>\", \
        value.iter().collect::<String>()));\n");
        html.push_str("\t\t\t}\n");
    }

    for (k, _v) in generator::get_constant(h) {
        html.push_str("\t\t\t");
        html.push_str(&format!("token::Token::{}(value)", k.as_str().unwrap()));
        html.push_str(" => {\n");
        html.push_str("\t\t\t\thtml.push_str(&format!(\"<span class=\\\"hl-c\\\">{}</span>\", \
        value.iter().collect::<String>()));\n");
        html.push_str("\t\t\t}\n");
    }

    for (k, _v) in generator::get_var(h) {
        html.push_str("\t\t\t");
        html.push_str(&format!("token::Token::{}(value)", k.as_str().unwrap()));
        html.push_str(" => {\n");
        html.push_str("\t\t\t\thtml.push_str(&format!(\"<span class=\\\"hl-v\\\">{}</span>\", \
        value.iter().collect::<String>()));\n");
        html.push_str("\t\t\t}\n");
    }

    for (k, _v) in generator::get_keyword(h) {
        html.push_str("\t\t\t");
        html.push_str(&format!("token::Token::{}(value)", k.as_str().unwrap()));
        html.push_str(" => {\n");
        html.push_str("\t\t\t\thtml.push_str(&format!(\"<span class=\\\"hl-k\\\">{}</span>\", \
        value.iter().collect::<String>()));\n");
        html.push_str("\t\t\t}\n");
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

    html.push_str("\t\t}\n");
    html.push_str("\t}\n\n");
    html.push_str("\thtml.push_str(\"</tbody>\\n\");\n");
    html.push_str("\thtml.push_str(\"</table>\");\n");
    html.push_str("\thtml\n");
    html.push_str("}\n");
    html.to_string()
}
