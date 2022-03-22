// ---- DON'T EDIT! THIS IS AUTO GENERATED CODE ---- //
use crate::lexers::Token;

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        "true" => Ok(Token::CONSTANT(identifier.to_vec())),
        "false" => Ok(Token::CONSTANT(identifier.to_vec())),
        "null" => Ok(Token::CONSTANT(identifier.to_vec())),
        "string" => Ok(Token::CONSTANT(identifier.to_vec())),
        "array" => Ok(Token::CONSTANT(identifier.to_vec())),
        "html" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "body" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "div" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "span" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "applet" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "object" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "iframe" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "h1" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "h2" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "h3" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "h4" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "h5" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "h6" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "p" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "blockquote" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "button" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "pre" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "a" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "abbr" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "acronym" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "address" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "big" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "cite" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "code" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "del" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "dfn" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "em" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "img" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "ins" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "kbd" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "q" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "s" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "samp" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "select" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "small" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "strike" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "strong" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "sub" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "sup" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "script" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "tt" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "var" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "b" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "u" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "i" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "center" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "dl" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "dt" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "dd" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "ol" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "ul" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "li" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "fieldset" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "form" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "label" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "legend" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "table" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "caption" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "tbody" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "tfoot" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "thead" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "tr" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "th" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "td" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "article" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "canvas" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "embed" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "output" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "ruby" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "summary" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "time" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "mark" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "audio" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "videoarticle" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "aside" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "details" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "figcaption" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "figure" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "footer" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "header" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "hgroup" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "menu" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "nav" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "section" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "video" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "textarea" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "input" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "hr" => Ok(Token::ENTITYTAG(identifier.to_vec())),
        "abstract" => Ok(Token::KEYWORD(identifier.to_vec())),
        "as" => Ok(Token::KEYWORD(identifier.to_vec())),
        "break" => Ok(Token::KEYWORD(identifier.to_vec())),
        "and" => Ok(Token::KEYWORD(identifier.to_vec())),
        "callable" => Ok(Token::KEYWORD(identifier.to_vec())),
        "case" => Ok(Token::KEYWORD(identifier.to_vec())),
        "catch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "clone" => Ok(Token::KEYWORD(identifier.to_vec())),
        "const" => Ok(Token::KEYWORD(identifier.to_vec())),
        "continue" => Ok(Token::KEYWORD(identifier.to_vec())),
        "declare" => Ok(Token::KEYWORD(identifier.to_vec())),
        "do" => Ok(Token::KEYWORD(identifier.to_vec())),
        "echo" => Ok(Token::KEYWORD(identifier.to_vec())),
        "else" => Ok(Token::KEYWORD(identifier.to_vec())),
        "enddeclare" => Ok(Token::KEYWORD(identifier.to_vec())),
        "endfor" => Ok(Token::KEYWORD(identifier.to_vec())),
        "endforeach" => Ok(Token::KEYWORD(identifier.to_vec())),
        "endswitch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "endwhile" => Ok(Token::KEYWORD(identifier.to_vec())),
        "eval" => Ok(Token::KEYWORD(identifier.to_vec())),
        "finally" => Ok(Token::KEYWORD(identifier.to_vec())),
        "for" => Ok(Token::KEYWORD(identifier.to_vec())),
        "foreach" => Ok(Token::KEYWORD(identifier.to_vec())),
        "global" => Ok(Token::KEYWORD(identifier.to_vec())),
        "goto" => Ok(Token::KEYWORD(identifier.to_vec())),
        "if" => Ok(Token::KEYWORD(identifier.to_vec())),
        "implements" => Ok(Token::KEYWORD(identifier.to_vec())),
        "include_once" => Ok(Token::KEYWORD(identifier.to_vec())),
        "instanceof" => Ok(Token::KEYWORD(identifier.to_vec())),
        "insteadof" => Ok(Token::KEYWORD(identifier.to_vec())),
        "interface" => Ok(Token::KEYWORD(identifier.to_vec())),
        "namespace" => Ok(Token::KEYWORD(identifier.to_vec())),
        "new" => Ok(Token::KEYWORD(identifier.to_vec())),
        "or" => Ok(Token::KEYWORD(identifier.to_vec())),
        "private" => Ok(Token::KEYWORD(identifier.to_vec())),
        "protected" => Ok(Token::KEYWORD(identifier.to_vec())),
        "public" => Ok(Token::KEYWORD(identifier.to_vec())),
        "require" => Ok(Token::KEYWORD(identifier.to_vec())),
        "require_once" => Ok(Token::KEYWORD(identifier.to_vec())),
        "return" => Ok(Token::KEYWORD(identifier.to_vec())),
        "static" => Ok(Token::KEYWORD(identifier.to_vec())),
        "switch" => Ok(Token::KEYWORD(identifier.to_vec())),
        "throw" => Ok(Token::KEYWORD(identifier.to_vec())),
        "try" => Ok(Token::KEYWORD(identifier.to_vec())),
        "use" => Ok(Token::KEYWORD(identifier.to_vec())),
        "class" => Ok(Token::KEYWORD(identifier.to_vec())),
        "default" => Ok(Token::KEYWORD(identifier.to_vec())),
        "elseif" => Ok(Token::KEYWORD(identifier.to_vec())),
        "endif" => Ok(Token::KEYWORD(identifier.to_vec())),
        "extends" => Ok(Token::KEYWORD(identifier.to_vec())),
        "function" => Ok(Token::KEYWORD(identifier.to_vec())),
        "include" => Ok(Token::KEYWORD(identifier.to_vec())),
        "print" => Ok(Token::KEYWORD(identifier.to_vec())),
        "trait" => Ok(Token::KEYWORD(identifier.to_vec())),
        "while" => Ok(Token::KEYWORD(identifier.to_vec())),
        "xor" => Ok(Token::KEYWORD(identifier.to_vec())),
        "yield" => Ok(Token::KEYWORD(identifier.to_vec())),
        _ => Err(String::from("Not a keyword")),
    }
}