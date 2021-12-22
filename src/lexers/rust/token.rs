#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
	ILLEGAL,
	ENDL(char),

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
	TAB(char),
	EOF(char),

	// Constants
	TRUE(Vec<char>),
	FALSE(Vec<char>),
	PRINTLN(Vec<char>),
	STRING(Vec<char>),
	INT(Vec<char>),

	// Skip token
	IDENT(Vec<char>),

	// Entity
	ENTITY(Vec<char>),

	// Keyword
	FN(Vec<char>),
	FOR(Vec<char>),
	LOOP(Vec<char>),
	LET(Vec<char>),
	MUT(Vec<char>),
	IF(Vec<char>),
	ELSE(Vec<char>),
	PUB(Vec<char>),
	MATCH(Vec<char>),
	MOD(Vec<char>),
	RETURN(Vec<char>),
	WHERE(Vec<char>),
	IMPL(Vec<char>),
	CHAR(Vec<char>),
	STR(Vec<char>),
	CRATE(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"println" => Ok(Token::PRINTLN(identifier.to_vec())),
		"fn" => Ok(Token::FN(identifier.to_vec())),
		"for" => Ok(Token::FOR(identifier.to_vec())),
		"loop" => Ok(Token::LOOP(identifier.to_vec())),
		"let" => Ok(Token::LET(identifier.to_vec())),
		"mut" => Ok(Token::MUT(identifier.to_vec())),
		"if" => Ok(Token::IF(identifier.to_vec())),
		"else" => Ok(Token::ELSE(identifier.to_vec())),
		"pub" => Ok(Token::PUB(identifier.to_vec())),
		"match" => Ok(Token::MATCH(identifier.to_vec())),
		"mod" => Ok(Token::MOD(identifier.to_vec())),
		"return" => Ok(Token::RETURN(identifier.to_vec())),
		"where" => Ok(Token::WHERE(identifier.to_vec())),
		"impl" => Ok(Token::IMPL(identifier.to_vec())),
		"char" => Ok(Token::CHAR(identifier.to_vec())),
		"str" => Ok(Token::STR(identifier.to_vec())),
		"crate" => Ok(Token::CRATE(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}