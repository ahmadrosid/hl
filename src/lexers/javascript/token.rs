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
	UNDEFINED(Vec<char>),
	NULL(Vec<char>),
	STRING(Vec<char>),
	INT(Vec<char>),

	// Var
	VINFINITY(Vec<char>),
	VNAN(Vec<char>),
	MATH(Vec<char>),
	DATE(Vec<char>),

	// Skip token
	IDENT(Vec<char>),

	// Entity
	ENTITY(Vec<char>),

	// Entity tag

	// Keyword
	IMPORT(Vec<char>),
	AS(Vec<char>),
	NEW(Vec<char>),
	THIS(Vec<char>),
	CLASS(Vec<char>),
	VAR(Vec<char>),
	CONST(Vec<char>),
	LET(Vec<char>),
	FUNCTION(Vec<char>),
	FOR(Vec<char>),
	IF(Vec<char>),
	ELSE(Vec<char>),
	VOID(Vec<char>),
	RETURN(Vec<char>),
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"undefined" => Ok(Token::UNDEFINED(identifier.to_vec())),
		"null" => Ok(Token::NULL(identifier.to_vec())),
		"Infinity" => Ok(Token::VINFINITY(identifier.to_vec())),
		"NaN" => Ok(Token::VNAN(identifier.to_vec())),
		"Math" => Ok(Token::MATH(identifier.to_vec())),
		"Date" => Ok(Token::DATE(identifier.to_vec())),
		"import" => Ok(Token::IMPORT(identifier.to_vec())),
		"as" => Ok(Token::AS(identifier.to_vec())),
		"new" => Ok(Token::NEW(identifier.to_vec())),
		"this" => Ok(Token::THIS(identifier.to_vec())),
		"class" => Ok(Token::CLASS(identifier.to_vec())),
		"var" => Ok(Token::VAR(identifier.to_vec())),
		"const" => Ok(Token::CONST(identifier.to_vec())),
		"let" => Ok(Token::LET(identifier.to_vec())),
		"function" => Ok(Token::FUNCTION(identifier.to_vec())),
		"for" => Ok(Token::FOR(identifier.to_vec())),
		"if" => Ok(Token::IF(identifier.to_vec())),
		"else" => Ok(Token::ELSE(identifier.to_vec())),
		"void" => Ok(Token::VOID(identifier.to_vec())),
		"return" => Ok(Token::RETURN(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}
