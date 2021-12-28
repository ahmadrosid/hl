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
	THIS(Vec<char>),
	TRUE(Vec<char>),
	FALSE(Vec<char>),
	SUPER(Vec<char>),
	NULL(Vec<char>),
	CLASS(Vec<char>),
	FINAL(Vec<char>),
	PACKAGE(Vec<char>),
	IMPORT(Vec<char>),
	PUBLIC(Vec<char>),
	PRIVATE(Vec<char>),
	PROTECTED(Vec<char>),
	EXTENDS(Vec<char>),
	STATIC(Vec<char>),
	VOID(Vec<char>),
	RETURN(Vec<char>),
	NEW(Vec<char>),
	IF(Vec<char>),
	ELSE(Vec<char>),
	INSTANCEOF(Vec<char>),
	BOOLEAN(Vec<char>),
	ASSERT(Vec<char>),
	CONTINUE(Vec<char>),
	LIST(Vec<char>),
	ARRAYLIST(Vec<char>),
	MAP(Vec<char>),
	HASHMAP(Vec<char>),
	LINKEDHASHSET(Vec<char>),
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"this" => Ok(Token::THIS(identifier.to_vec())),
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"super" => Ok(Token::SUPER(identifier.to_vec())),
		"null" => Ok(Token::NULL(identifier.to_vec())),
		"class" => Ok(Token::CLASS(identifier.to_vec())),
		"final" => Ok(Token::FINAL(identifier.to_vec())),
		"package" => Ok(Token::PACKAGE(identifier.to_vec())),
		"import" => Ok(Token::IMPORT(identifier.to_vec())),
		"public" => Ok(Token::PUBLIC(identifier.to_vec())),
		"private" => Ok(Token::PRIVATE(identifier.to_vec())),
		"protected" => Ok(Token::PROTECTED(identifier.to_vec())),
		"extends" => Ok(Token::EXTENDS(identifier.to_vec())),
		"static" => Ok(Token::STATIC(identifier.to_vec())),
		"void" => Ok(Token::VOID(identifier.to_vec())),
		"return" => Ok(Token::RETURN(identifier.to_vec())),
		"new" => Ok(Token::NEW(identifier.to_vec())),
		"if" => Ok(Token::IF(identifier.to_vec())),
		"else" => Ok(Token::ELSE(identifier.to_vec())),
		"instanceof" => Ok(Token::INSTANCEOF(identifier.to_vec())),
		"boolean" => Ok(Token::BOOLEAN(identifier.to_vec())),
		"assert" => Ok(Token::ASSERT(identifier.to_vec())),
		"continue" => Ok(Token::CONTINUE(identifier.to_vec())),
		"List" => Ok(Token::LIST(identifier.to_vec())),
		"ArrayList" => Ok(Token::ARRAYLIST(identifier.to_vec())),
		"Map" => Ok(Token::MAP(identifier.to_vec())),
		"HashMap" => Ok(Token::HASHMAP(identifier.to_vec())),
		"LinkedHashSet" => Ok(Token::LINKEDHASHSET(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}
