// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::rust::Lexer;
use crate::lexers::rust::token;

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
			token::Token::IDENT(value) => {
				html.push_str(&value.iter().collect::<String>());
			}
			token::Token::ENTITY(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TRUE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FALSE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::NONE(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CSELF(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::USE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FN(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FOR(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::LOOP(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::LET(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MUT(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IF(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ELSE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::PUB(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MATCH(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MOD(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RETURN(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::WHERE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IMPL(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CHAR(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::STR(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::KSTRING(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VEC(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::KVEC(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CRATE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::REF(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::KSELF(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ASYNC(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::AWAIT(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TYPE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RESULT(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::OPTION(value) => {
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
