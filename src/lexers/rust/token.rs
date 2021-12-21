#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
	ILLEGAL,
	EOF,

	// Base
	PLUS(char),
	MINUS(char),
	LPARENT(char),
	RPARENT(char),
	LBRACE(char),
	RBRACE(char),
	SPACE(char),
	ASSIGN(char),
	COLON(char),
	SEMICOLON(char),
	GT(char),
	LT(char),
	ASTERISK(char),
	EDNL(char),
	TAB(char),

	// Constants
	TRUE(Vec<char>),
	FALSE(Vec<char>),

	// Keyword
	FN(Vec<char>),
	LET(Vec<char>),
	MUT(Vec<char>),
	IF(Vec<char>),
	ELSE(Vec<char>),
	PUB(Vec<char>),
	MATCH(Vec<char>),
	MOD(Vec<char>),
	RETURN(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"fn" => OK(Token::FN),
		"let" => OK(Token::LET),
		"mut" => OK(Token::MUT),
		"if" => OK(Token::IF),
		"else" => OK(Token::ELSE),
		"pub" => OK(Token::PUB),
		"match" => OK(Token::MATCH),
		"mod" => OK(Token::MOD),
		"return" => OK(Token::RETURN),
		_ => Err(String::from("Not a keyword"))
	}
}
