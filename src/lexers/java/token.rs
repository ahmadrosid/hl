// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
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
    CSTRING(Vec<char>),
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
        "String" => Ok(Token::CSTRING(identifier.to_vec())),
        "abstract" => Ok(Token::KEYWORD(identifier.to_vec())),
        "byte" => Ok(Token::KEYWORD(identifier.to_vec())),
        "break" => Ok(Token::KEYWORD(identifier.to_vec())),
        "class" => Ok(Token::KEYWORD(identifier.to_vec())),
        "double" => Ok(Token::KEYWORD(identifier.to_vec())),
        "float" => Ok(Token::KEYWORD(identifier.to_vec())),
        "final" => Ok(Token::KEYWORD(identifier.to_vec())),
        "int" => Ok(Token::KEYWORD(identifier.to_vec())),
        "interface" => Ok(Token::KEYWORD(identifier.to_vec())),
        "char" => Ok(Token::KEYWORD(identifier.to_vec())),
        "case" => Ok(Token::KEYWORD(identifier.to_vec())),
        "default" => Ok(Token::KEYWORD(identifier.to_vec())),
        "short" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "package" => Ok(Token::KEYWORD(identifier.to_vec())),
        "import" => Ok(Token::KEYWORD(identifier.to_vec())),
        "public" => Ok(Token::KEYWORD(identifier.to_vec())),
        "private" => Ok(Token::KEYWORD(identifier.to_vec())),
        "protected" => Ok(Token::KEYWORD(identifier.to_vec())),
        "extends" => Ok(Token::KEYWORD(identifier.to_vec())),
        "static" => Ok(Token::KEYWORD(identifier.to_vec())),
        "void" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "new" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "enum" => Ok(Token::KEYWORD(identifier.to_vec())),
        "instanceof" => Ok(Token::KEYWORD(identifier.to_vec())),
        "boolean" => Ok(Token::KEYWORD(identifier.to_vec())),
        "assert" => Ok(Token::KEYWORD(identifier.to_vec())),
        "continue" => Ok(Token::KEYWORD(identifier.to_vec())),
        "List" => Ok(Token::KEYWORD(identifier.to_vec())),
        "ArrayList" => Ok(Token::KEYWORD(identifier.to_vec())),
        "Map" => Ok(Token::KEYWORD(identifier.to_vec())),
        "native" => Ok(Token::KEYWORD(identifier.to_vec())),
        "HashMap" => Ok(Token::KEYWORD(identifier.to_vec())),
        "LinkedHashSet" => Ok(Token::KEYWORD(identifier.to_vec())),
        "switch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "synchronized" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "throw" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "volatile" => Ok(Token::KEYWORD(identifier.to_vec())),
        "while" => Ok(Token::KEYWORD(identifier.to_vec())),
        "throws" => Ok(Token::KEYWORD(identifier.to_vec())),
        "finally" => Ok(Token::KEYWORD(identifier.to_vec())),
        "long" => Ok(Token::KEYWORD(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "transient" => Ok(Token::KEYWORD(identifier.to_vec())),
        "strictfp" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword"))
    }
}
