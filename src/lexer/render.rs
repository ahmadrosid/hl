use std::fs;
use crate::lexer;

pub fn parse_file_path(path: &str) -> String {
    let source_code = fs::read(path).expect(&format!("Filed reading file {}", path));
    let input: Vec<char> = source_code.iter().map(|c| *c as char).collect::<Vec<_>>();
    let mut l = lexer::Lexer::new(input);
    l.read_char();

    let mut html = String::new();
    let mut line = 1;
    html.push_str("<table>\n");
    html.push_str("<tbody>\n");
    html.push_str("<tr>");
    html.push_str(&format!("<td data-line=\"{}\"><td><td>", line));
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EOF {
            html.push_str("</td>\n");
            break;
        }
        match token {
            lexer::token::Token::LET => {
                html.push_str(&format!("<span class=\"hl-k\">{}</span>", format!("{:?}", token).to_lowercase()));
            }
            lexer::token::Token::IDENT(value) => {
                html.push_str(&format!("<span class=\"hl-id\">{}</span>", value.iter().collect::<String>()));
            }
            lexer::token::Token::ASSIGN(value) => {
                html.push_str(&format!("<span class=\"hl-op\">{}</span>", value));
            }
            lexer::token::Token::INT(value) => {
                html.push_str(&format!("<span class=\"hl-val\">{}</span>", value.iter().collect::<String>()));
            }
            lexer::token::Token::SEMICOLON(value) => {
                html.push_str(&format!("{}", value));
            }
            lexer::token::Token::IF => {
                html.push_str(&format!("<span class=\"hl-k\">{}</span>", format!("{:?}", token).to_lowercase()));
            }
            lexer::token::Token::LPAREN(value) => {
                html.push_str(&format!("{}", value));
            }
            lexer::token::Token::FUNCTION => {
                html.push_str(&format!("<span class=\"hl-k\">{}</span>", "fn"));
            }
            lexer::token::Token::TRUE => {
                html.push_str(&format!("{}", format!("{:?}", token).to_lowercase()));
            }
            lexer::token::Token::RPAREN(value) => {
                html.push_str(&format!("{}", value));
            }
            lexer::token::Token::LBRACE(value) => {
                html.push_str(&format!("{}", value));
            }
            lexer::token::Token::RBRACE(value) => {
                html.push_str(&format!("{}", value));
            }
            lexer::token::Token::SPACE => {
                html.push_str(&format!("{}", " "));
            }
            lexer::token::Token::ENL => {
                line = line + 1;
                html.push_str("</td></tr>\n");
                html.push_str(&format!("<tr><td data-line=\"{}\"><td><td>", line));
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
