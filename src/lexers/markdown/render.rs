// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::markdown::token;
use crate::lexers::markdown::Lexer;

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
                let split = s.split("\n");
                let split_len = split.clone().collect::<Vec<&str>>().len();
                let mut index = 0;
                for val in split {
                    html.push_str(&format!("<span class=\"hl-s\">{}</span>", val));
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
            token::Token::HEAD(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-mh\">{}</span>",
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
            token::Token::ENDL(_) => {
                line = line + 1;
                html.push_str("</td></tr>\n");
                html.push_str(&format!(
                    "<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>",
                    line
                ));
            }
            _ => {
                if l.ch == '<' {
                    html.push_str("&lt;");
                    l.read_char();
                    continue;
                }
                if l.ch == '>' {
                    html.push_str("&gt;");
                    l.read_char();
                    continue;
                }
                html.push(l.ch);
                l.read_char();
            }
        }
    }

    html.push_str("</tbody>\n");
    html.push_str("</table>");
    html
}
