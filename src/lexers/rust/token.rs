// ---- DON'T EDIT THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
	ILLEGAL,
	EOF,
	CH(char),
	ENDL(char),
	INT(Vec<char>),
	IDENT(Vec<char>),
	ENTITY(Vec<char>),
	STRING(Vec<char>),
	KEYWORD(Vec<char>),
	TRUE(Vec<char>),
	FALSE(Vec<char>),
	NONE(Vec<char>),
	CSELF(Vec<char>),
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"None" => Ok(Token::NONE(identifier.to_vec())),
		"self" => Ok(Token::CSELF(identifier.to_vec())),
		"use" => Ok(Token::KEYWORD(identifier.to_vec())),
		"fn" => Ok(Token::KEYWORD(identifier.to_vec())),
		"for" => Ok(Token::KEYWORD(identifier.to_vec())),
		"loop" => Ok(Token::KEYWORD(identifier.to_vec())),
		"let" => Ok(Token::KEYWORD(identifier.to_vec())),
		"mut" => Ok(Token::KEYWORD(identifier.to_vec())),
		"if" => Ok(Token::KEYWORD(identifier.to_vec())),
		"else" => Ok(Token::KEYWORD(identifier.to_vec())),
		"pub" => Ok(Token::KEYWORD(identifier.to_vec())),
		"match" => Ok(Token::KEYWORD(identifier.to_vec())),
		"mod" => Ok(Token::KEYWORD(identifier.to_vec())),
		"return" => Ok(Token::KEYWORD(identifier.to_vec())),
		"where" => Ok(Token::KEYWORD(identifier.to_vec())),
		"impl" => Ok(Token::KEYWORD(identifier.to_vec())),
		"char" => Ok(Token::KEYWORD(identifier.to_vec())),
		"str" => Ok(Token::KEYWORD(identifier.to_vec())),
		"String" => Ok(Token::KEYWORD(identifier.to_vec())),
		"Vec" => Ok(Token::KEYWORD(identifier.to_vec())),
		"vec" => Ok(Token::KEYWORD(identifier.to_vec())),
		"crate" => Ok(Token::KEYWORD(identifier.to_vec())),
		"ref" => Ok(Token::KEYWORD(identifier.to_vec())),
		"unsafe" => Ok(Token::KEYWORD(identifier.to_vec())),
		"Self" => Ok(Token::KEYWORD(identifier.to_vec())),
		"async" => Ok(Token::KEYWORD(identifier.to_vec())),
		"await" => Ok(Token::KEYWORD(identifier.to_vec())),
		"type" => Ok(Token::KEYWORD(identifier.to_vec())),
		"Result" => Ok(Token::KEYWORD(identifier.to_vec())),
		"Option" => Ok(Token::KEYWORD(identifier.to_vec())),
		"dyn" => Ok(Token::KEYWORD(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}
