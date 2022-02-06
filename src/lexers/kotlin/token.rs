// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    IDENT(Vec<char>),
    INT(Vec<char>),
    CH(char),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    VAR(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "null" => Ok(Token::CONSTANT(identifier.to_vec())),
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "this" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Any" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Array" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Boolean" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Double" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Long" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Int" => Ok(Token::CONSTANT(identifier.to_vec())),
        "String" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Short" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Lazy" => Ok(Token::CONSTANT(identifier.to_vec())),
        "List" => Ok(Token::CONSTANT(identifier.to_vec())),
        "as" => Ok(Token::KEYWORD(identifier.to_vec())),
        "abstract" => Ok(Token::KEYWORD(identifier.to_vec())),
        "break" => Ok(Token::KEYWORD(identifier.to_vec())),
        "class" => Ok(Token::KEYWORD(identifier.to_vec())),
        "continue" => Ok(Token::KEYWORD(identifier.to_vec())),
        "companion" => Ok(Token::KEYWORD(identifier.to_vec())),
        "data" => Ok(Token::KEYWORD(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "enum" => Ok(Token::KEYWORD(identifier.to_vec())),
        "external" => Ok(Token::KEYWORD(identifier.to_vec())),
        "expect" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "fun" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "in" => Ok(Token::KEYWORD(identifier.to_vec())),
        "internal" => Ok(Token::KEYWORD(identifier.to_vec())),
        "import" => Ok(Token::KEYWORD(identifier.to_vec())),
        "interface" => Ok(Token::KEYWORD(identifier.to_vec())),
        "inline" => Ok(Token::KEYWORD(identifier.to_vec())),
        "is" => Ok(Token::KEYWORD(identifier.to_vec())),
        "noinline" => Ok(Token::KEYWORD(identifier.to_vec())),
        "object" => Ok(Token::KEYWORD(identifier.to_vec())),
        "open" => Ok(Token::KEYWORD(identifier.to_vec())),
        "operator" => Ok(Token::KEYWORD(identifier.to_vec())),
        "override" => Ok(Token::KEYWORD(identifier.to_vec())),
        "package" => Ok(Token::KEYWORD(identifier.to_vec())),
        "public" => Ok(Token::KEYWORD(identifier.to_vec())),
        "private" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "super" => Ok(Token::KEYWORD(identifier.to_vec())),
        "set" => Ok(Token::KEYWORD(identifier.to_vec())),
        "get" => Ok(Token::KEYWORD(identifier.to_vec())),
        "throw" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "typealias" => Ok(Token::KEYWORD(identifier.to_vec())),
        "typeof" => Ok(Token::KEYWORD(identifier.to_vec())),
        "val" => Ok(Token::KEYWORD(identifier.to_vec())),
        "var" => Ok(Token::KEYWORD(identifier.to_vec())),
        "when" => Ok(Token::KEYWORD(identifier.to_vec())),
        "while" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}