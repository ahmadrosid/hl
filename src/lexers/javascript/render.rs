// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::javascript::token;
use crate::lexers::javascript::Lexer;

pub fn render_html(input: Vec<char>) -> String {
    let mut l = Lexer::new(input);
    l.read_char();
    let mut html = String::new();
    let mut line = 1;
    html.push_str("<table class=\"highlight-table\">\n");
    html.push_str("<tbody>\n");
    html.push_str("<tr>");
    html.push_str(&format!(
        "<td class=\"hl-num\" data-line=\"{}\"></td><td>",
        line
    ));

    loop {
        let token = l.next_token();
        if token == token::Token::EOF {
            html.push_str("</td></tr>\n");
            break;
        }

        match token {
            token::Token::INT(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-c\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            token::Token::IDENT(value) => {
                html.push_str(&value.iter().collect::<String>());
            }
            token::Token::STRING(value) => {
                let mut s = String::new();
                for ch in value {
                    if ch == '<' {
                        s.push_str("&lt;");
                    } else if ch == '>' {
                        s.push_str("&gt;");
                    } else {
                        s.push(ch);
                    }
                }
                html.push_str(&format!("<span class=\"hl-s\">{}</span>", s));
            }
            token::Token::CH(value) => {
                html.push(value);
            }
            token::Token::ENTITY(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-en\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            token::Token::CONSTANT(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-c\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            token::Token::KEYWORD(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-k\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            token::Token::COMMENT(value) => {
                let mut lines = String::new();
                for ch in value {
                    if ch == '<' {
                        lines.push_str("&lt;");
                    } else if ch == '>' {
                        lines.push_str("&gt;");
                    } else {
                        lines.push(ch);
                    }
                }
                let split = lines.split("\n");
                let split_len = split.clone().collect::<Vec<&str>>().len();
                let mut index = 0;
                for val in split {
                    if val.len() > 1 {
                        html.push_str(&format!("<span class=\"hl-cmt\">{}</span>", val));
                    }
                    index = index + 1;
                    if index != split_len {
                        line = line + 1;
                        html.push_str("</td></tr>\n");
                        html.push_str(&format!(
                            "<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>",
                            line
                        ));
                    }
                }
            }
            token::Token::VINFINITY(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-v\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            token::Token::VNAN(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-v\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            token::Token::MATH(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-v\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            token::Token::DATE(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-v\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            token::Token::ENDL(_) => {
                line = line + 1;
                html.push_str("</td></tr>\n");
                html.push_str(&format!(
                    "<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>",
                    line
                ));
            }
            _ => {
                html.push(l.ch);
                l.read_char();
            }
        }
    }

    html.push_str("</tbody>\n");
    html.push_str("</table>");
    html
}
