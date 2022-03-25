// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "this" => Ok(Token::CONSTANT(identifier.clone())),
        "true" => Ok(Token::CONSTANT(identifier.clone())),
        "false" => Ok(Token::CONSTANT(identifier.clone())),
        "super" => Ok(Token::CONSTANT(identifier.clone())),
        "null" => Ok(Token::CONSTANT(identifier.clone())),
        "String" => Ok(Token::CONSTANT(identifier.clone())),
        "Long" => Ok(Token::CONSTANT(identifier.clone())),
        "Object" => Ok(Token::CONSTANT(identifier.clone())),
        "Boolean" => Ok(Token::CONSTANT(identifier.clone())),
        "Array" => Ok(Token::CONSTANT(identifier.clone())),
        "List" => Ok(Token::CONSTANT(identifier.clone())),
        "ArrayList" => Ok(Token::CONSTANT(identifier.clone())),
        "Arrays" => Ok(Token::CONSTANT(identifier.clone())),
        "Map" => Ok(Token::CONSTANT(identifier.clone())),
        "HashMap" => Ok(Token::CONSTANT(identifier.clone())),
        "LinkedHashSet" => Ok(Token::CONSTANT(identifier.clone())),
        "abstract" | "byte" | "break" | "class" | "double" | "float" | "final" | "int"
        | "interface" | "char" | "case" | "default" | "short" | "for" | "package" | "import"
        | "public" | "private" | "protected" | "extends" | "static" | "void" | "return" | "new"
        | "if" | "else" | "enum" | "instanceof" | "boolean" | "assert" | "continue" | "native"
        | "switch" | "synchronized" | "try" | "throw" | "catch" | "volatile" | "while"
        | "throws" | "finally" | "long" | "do" | "transient" | "strictfp" | "var" => {
            Ok(Token::KEYWORD(identifier.clone()))
        }
        _ => Err(String::from("Not a keyword")),
    }
}
