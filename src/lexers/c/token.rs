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
	NULL(Vec<char>),
	SizeT(Vec<char>),
	COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
	let identifiers: String = identifier.into_iter().collect();
	match &identifiers[..] {
		"true" => Ok(Token::TRUE(identifier.to_vec())),
		"false" => Ok(Token::FALSE(identifier.to_vec())),
		"NULL" => Ok(Token::NULL(identifier.to_vec())),
		"size_t" => Ok(Token::SizeT(identifier.to_vec())),
		"asm" => Ok(Token::KEYWORD(identifier.to_vec())),
		"const" => Ok(Token::KEYWORD(identifier.to_vec())),
		"char" => Ok(Token::KEYWORD(identifier.to_vec())),
		"case" => Ok(Token::KEYWORD(identifier.to_vec())),
		"catch" => Ok(Token::KEYWORD(identifier.to_vec())),
		"continue" => Ok(Token::KEYWORD(identifier.to_vec())),
		"do" => Ok(Token::KEYWORD(identifier.to_vec())),
		"default" => Ok(Token::KEYWORD(identifier.to_vec())),
		"define" => Ok(Token::KEYWORD(identifier.to_vec())),
		"delete" => Ok(Token::KEYWORD(identifier.to_vec())),
		"double" => Ok(Token::KEYWORD(identifier.to_vec())),
		"else" => Ok(Token::KEYWORD(identifier.to_vec())),
		"float" => Ok(Token::KEYWORD(identifier.to_vec())),
		"goto" => Ok(Token::KEYWORD(identifier.to_vec())),
		"for" => Ok(Token::KEYWORD(identifier.to_vec())),
		"if" => Ok(Token::KEYWORD(identifier.to_vec())),
		"ifdef" => Ok(Token::KEYWORD(identifier.to_vec())),
		"endif" => Ok(Token::KEYWORD(identifier.to_vec())),
		"long" => Ok(Token::KEYWORD(identifier.to_vec())),
		"return" => Ok(Token::KEYWORD(identifier.to_vec())),
		"int" => Ok(Token::KEYWORD(identifier.to_vec())),
		"include" => Ok(Token::KEYWORD(identifier.to_vec())),
		"inline" => Ok(Token::KEYWORD(identifier.to_vec())),
		"unsigned" => Ok(Token::KEYWORD(identifier.to_vec())),
		"union" => Ok(Token::KEYWORD(identifier.to_vec())),
		"break" => Ok(Token::KEYWORD(identifier.to_vec())),
		"void" => Ok(Token::KEYWORD(identifier.to_vec())),
		"volatile" => Ok(Token::KEYWORD(identifier.to_vec())),
		"register" => Ok(Token::KEYWORD(identifier.to_vec())),
		"signed" => Ok(Token::KEYWORD(identifier.to_vec())),
		"static" => Ok(Token::KEYWORD(identifier.to_vec())),
		"struct" => Ok(Token::KEYWORD(identifier.to_vec())),
		"sizeof" => Ok(Token::KEYWORD(identifier.to_vec())),
		"short" => Ok(Token::KEYWORD(identifier.to_vec())),
		"switch" => Ok(Token::KEYWORD(identifier.to_vec())),
		"typedef" => Ok(Token::KEYWORD(identifier.to_vec())),
		"try" => Ok(Token::KEYWORD(identifier.to_vec())),
		"throw" => Ok(Token::KEYWORD(identifier.to_vec())),
		"while" => Ok(Token::KEYWORD(identifier.to_vec())),
		_ => Err(String::from("Not a keyword"))
	}
}
