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
	USE(Vec<char>),
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
	KSTRING(Vec<char>),
	VEC(Vec<char>),
	KVEC(Vec<char>),
	CRATE(Vec<char>),
	REF(Vec<char>),
	UNSAFE(Vec<char>),
	KSELF(Vec<char>),
	ASYNC(Vec<char>),
	AWAIT(Vec<char>),
	TYPE(Vec<char>),
	RESULT(Vec<char>),
	OPTION(Vec<char>),
	DYN(Vec<char>),
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"None" => Ok(Token::NONE(identifier.to_vec())),
		"self" => Ok(Token::CSELF(identifier.to_vec())),
		"use" => Ok(Token::USE(identifier.to_vec())),
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
		"String" => Ok(Token::KSTRING(identifier.to_vec())),
		"Vec" => Ok(Token::VEC(identifier.to_vec())),
		"vec" => Ok(Token::KVEC(identifier.to_vec())),
		"crate" => Ok(Token::CRATE(identifier.to_vec())),
		"ref" => Ok(Token::REF(identifier.to_vec())),
		"unsafe" => Ok(Token::UNSAFE(identifier.to_vec())),
		"Self" => Ok(Token::KSELF(identifier.to_vec())),
		"async" => Ok(Token::ASYNC(identifier.to_vec())),
		"await" => Ok(Token::AWAIT(identifier.to_vec())),
		"type" => Ok(Token::TYPE(identifier.to_vec())),
		"Result" => Ok(Token::RESULT(identifier.to_vec())),
		"Option" => Ok(Token::OPTION(identifier.to_vec())),
		"dyn" => Ok(Token::DYN(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}
