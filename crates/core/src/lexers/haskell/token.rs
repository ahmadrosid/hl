// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;
pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "True" | "False" | "return" | "mempty" | "not" | "foldr" | "Map" | "lookup" | "Left"
        | "Right" | "Walk" | "Char" | "Bool" | "Maybe" | "Int" | "Prelude" | "any" | "filter"
        | "map" | "init" | "null" | "last" | "unwords" | "otherwise" | "either" | "const"
        | "fmap" | "take" | "max" | "putStrLn" => Ok(Token::CONSTANT(identifier.clone())),
        "as" | "import" | "type" | "case" | "do" | "of" | "if" | "then" | "else" | "let" | "in"
        | "module" | "qualified" | "where" | "LANGUAGE" | "OPTIONS_GHC" | "hiding" | "data"
        | "deriving" => Ok(Token::KEYWORD(identifier.clone())),
        "Bibtex" | "Biblatex" | "Nothing" | "Just" | "Str" | "Space" | "FancyVal" | "BibState"
        | "Many" => Ok(Token::ENTITYTAG(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
