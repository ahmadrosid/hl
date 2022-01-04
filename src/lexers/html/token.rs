// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    CH(char),
    ENDL(char),
    INT(Vec<char>),
    IDENT(Vec<char>),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    CONSTANT(Vec<char>),
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let identifiers: String = identifier.into_iter().collect();
    match &identifiers[..] {
        _ => Err(String::from("Not a keyword"))
    }
}
