#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
	ILLEGAL,

	// Base
	PLUS(char),
	MINUS(char),
	LPAREN(char),
	RPAREN(char),
	LBRACE(char),
	RBRACE(char),
	SPACE(char),
	ASSIGN(char),
	COLON(char),
	SEMICOLON(char),
	MEMBERACCESS(char),
	COMMA(char),
	GT(char),
	LT(char),
	AND(char),
	BANG(char),
	ASTERISK(char),
	ENDL(char),
	TAB(char),
	EOF(char),

	// Constants
	TRUE(Vec<char>),
	FALSE(Vec<char>),
	STRING(Vec<char>),
	IDENT(Vec<char>),
	INT(Vec<char>),

	// Keyword
	FN,
	FOR,
	LET,
	MUT,
	IF,
	ELSE,
	PUB,
	MATCH,
	MOD,
	RETURN,
	WHERE,
	IMPL,
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"fn" => Ok(Token::FN),
		"for" => Ok(Token::FOR),
		"let" => Ok(Token::LET),
		"mut" => Ok(Token::MUT),
		"if" => Ok(Token::IF),
		"else" => Ok(Token::ELSE),
		"pub" => Ok(Token::PUB),
		"match" => Ok(Token::MATCH),
		"mod" => Ok(Token::MOD),
		"return" => Ok(Token::RETURN),
		"where" => Ok(Token::WHERE),
		"impl" => Ok(Token::IMPL),
		_ => Err(String::from("Not a keyword"))
	}
}
