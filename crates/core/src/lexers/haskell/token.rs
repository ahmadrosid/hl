// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "True" => Ok(Token::CONSTANT(identifier.clone())),
        "False" => Ok(Token::CONSTANT(identifier.clone())),
        "return" => Ok(Token::CONSTANT(identifier.clone())),
        "mempty" => Ok(Token::CONSTANT(identifier.clone())),
        "not" => Ok(Token::CONSTANT(identifier.clone())),
        "foldr" => Ok(Token::CONSTANT(identifier.clone())),
        "Map" => Ok(Token::CONSTANT(identifier.clone())),
        "lookup" => Ok(Token::CONSTANT(identifier.clone())),
        "Left" => Ok(Token::CONSTANT(identifier.clone())),
        "Right" => Ok(Token::CONSTANT(identifier.clone())),
        "Walk" => Ok(Token::CONSTANT(identifier.clone())),
        "Char" => Ok(Token::CONSTANT(identifier.clone())),
        "Bool" => Ok(Token::CONSTANT(identifier.clone())),
        "Maybe" => Ok(Token::CONSTANT(identifier.clone())),
        "Int" => Ok(Token::CONSTANT(identifier.clone())),
        "Prelude" => Ok(Token::CONSTANT(identifier.clone())),
        "any" => Ok(Token::CONSTANT(identifier.clone())),
        "filter" => Ok(Token::CONSTANT(identifier.clone())),
        "map" => Ok(Token::CONSTANT(identifier.clone())),
        "init" => Ok(Token::CONSTANT(identifier.clone())),
        "null" => Ok(Token::CONSTANT(identifier.clone())),
        "last" => Ok(Token::CONSTANT(identifier.clone())),
        "unwords" => Ok(Token::CONSTANT(identifier.clone())),
        "otherwise" => Ok(Token::CONSTANT(identifier.clone())),
        "either" => Ok(Token::CONSTANT(identifier.clone())),
        "const" => Ok(Token::CONSTANT(identifier.clone())),
        "fmap" => Ok(Token::CONSTANT(identifier.clone())),
        "take" => Ok(Token::CONSTANT(identifier.clone())),
        "max" => Ok(Token::CONSTANT(identifier.clone())),
        "putStrLn" => Ok(Token::CONSTANT(identifier.clone())),
        "Bibtex" | "Biblatex" | "Nothing" | "Just" | "Str" | "Space" | "FancyVal" | "BibState"
        | "Many" => Ok(Token::ENTITYTAG(identifier.clone())),
        "as" | "import" | "type" | "case" | "do" | "of" | "if" | "then" | "else" | "let" | "in"
        | "module" | "qualified" | "where" | "LANGUAGE" | "OPTIONS_GHC" | "hiding" | "data"
        | "deriving" => Ok(Token::KEYWORD(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
