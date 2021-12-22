use crate::rust;
use std::fs;

pub fn parse_file_path(path: &str) -> String {
    let source_code = fs::read(path).expect(&format!("Filed reading file {}", path));
    let input: Vec<char> = source_code.iter().map(|c| *c as char).collect::<Vec<_>>();
    let len = input.len();
    let mut l = rust::Lexer::new(input);
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
        if token == rust::token::Token::EOF('0') {
            html.push_str("</td></tr>\n");
            break;
        }

        if token == rust::token::Token::ILLEGAL {
            println!("Illegal token position: {} char: {}", l.position, l.ch);
            break;
        }

        match token {
            rust::token::Token::LET => {
                html.push_str(&format!(
                    "<span class=\"hl-k\">{}</span>",
                    format!("{:?}", token).to_lowercase()
                ));
            }
            rust::token::Token::IDENT(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-id\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            rust::token::Token::ASSIGN(value) => {
                html.push_str(&format!("<span class=\"hl-op\">{}</span>", value));
            }
            rust::token::Token::INT(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-val\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            rust::token::Token::SEMICOLON(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::COLON(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::COMMA(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::AND(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::BANG(value) => {
                html.push_str(&format!("{}", value));
            }
            // rust::token::Token::PATHSEPARATOR(value) => {
            //     html.push_str(&format!("{}", value.iter().collect::<String>()));
            // }
            rust::token::Token::MEMBERACCESS(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::STRING(value) => {
                html.push_str(&format!(
                    "<span class=\"hl-s\">{}</span>",
                    value.iter().collect::<String>()
                ));
            }
            rust::token::Token::IF => {
                html.push_str(&format!(
                    "<span class=\"hl-k\">{}</span>",
                    format!("{:?}", token).to_lowercase()
                ));
            }
            rust::token::Token::LPAREN(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::FN => {
                html.push_str(&format!("<span class=\"hl-k\">{}</span>", "fn"));
            }
            rust::token::Token::TRUE(value) => {
                html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
            }
            rust::token::Token::RPAREN(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::LBRACE(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::RBRACE(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::SPACE(value) => {
                html.push_str(&format!("{}", value));
            }
            rust::token::Token::TAB(_) => {
                html.push_str(&format!("{}", "    "));
            }
            rust::token::Token::ENDL(_) => {
                line = line + 1;
                html.push_str("</td></tr>\n");
                html.push_str(&format!(
                    "<tr><td class=\"hl-num\" data-line=\"{}\"></td><td>",
                    line
                ));
            }
            _ => {
                println!("{:?}", token);
            }
        }
    }

    html.push_str("</tbody>\n");
    html.push_str("</table>");
    html
}
