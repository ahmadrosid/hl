// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::go::Lexer;
use crate::lexers::go::token;

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
			token::Token::CH(value) => {
				html.push(value);
			}
			token::Token::STRING(value) => {
				let mut s = String::new();
				for ch in value {
					if ch == '<' {
						s.push_str("&lt;");
					} else if ch == '<' {
						s.push_str("&gt;");
					} else {
						s.push(ch);
					}
				}
				html.push_str(&format!("<span class=\"hl-s\">{}</span>", s));
			}
			token::Token::INT(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IDENT(value) => {
				html.push_str(&value.iter().collect::<String>());
			}
			token::Token::ENTITY(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::KEYWORD(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::COMMENT(value) => {
				let lines = value.iter().collect::<String>();
				let split = lines.split("\n");
				let split_len = split.clone().collect::<Vec<&str>>().len();
				let mut index = 0;
				for val in split {
					html.push_str(&format!("<span class=\"hl-cmt\">{}</span>", val));
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
			token::Token::ARGS(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TRUE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FALSE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::NIL(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::BOOL(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CINT(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::UINT(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::BYTE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CSTRING(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ERROR(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RUNE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::UINTPTR(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FLOAT(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::PACKAGE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
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
			token::Token::TYPE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::INTERFACE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CHAN(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::STRUCT(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CONST(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RANGE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::BREAK(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MAP(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CONTINUE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
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
