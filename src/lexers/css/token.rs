#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
	ILLEGAL,
	EOF,
	CH(char),
	ENDL(char),

	// Base
	CLASS(char),
	LBRACE(char),
	RBRACE(char),
	COLON(char),
	SEMICOLON(char),

	// Constants
	ROOT(Vec<char>),
	RGB(Vec<char>),
	RGBA(Vec<char>),
	CLAC(Vec<char>),
	MEDIA(Vec<char>),
	STRING(Vec<char>),
	INT(Vec<char>),

	// Var

	// Skip token
	IDENT(Vec<char>),

	// Entity
	AFTER(Vec<char>),
	BEFORE(Vec<char>),
	HOVER(Vec<char>),
	NOT(Vec<char>),
	FOCUS(Vec<char>),
	ACTIVE(Vec<char>),
	SELECTION(Vec<char>),
	PX(Vec<char>),
	REM(Vec<char>),
	ENTITY(Vec<char>),

	// Entity tag
	HTML(Vec<char>),
	BODY(Vec<char>),
	DIV(Vec<char>),
	SPAN(Vec<char>),
	APPLET(Vec<char>),
	OBJECT(Vec<char>),
	IFRAME(Vec<char>),
	H1(Vec<char>),
	H2(Vec<char>),
	H3(Vec<char>),
	H4(Vec<char>),
	H5(Vec<char>),
	H6(Vec<char>),
	P(Vec<char>),
	BLOCKQUOTE(Vec<char>),
	BUTTON(Vec<char>),
	PRE(Vec<char>),
	A(Vec<char>),
	ABBR(Vec<char>),
	ACRONYM(Vec<char>),
	ADDRESS(Vec<char>),
	BIG(Vec<char>),
	CITE(Vec<char>),
	CODE(Vec<char>),
	DEL(Vec<char>),
	DFN(Vec<char>),
	EM(Vec<char>),
	IMG(Vec<char>),
	INS(Vec<char>),
	KBD(Vec<char>),
	Q(Vec<char>),
	S(Vec<char>),
	SAMP(Vec<char>),
	SELECT(Vec<char>),
	SMALL(Vec<char>),
	STRIKE(Vec<char>),
	STRONG(Vec<char>),
	SUB(Vec<char>),
	SUP(Vec<char>),
	TT(Vec<char>),
	VAR(Vec<char>),
	B(Vec<char>),
	U(Vec<char>),
	I(Vec<char>),
	CENTER(Vec<char>),
	DL(Vec<char>),
	DT(Vec<char>),
	DD(Vec<char>),
	OL(Vec<char>),
	UL(Vec<char>),
	LI(Vec<char>),
	FIELDSET(Vec<char>),
	FORM(Vec<char>),
	LABEL(Vec<char>),
	LEGEND(Vec<char>),
	TABLE(Vec<char>),
	CAPTION(Vec<char>),
	TBODY(Vec<char>),
	TFOOT(Vec<char>),
	THEAD(Vec<char>),
	TR(Vec<char>),
	TH(Vec<char>),
	TD(Vec<char>),
	ARTICLE(Vec<char>),
	CANVAS(Vec<char>),
	EMBED(Vec<char>),
	OUTPUT(Vec<char>),
	RUBY(Vec<char>),
	SUMMARY(Vec<char>),
	TIME(Vec<char>),
	MARK(Vec<char>),
	AUDIO(Vec<char>),
	VIDEOARTICLE(Vec<char>),
	ASIDE(Vec<char>),
	DETAILS(Vec<char>),
	FIGCAPTION(Vec<char>),
	FIGURE(Vec<char>),
	FOOTER(Vec<char>),
	HEADER(Vec<char>),
	HGROUP(Vec<char>),
	MENU(Vec<char>),
	NAV(Vec<char>),
	SECTION(Vec<char>),
	VIDEO(Vec<char>),
	TEXTAREA(Vec<char>),
	INPUT(Vec<char>),

	// Keyword
	IMPORTANT(Vec<char>),
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"root" => Ok(Token::ROOT(identifier.to_vec())),
		"rgb" => Ok(Token::RGB(identifier.to_vec())),
		"rgba" => Ok(Token::RGBA(identifier.to_vec())),
		"calc" => Ok(Token::CLAC(identifier.to_vec())),
		"media" => Ok(Token::MEDIA(identifier.to_vec())),
		"after" => Ok(Token::AFTER(identifier.to_vec())),
		"before" => Ok(Token::BEFORE(identifier.to_vec())),
		"hover" => Ok(Token::HOVER(identifier.to_vec())),
		"not" => Ok(Token::NOT(identifier.to_vec())),
		"focus" => Ok(Token::FOCUS(identifier.to_vec())),
		"active" => Ok(Token::ACTIVE(identifier.to_vec())),
		"selection" => Ok(Token::SELECTION(identifier.to_vec())),
		"px" => Ok(Token::PX(identifier.to_vec())),
		"rem" => Ok(Token::REM(identifier.to_vec())),
		"html" => Ok(Token::HTML(identifier.to_vec())),
		"body" => Ok(Token::BODY(identifier.to_vec())),
		"div" => Ok(Token::DIV(identifier.to_vec())),
		"span" => Ok(Token::SPAN(identifier.to_vec())),
		"applet" => Ok(Token::APPLET(identifier.to_vec())),
		"object" => Ok(Token::OBJECT(identifier.to_vec())),
		"iframe" => Ok(Token::IFRAME(identifier.to_vec())),
		"h1" => Ok(Token::H1(identifier.to_vec())),
		"h2" => Ok(Token::H2(identifier.to_vec())),
		"h3" => Ok(Token::H3(identifier.to_vec())),
		"h4" => Ok(Token::H4(identifier.to_vec())),
		"h5" => Ok(Token::H5(identifier.to_vec())),
		"h6" => Ok(Token::H6(identifier.to_vec())),
		"p" => Ok(Token::P(identifier.to_vec())),
		"blockquote" => Ok(Token::BLOCKQUOTE(identifier.to_vec())),
		"button" => Ok(Token::BUTTON(identifier.to_vec())),
		"pre" => Ok(Token::PRE(identifier.to_vec())),
		"a" => Ok(Token::A(identifier.to_vec())),
		"abbr" => Ok(Token::ABBR(identifier.to_vec())),
		"acronym" => Ok(Token::ACRONYM(identifier.to_vec())),
		"address" => Ok(Token::ADDRESS(identifier.to_vec())),
		"big" => Ok(Token::BIG(identifier.to_vec())),
		"cite" => Ok(Token::CITE(identifier.to_vec())),
		"code" => Ok(Token::CODE(identifier.to_vec())),
		"del" => Ok(Token::DEL(identifier.to_vec())),
		"dfn" => Ok(Token::DFN(identifier.to_vec())),
		"em" => Ok(Token::EM(identifier.to_vec())),
		"img" => Ok(Token::IMG(identifier.to_vec())),
		"ins" => Ok(Token::INS(identifier.to_vec())),
		"kbd" => Ok(Token::KBD(identifier.to_vec())),
		"q" => Ok(Token::Q(identifier.to_vec())),
		"s" => Ok(Token::S(identifier.to_vec())),
		"samp" => Ok(Token::SAMP(identifier.to_vec())),
		"select" => Ok(Token::SELECT(identifier.to_vec())),
		"small" => Ok(Token::SMALL(identifier.to_vec())),
		"strike" => Ok(Token::STRIKE(identifier.to_vec())),
		"strong" => Ok(Token::STRONG(identifier.to_vec())),
		"sub" => Ok(Token::SUB(identifier.to_vec())),
		"sup" => Ok(Token::SUP(identifier.to_vec())),
		"tt" => Ok(Token::TT(identifier.to_vec())),
		"var" => Ok(Token::VAR(identifier.to_vec())),
		"b" => Ok(Token::B(identifier.to_vec())),
		"u" => Ok(Token::U(identifier.to_vec())),
		"i" => Ok(Token::I(identifier.to_vec())),
		"center" => Ok(Token::CENTER(identifier.to_vec())),
		"dl" => Ok(Token::DL(identifier.to_vec())),
		"dt" => Ok(Token::DT(identifier.to_vec())),
		"dd" => Ok(Token::DD(identifier.to_vec())),
		"ol" => Ok(Token::OL(identifier.to_vec())),
		"ul" => Ok(Token::UL(identifier.to_vec())),
		"li" => Ok(Token::LI(identifier.to_vec())),
		"fieldset" => Ok(Token::FIELDSET(identifier.to_vec())),
		"form" => Ok(Token::FORM(identifier.to_vec())),
		"label" => Ok(Token::LABEL(identifier.to_vec())),
		"legend" => Ok(Token::LEGEND(identifier.to_vec())),
		"table" => Ok(Token::TABLE(identifier.to_vec())),
		"caption" => Ok(Token::CAPTION(identifier.to_vec())),
		"tbody" => Ok(Token::TBODY(identifier.to_vec())),
		"tfoot" => Ok(Token::TFOOT(identifier.to_vec())),
		"thead" => Ok(Token::THEAD(identifier.to_vec())),
		"tr" => Ok(Token::TR(identifier.to_vec())),
		"th" => Ok(Token::TH(identifier.to_vec())),
		"td" => Ok(Token::TD(identifier.to_vec())),
		"article" => Ok(Token::ARTICLE(identifier.to_vec())),
		"canvas" => Ok(Token::CANVAS(identifier.to_vec())),
		"embed" => Ok(Token::EMBED(identifier.to_vec())),
		"output" => Ok(Token::OUTPUT(identifier.to_vec())),
		"ruby" => Ok(Token::RUBY(identifier.to_vec())),
		"summary" => Ok(Token::SUMMARY(identifier.to_vec())),
		"time" => Ok(Token::TIME(identifier.to_vec())),
		"mark" => Ok(Token::MARK(identifier.to_vec())),
		"audio" => Ok(Token::AUDIO(identifier.to_vec())),
		"videoarticle" => Ok(Token::VIDEOARTICLE(identifier.to_vec())),
		"aside" => Ok(Token::ASIDE(identifier.to_vec())),
		"details" => Ok(Token::DETAILS(identifier.to_vec())),
		"figcaption" => Ok(Token::FIGCAPTION(identifier.to_vec())),
		"figure" => Ok(Token::FIGURE(identifier.to_vec())),
		"footer" => Ok(Token::FOOTER(identifier.to_vec())),
		"header" => Ok(Token::HEADER(identifier.to_vec())),
		"hgroup" => Ok(Token::HGROUP(identifier.to_vec())),
		"menu" => Ok(Token::MENU(identifier.to_vec())),
		"nav" => Ok(Token::NAV(identifier.to_vec())),
		"section" => Ok(Token::SECTION(identifier.to_vec())),
		"video" => Ok(Token::VIDEO(identifier.to_vec())),
		"textarea" => Ok(Token::TEXTAREA(identifier.to_vec())),
		"input" => Ok(Token::INPUT(identifier.to_vec())),
		"important" => Ok(Token::IMPORTANT(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}