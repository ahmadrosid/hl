use crate::lexers::javascript::Lexer;
use crate::lexers::javascript::token;

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
			token::Token::CH(value) => {
				html.push(value);
			}
			token::Token::COMMENT(value) => {
				html.push_str(&format!("<span class=\"hl-cmt\">{}</span>", value.iter().collect::<String>()));
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
			token::Token::UNDEFINED(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::NULL(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::STRING(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::INT(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VINFINITY(value) => {
				html.push_str(&format!("<span class=\"hl-v\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VNAN(value) => {
				html.push_str(&format!("<span class=\"hl-v\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MATH(value) => {
				html.push_str(&format!("<span class=\"hl-v\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DATE(value) => {
				html.push_str(&format!("<span class=\"hl-v\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IMPORT(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::AS(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::NEW(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::THIS(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CLASS(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VAR(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CONST(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::LET(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FUNCTION(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FOR(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IF(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ELSE(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VOID(value) => {
				html.push_str(&format!("<span class=\"hl-k\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RETURN(value) => {
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