// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "root" | "rgb" | "rgba" | "calc" | "media" => Ok(Token::CONSTANT(identifier.clone())),
        "after" | "before" | "hover" | "not" | "focus" | "active" | "selection" | "px" | "rem" => {
            Ok(Token::ENTITY(identifier.clone()))
        }
        "important" => Ok(Token::KEYWORD(identifier.clone())),
        "html" | "body" | "div" | "span" | "applet" | "object" | "iframe" | "h1" | "h2" | "h3"
        | "h4" | "h5" | "h6" | "p" | "blockquote" | "button" | "pre" | "a" | "abbr" | "acronym"
        | "address" | "big" | "cite" | "code" | "del" | "dfn" | "em" | "img" | "ins" | "kbd"
        | "q" | "s" | "samp" | "select" | "small" | "strike" | "strong" | "sub" | "sup" | "tt"
        | "var" | "b" | "u" | "i" | "center" | "dl" | "dt" | "dd" | "ol" | "ul" | "li"
        | "fieldset" | "form" | "label" | "legend" | "table" | "caption" | "tbody" | "tfoot"
        | "thead" | "tr" | "th" | "td" | "article" | "canvas" | "embed" | "output" | "ruby"
        | "summary" | "time" | "mark" | "audio" | "videoarticle" | "aside" | "details"
        | "figcaption" | "figure" | "footer" | "header" | "hgroup" | "menu" | "nav" | "section"
        | "video" | "textarea" | "input" => Ok(Token::ENTITYTAG(identifier.clone())),
        _ => Err(String::from("Not a keyword")),
    }
}
