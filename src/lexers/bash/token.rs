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
    COMMENT(Vec<char>),
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "builtin" => Ok(Token::CONSTANT(identifier.to_vec())),
        "command" => Ok(Token::CONSTANT(identifier.to_vec())),
        "compgen" => Ok(Token::CONSTANT(identifier.to_vec())),
        "echo" => Ok(Token::CONSTANT(identifier.to_vec())),
        "eval" => Ok(Token::CONSTANT(identifier.to_vec())),
        "exit" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "hash" => Ok(Token::CONSTANT(identifier.to_vec())),
        "kill" => Ok(Token::CONSTANT(identifier.to_vec())),
        "read" => Ok(Token::CONSTANT(identifier.to_vec())),
        "source" => Ok(Token::CONSTANT(identifier.to_vec())),
        "unset" => Ok(Token::CONSTANT(identifier.to_vec())),
        "test" => Ok(Token::CONSTANT(identifier.to_vec())),
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "printf" => Ok(Token::CONSTANT(identifier.to_vec())),
        "case" => Ok(Token::KEYWORD(identifier.to_vec())),
        "continue" => Ok(Token::KEYWORD(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "done" => Ok(Token::KEYWORD(identifier.to_vec())),
        "elif" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "esac" => Ok(Token::KEYWORD(identifier.to_vec())),
        "export" => Ok(Token::KEYWORD(identifier.to_vec())),
        "fi" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "function" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "in" => Ok(Token::KEYWORD(identifier.to_vec())),
        "local" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "select" => Ok(Token::KEYWORD(identifier.to_vec())),
        "then" => Ok(Token::KEYWORD(identifier.to_vec())),
        "time" => Ok(Token::KEYWORD(identifier.to_vec())),
        "until" => Ok(Token::KEYWORD(identifier.to_vec())),
        "while" => Ok(Token::KEYWORD(identifier.to_vec())),
        "EOF" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}
