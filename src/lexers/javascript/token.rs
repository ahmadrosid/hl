// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    IDENT(Vec<char>),
    INT(Vec<char>),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    CONSTANT(Vec<char>),
    KEYWORD(Vec<char>),
    VINFINITY(Vec<char>),
    VNAN(Vec<char>),
    MATH(Vec<char>),
    DATE(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "undefined" => Ok(Token::CONSTANT(identifier.to_vec())),
        "null" => Ok(Token::CONSTANT(identifier.to_vec())),
        "Infinity" => Ok(Token::VINFINITY(identifier.to_vec())),
        "NaN" => Ok(Token::VNAN(identifier.to_vec())),
        "Math" => Ok(Token::MATH(identifier.to_vec())),
        "Date" => Ok(Token::DATE(identifier.to_vec())),
        "await" => Ok(Token::KEYWORD(identifier.to_vec())),
        "break" => Ok(Token::KEYWORD(identifier.to_vec())),
        "case" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "class" => Ok(Token::KEYWORD(identifier.to_vec())),
        "const" => Ok(Token::KEYWORD(identifier.to_vec())),
        "continue" => Ok(Token::KEYWORD(identifier.to_vec())),
        "debugger" => Ok(Token::KEYWORD(identifier.to_vec())),
        "default" => Ok(Token::KEYWORD(identifier.to_vec())),
        "delete" => Ok(Token::KEYWORD(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "enum" => Ok(Token::KEYWORD(identifier.to_vec())),
        "export" => Ok(Token::KEYWORD(identifier.to_vec())),
        "extends" => Ok(Token::KEYWORD(identifier.to_vec())),
        "finally" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "function" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "implements" => Ok(Token::KEYWORD(identifier.to_vec())),
        "import" => Ok(Token::KEYWORD(identifier.to_vec())),
        "in" => Ok(Token::KEYWORD(identifier.to_vec())),
        "instanceof" => Ok(Token::KEYWORD(identifier.to_vec())),
        "interface" => Ok(Token::KEYWORD(identifier.to_vec())),
        "let" => Ok(Token::KEYWORD(identifier.to_vec())),
        "new" => Ok(Token::KEYWORD(identifier.to_vec())),
        "package" => Ok(Token::KEYWORD(identifier.to_vec())),
        "private" => Ok(Token::KEYWORD(identifier.to_vec())),
        "protected" => Ok(Token::KEYWORD(identifier.to_vec())),
        "public" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "super" => Ok(Token::KEYWORD(identifier.to_vec())),
        "switch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "static" => Ok(Token::KEYWORD(identifier.to_vec())),
        "this" => Ok(Token::KEYWORD(identifier.to_vec())),
        "throw" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "typeof" => Ok(Token::KEYWORD(identifier.to_vec())),
        "var" => Ok(Token::KEYWORD(identifier.to_vec())),
        "void" => Ok(Token::KEYWORD(identifier.to_vec())),
        "while" => Ok(Token::KEYWORD(identifier.to_vec())),
        "with" => Ok(Token::KEYWORD(identifier.to_vec())),
        "yield" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
