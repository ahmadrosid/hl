use yaml_rust::yaml::Hash;
use crate::generator;

pub fn generate_html(h: &Hash, name: String) -> String {
    let mut html = String::new();
    html.push_str(&format!("use crate::{};\n", name));
    html.push_str(&format!("use crate::{}::token;\n", name));
    html.push_str("use std::fs::read;\n\n");
    html.push_str("pub fn render_html(path: &str) -> String {\n");
    html.push_str("\tlet source = read(path).expect(&format!(\"Filed reading file {}\", path));\n");
    html.push_str("\tlet input: Vec<char> = source.iter().map(|c| *c as char).collect::<Vec<_>>();\n");
    html.push_str(&format!("\tlet mut l = {}::Lexer::new(input);\n", name));
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
    html.push_str("\t\tif token == rust::token::Token::EOF('0') {\n");
    html.push_str("\t\t\thtml.push_str(\"</td></tr>\\n\");\n");
    html.push_str("\t\t\tbreak;\n\t\t}\n\n");
    html.push_str("\t\tif token == rust::token::Token::ILLEGAL {\n");
    html.push_str("\t\t\tprintln!(\"Illegal token idx: {} char: {}\", l.position, l.ch);\n");
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

    for (k, _v) in generator::get_skip(h) {
        html.push_str("\t\t\t");
        html.push_str(&format!("token::Token::{}(value)", k.as_str().unwrap()));
        html.push_str(" => {\n");
        html.push_str("\t\t\t\thtml.push_str(&value.iter().collect::<String>());\n");
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

    for (k, _v) in generator::get_keyword(h) {
        html.push_str("\t\t\t");
        html.push_str(&format!("token::Token::{}(value)", k.as_str().unwrap()));
        html.push_str(" => {\n");
        html.push_str("\t\t\t\thtml.push_str(&format!(\"<span class=\\\"hl-k\\\">{}</span>\", \
        value.iter().collect::<String>()));\n");
        html.push_str("\t\t\t}\n");
    }


    html.push_str("\t\t\t\
    rust::token::Token::ENDL(_) => {\n\t\t\t\t\
        line = line + 1;\n\t\t\t\t\
        html.push_str(\"</td></tr>\\n\");\n\t\t\t\t\
        html.push_str(&format!(\n\t\t\t\t\t\
            \"<tr><td class=\\\"hl-num\\\" data-line=\\\"{}\\\"></td><td>\",\n\t\t\t\t\t\
            line\n\t\t\t\t\
        ));\n\t\t\t\
    }\n\t\t\t\
    _ => {\n\t\t\t\t\
    println!(\"{:?}\", token);\n\t\t\t\
    }\n");

    html.push_str("\t\t}\n");
    html.push_str("\t}\n\n");
    html.push_str("\thtml.push_str(\"</tbody>\\n\");\n");
    html.push_str("\thtml.push_str(\"</table>\");\n");
    html.push_str("\thtml\n");
    html.push_str("}\n");

    html
}
