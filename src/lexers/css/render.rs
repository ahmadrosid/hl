// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::css::Lexer;
use crate::lexers::css::token;

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
			token::Token::AFTER(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::BEFORE(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::HOVER(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::NOT(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FOCUS(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ACTIVE(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SELECTION(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::PX(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::REM(value) => {
				html.push_str(&format!("<span class=\"hl-en\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::HTML(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::BODY(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DIV(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SPAN(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::APPLET(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::OBJECT(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IFRAME(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::H1(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::H2(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::H3(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::H4(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::H5(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::H6(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::P(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::BLOCKQUOTE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::BUTTON(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::PRE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::A(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ABBR(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ACRONYM(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ADDRESS(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::BIG(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CITE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CODE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DEL(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DFN(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::EM(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::IMG(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::INS(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::KBD(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::Q(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::S(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SAMP(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SELECT(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SMALL(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::STRIKE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::STRONG(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SUB(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SUP(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TT(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VAR(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::B(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::U(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::I(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CENTER(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DL(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DT(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DD(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::OL(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::UL(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::LI(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FIELDSET(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FORM(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::LABEL(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::LEGEND(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TABLE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CAPTION(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TBODY(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TFOOT(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::THEAD(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TR(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TH(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TD(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ARTICLE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CANVAS(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::EMBED(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::OUTPUT(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RUBY(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SUMMARY(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TIME(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MARK(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::AUDIO(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VIDEOARTICLE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ASIDE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::DETAILS(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FIGCAPTION(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FIGURE(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::FOOTER(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::HEADER(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::HGROUP(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MENU(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::NAV(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::SECTION(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::VIDEO(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::TEXTAREA(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::INPUT(value) => {
				html.push_str(&format!("<span class=\"hl-ent\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::ROOT(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RGB(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::RGBA(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::CLAC(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
			}
			token::Token::MEDIA(value) => {
				html.push_str(&format!("<span class=\"hl-c\">{}</span>", value.iter().collect::<String>()));
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
