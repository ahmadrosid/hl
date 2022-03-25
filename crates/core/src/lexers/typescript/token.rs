// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" => Ok(Token::CONSTANT(identifier.clone())),
        "false" => Ok(Token::CONSTANT(identifier.clone())),
        "undefined" => Ok(Token::CONSTANT(identifier.clone())),
        "null" => Ok(Token::CONSTANT(identifier.clone())),
        "number" => Ok(Token::CONSTANT(identifier.clone())),
        "string" => Ok(Token::CONSTANT(identifier.clone())),
        "boolean" => Ok(Token::CONSTANT(identifier.clone())),
        "type" => Ok(Token::CONSTANT(identifier.clone())),
        "Infinity" => Ok(Token::VAR(identifier.clone())),
        "NaN" => Ok(Token::VAR(identifier.clone())),
        "Math" => Ok(Token::VAR(identifier.clone())),
        "Date" => Ok(Token::VAR(identifier.clone())),
        "async" => Ok(Token::KEYWORD(identifier.clone())),
        "await" => Ok(Token::KEYWORD(identifier.clone())),
        "break" => Ok(Token::KEYWORD(identifier.clone())),
        "as" => Ok(Token::KEYWORD(identifier.clone())),
        "switch" => Ok(Token::KEYWORD(identifier.clone())),
        "case" => Ok(Token::KEYWORD(identifier.clone())),
        "if" => Ok(Token::KEYWORD(identifier.clone())),
        "throw" => Ok(Token::KEYWORD(identifier.clone())),
        "else" => Ok(Token::KEYWORD(identifier.clone())),
        "var" => Ok(Token::KEYWORD(identifier.clone())),
        "get" => Ok(Token::KEYWORD(identifier.clone())),
        "module" => Ok(Token::KEYWORD(identifier.clone())),
        "instanceof" => Ok(Token::KEYWORD(identifier.clone())),
        "import" => Ok(Token::KEYWORD(identifier.clone())),
        "from" => Ok(Token::KEYWORD(identifier.clone())),
        "typeof" => Ok(Token::KEYWORD(identifier.clone())),
        "public" => Ok(Token::KEYWORD(identifier.clone())),
        "private" => Ok(Token::KEYWORD(identifier.clone())),
        "enum" => Ok(Token::KEYWORD(identifier.clone())),
        "export" => Ok(Token::KEYWORD(identifier.clone())),
        "finally" => Ok(Token::KEYWORD(identifier.clone())),
        "for" => Ok(Token::KEYWORD(identifier.clone())),
        "while" => Ok(Token::KEYWORD(identifier.clone())),
        "void" => Ok(Token::KEYWORD(identifier.clone())),
        "super" => Ok(Token::KEYWORD(identifier.clone())),
        "this" => Ok(Token::KEYWORD(identifier.clone())),
        "in" => Ok(Token::KEYWORD(identifier.clone())),
        "return" => Ok(Token::KEYWORD(identifier.clone())),
        "any" => Ok(Token::KEYWORD(identifier.clone())),
        "extends" => Ok(Token::KEYWORD(identifier.clone())),
        "static" => Ok(Token::KEYWORD(identifier.clone())),
        "let" => Ok(Token::KEYWORD(identifier.clone())),
        "package" => Ok(Token::KEYWORD(identifier.clone())),
        "implements" => Ok(Token::KEYWORD(identifier.clone())),
        "interface" => Ok(Token::KEYWORD(identifier.clone())),
        "function" => Ok(Token::KEYWORD(identifier.clone())),
        "new" => Ok(Token::KEYWORD(identifier.clone())),
        "try" => Ok(Token::KEYWORD(identifier.clone())),
        "yield" => Ok(Token::KEYWORD(identifier.clone())),
        "const" => Ok(Token::KEYWORD(identifier.clone())),
        "continue" => Ok(Token::KEYWORD(identifier.clone())),
        "do" => Ok(Token::KEYWORD(identifier.clone())),
        "catch" => Ok(Token::KEYWORD(identifier.clone())),
        "default" => Ok(Token::KEYWORD(identifier.clone())),
        "class" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
