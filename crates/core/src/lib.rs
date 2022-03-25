pub mod lexers;
use crate::lexers::{
    actionscript, bash, c, clojure, cpp, cs, css, cuda, dart, edn, erlang, go, groovy, haskell,
    html, java, javascript, json, kotlin, lua, makefile, markdown, nim, php, python, raw, ruby,
    rust, toml, typescript, vue, yaml, zig,
};

pub fn render_html(input: Vec<char>, lang: &str) -> String {
    match lang {
        "python" => python::render_html(input),
        "clojure" => clojure::render_html(input),
        "cs" => cs::render_html(input),
        "json" => json::render_html(input),
        "lua" => lua::render_html(input),
        "toml" => toml::render_html(input),
        "ruby" => ruby::render_html(input),
        "groovy" => groovy::render_html(input),
        "c" => c::render_html(input),
        "haskell" => haskell::render_html(input),
        "zig" => zig::render_html(input),
        "go" => go::render_html(input),
        "typescript" => typescript::render_html(input),
        "yaml" => yaml::render_html(input),
        "php" => php::render_html(input),
        "erlang" => erlang::render_html(input),
        "cuda" => cuda::render_html(input),
        "edn" => edn::render_html(input),
        "css" => css::render_html(input),
        "makefile" => makefile::render_html(input),
        "bash" => bash::render_html(input),
        "markdown" => markdown::render_html(input),
        "javascript" => javascript::render_html(input),
        "cpp" => cpp::render_html(input),
        "java" => java::render_html(input),
        "nim" => nim::render_html(input),
        "actionscript" => actionscript::render_html(input),
        "html" => html::render_html(input),
        "kotlin" => kotlin::render_html(input),
        "vue" => vue::render_html(input),
        "dart" => dart::render_html(input),
        "rust" => rust::render_html(input),
        _ => raw::render_html(input),
    }
}
