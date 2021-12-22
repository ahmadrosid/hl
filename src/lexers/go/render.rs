use crate::go;
use crate::go::token;
use std::fs::read;

pub fn render_html(path: &str) -> String {
	let source = read(path).expect(&format!("Filed reading file {}", path));
	let input: Vec<char> = source.iter().map(|c| *c as char).collect::<Vec<_>>();
	let mut l = go::Lexer::new(input);
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
		if token == go::token::Token::EOF('0') {
			html.push_str("</td></tr>\n");
			break;
		}

		if token == go::token::Token::ILLEGAL {
			println!("Illegal token idx: {} char: {}", l.position, l.ch);
			break;
		}

		match token {
			token::Token::PLUS(value) => {
				html.push(value);
			}
			token::Token::MINUS(value) => {
				html.push(value);
			}
			token::Token::LPAREN(value) => {
				html.push(value);
			}
			token::Token::RPAREN(value) => {
				html.push(value);
			}
			token::Token::LBRACE(value) => {
				html.push(value);
			}
			token::Token::RBRACE(value) => {
				html.push(value);
			}
			token::Token::SPACE(value) => {
				html.push(value);
			}
			token::Token::ASSIGN(value) => {
				html.push(value);
			}
			token::Token::COLON(value) => {
				html.push(value);
			}
			token::Token::SEMICOLON(value) => {
				html.push(value);
			}
			token::Token::MEMBERACCESS(value) => {
				html.push(value);
			}
			token::Token::COMMA(value) => {
				html.push(value);
			}
			token::Token::SLASH(value) => {
				html.push(value);
			}
			token::Token::GT(value) => {
				html.push(value);
			}
			token::Token::LT(value) => {
				html.push(value);
			}
			token::Token::AND(value) => {
				html.push(value);
			}
			token::Token::BANG(value) => {
				html.push(value);
			}
			token::Token::ASTERISK(value) => {
				html.push(value);
			}
			token::Token::TAB(value) => {
				html.push(value);
			}
			token::Token::EOF(value) => {
				html.push(value);
			}
			token::Token::IDENT(value) => {
				html.push_str(&value.iter().collect::<String>());
			}
			token::Token::ENTITY(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ENTITY_TAG(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TRUE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FALSE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::STRING(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::INT(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FUNC(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FOR(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VAR(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IF(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ELSE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MATCH(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CASE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RETURN(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DEFAULT(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IMPORT(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			go::token::Token::ENDL(_) => {
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
