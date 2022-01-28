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
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "None" => Ok(Token::CONSTANT(identifier.to_vec())),
        "as" => Ok(Token::KEYWORD(identifier.to_vec())),
        "break" => Ok(Token::KEYWORD(identifier.to_vec())),
        "const" => Ok(Token::KEYWORD(identifier.to_vec())),
        "continue" => Ok(Token::KEYWORD(identifier.to_vec())),
        "char" => Ok(Token::KEYWORD(identifier.to_vec())),
        "crate" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "enum" => Ok(Token::KEYWORD(identifier.to_vec())),
        "extern" => Ok(Token::KEYWORD(identifier.to_vec())),
        "fn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "impl" => Ok(Token::KEYWORD(identifier.to_vec())),
        "in" => Ok(Token::KEYWORD(identifier.to_vec())),
        "let" => Ok(Token::KEYWORD(identifier.to_vec())),
        "loop" => Ok(Token::KEYWORD(identifier.to_vec())),
        "match" => Ok(Token::KEYWORD(identifier.to_vec())),
        "mod" => Ok(Token::KEYWORD(identifier.to_vec())),
        "move" => Ok(Token::KEYWORD(identifier.to_vec())),
        "mut" => Ok(Token::KEYWORD(identifier.to_vec())),
        "pub" => Ok(Token::KEYWORD(identifier.to_vec())),
        "ref" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "self" => Ok(Token::KEYWORD(identifier.to_vec())),
        "Self" => Ok(Token::KEYWORD(identifier.to_vec())),
        "static" => Ok(Token::KEYWORD(identifier.to_vec())),
        "struct" => Ok(Token::KEYWORD(identifier.to_vec())),
        "super" => Ok(Token::KEYWORD(identifier.to_vec())),
        "trait" => Ok(Token::KEYWORD(identifier.to_vec())),
        "type" => Ok(Token::KEYWORD(identifier.to_vec())),
        "unsafe" => Ok(Token::KEYWORD(identifier.to_vec())),
        "use" => Ok(Token::KEYWORD(identifier.to_vec())),
        "where" => Ok(Token::KEYWORD(identifier.to_vec())),
        "while" => Ok(Token::KEYWORD(identifier.to_vec())),
        "async" => Ok(Token::KEYWORD(identifier.to_vec())),
        "await" => Ok(Token::KEYWORD(identifier.to_vec())),
        "dyn" => Ok(Token::KEYWORD(identifier.to_vec())),
        "abstract" => Ok(Token::KEYWORD(identifier.to_vec())),
        "become" => Ok(Token::KEYWORD(identifier.to_vec())),
        "box" => Ok(Token::KEYWORD(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "final" => Ok(Token::KEYWORD(identifier.to_vec())),
        "macro" => Ok(Token::KEYWORD(identifier.to_vec())),
        "override" => Ok(Token::KEYWORD(identifier.to_vec())),
        "priv" => Ok(Token::KEYWORD(identifier.to_vec())),
        "typeof" => Ok(Token::KEYWORD(identifier.to_vec())),
        "unsized" => Ok(Token::KEYWORD(identifier.to_vec())),
        "virtual" => Ok(Token::KEYWORD(identifier.to_vec())),
        "yield" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "union" => Ok(Token::KEYWORD(identifier.to_vec())),
        "'static" => Ok(Token::KEYWORD(identifier.to_vec())),
        "String" => Ok(Token::KEYWORD(identifier.to_vec())),
        "Option" => Ok(Token::KEYWORD(identifier.to_vec())),
        "Default" => Ok(Token::KEYWORD(identifier.to_vec())),
        "Result" => Ok(Token::KEYWORD(identifier.to_vec())),
        "Vec" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i8" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i16" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i32" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i64" => Ok(Token::KEYWORD(identifier.to_vec())),
        "i128" => Ok(Token::KEYWORD(identifier.to_vec())),
        "isize" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u8" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u16" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u32" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u64" => Ok(Token::KEYWORD(identifier.to_vec())),
        "u128" => Ok(Token::KEYWORD(identifier.to_vec())),
        "usize" => Ok(Token::KEYWORD(identifier.to_vec())),
        "f32" => Ok(Token::KEYWORD(identifier.to_vec())),
        "f64" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
