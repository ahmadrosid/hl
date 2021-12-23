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
	STRING(Vec<char>),
	INT(Vec<char>),

	// Skip token
	IDENT(Vec<char>),

	// Entity
	ENTITY(Vec<char>),

	// Keyword
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		_ => Err(String::from("Not a keyword"))
	}
}
