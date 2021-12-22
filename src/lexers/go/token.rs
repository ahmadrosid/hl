#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
	ILLEGAL,
	EOF,
	CH(char),
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

	// Constants
	TRUE(Vec<char>),
	FALSE(Vec<char>),
	STRING(Vec<char>),
	INT(Vec<char>),

	// Skip token
	IDENT(Vec<char>),

	// Entity
	ENTITY(Vec<char>),

	// Keyword
	FUNC(Vec<char>),
	FOR(Vec<char>),
	VAR(Vec<char>),
	IF(Vec<char>),
	ELSE(Vec<char>),
	MATCH(Vec<char>),
	CASE(Vec<char>),
	RETURN(Vec<char>),
	DEFAULT(Vec<char>),
	IMPORT(Vec<char>),
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"func" => Ok(Token::FUNC(identifier.to_vec())),
		"for" => Ok(Token::FOR(identifier.to_vec())),
		"var" => Ok(Token::VAR(identifier.to_vec())),
		"if" => Ok(Token::IF(identifier.to_vec())),
		"else" => Ok(Token::ELSE(identifier.to_vec())),
		"switch" => Ok(Token::MATCH(identifier.to_vec())),
		"case" => Ok(Token::CASE(identifier.to_vec())),
		"return" => Ok(Token::RETURN(identifier.to_vec())),
		"default" => Ok(Token::DEFAULT(identifier.to_vec())),
		"import" => Ok(Token::IMPORT(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}
