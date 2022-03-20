pub mod c;

#[derive(PartialEq, Debug)]
pub enum Token {
    ILLEGAL,
    EOF,
    ENDL(char),
    CH(char),
    HEAD(Vec<char>),
    IDENT(Vec<char>),
    CONSTANT(Vec<char>),
    INT(Vec<char>),
    ENTITYTAG(Vec<char>),
    COMMENT(Vec<char>),
    ENTITY(Vec<char>),
    STRING(Vec<char>),
    KEYWORD(Vec<char>),
    VAR(Vec<char>),
}

