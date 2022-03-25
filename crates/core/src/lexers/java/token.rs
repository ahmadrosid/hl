// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "this" | "true" | "false" | "super" | "null" | "String" | "Long" | "Object" | "Boolean"
        | "Array" | "List" | "ArrayList" | "Arrays" | "Map" | "HashMap" | "LinkedHashSet" => {
            Ok(Token::CONSTANT(identifier.clone()))
        }
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
