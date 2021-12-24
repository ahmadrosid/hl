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
	ARGS(Vec<char>),
	TRUE(Vec<char>),
	FALSE(Vec<char>),
	NIL(Vec<char>),
	BOOL(Vec<char>),
	CSTRING(Vec<char>),
	STRING(Vec<char>),
	INT(Vec<char>),

	// Var

	// Skip token
	IDENT(Vec<char>),

	// Entity
	ENTITY(Vec<char>),

	// Entity tag

	// Keyword
	PACKAGE(Vec<char>),
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
	TYPE(Vec<char>),
	INTERFACE(Vec<char>),
	CHAN(Vec<char>),
	STRUCT(Vec<char>),
	CONST(Vec<char>),
	RANGE(Vec<char>),
	BREAK(Vec<char>),
	MAP(Vec<char>),
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"Args" => Ok(Token::ARGS(identifier.to_vec())),
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"nil" => Ok(Token::NIL(identifier.to_vec())),
		"bool" => Ok(Token::BOOL(identifier.to_vec())),
		"string" => Ok(Token::CSTRING(identifier.to_vec())),
		"." => Ok(Token::ENTITY(identifier.to_vec())),
		"package" => Ok(Token::PACKAGE(identifier.to_vec())),
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
		"type" => Ok(Token::TYPE(identifier.to_vec())),
		"interface" => Ok(Token::INTERFACE(identifier.to_vec())),
		"chan" => Ok(Token::CHAN(identifier.to_vec())),
		"struct" => Ok(Token::STRUCT(identifier.to_vec())),
		"const" => Ok(Token::CONST(identifier.to_vec())),
		"range" => Ok(Token::RANGE(identifier.to_vec())),
		"break" => Ok(Token::BREAK(identifier.to_vec())),
		"map" => Ok(Token::MAP(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}
