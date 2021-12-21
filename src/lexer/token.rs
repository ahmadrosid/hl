#[derive(PartialEq)]
#[derive(Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENL,
    SPACE,
    TAB,
    STRING(Vec<char>),
    PATHSEPARATOR(Vec<char>),
    MEMBERACCESS(char),
    IDENT(Vec<char>),
    INT(Vec<char>),
    ASSIGN(char),
    PLUS(char),
    COMMA(char),
    COLON(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    MINUS(char),
    BANG(char),
    ASTERISK(char),
    SLASH(char),
    LT(char),
    GT(char)
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let identifiers: String = identifier.into_iter().collect();
    match &identifiers[..] {
        "fn" => Ok(Token::FUNCTION),
        "let" => Ok(Token::LET),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "return" => Ok(Token::RETURN),
        _ => Err(String::from("Not a keyword"))
    }
}
