use hl_core::lexers::*;

macro_rules! lexer_tests {
    ($(($name:ident, $lang:ident, $input:expr, $output:expr)),* $(,)?) => {
        $(
            #[test]
            fn $name() {
                let input = include_str!(concat!("testdata/input/", $input))
                    .chars()
                    .collect::<Vec<char>>();
                let expected = include_str!(concat!("testdata/output/", $output));
                let actual = $lang::render_html(input);
                assert_eq!(expected, actual);
            }
        )*
    };
}

lexer_tests! {
    (test_action_script, actionscript, "ActionScript.as.stub", "ActionScript.html.stub"),
    (test_bash, bash, "bash.sh.stub", "bash.html.stub"),
    (test_c, c, "c.c.stub", "c.html.stub"),
    (test_clojure, clojure, "clojure.clj.stub", "clojure.html.stub"),
    (test_css, css, "css.css.stub", "css.html.stub"),
    (test_cpp, cpp, "cpp.cpp.stub", "cpp.html.stub"),
    (test_cs, cs, "cs.cs.stub", "cs.html.stub"),
    (test_edn, edn, "edn.edn.stub", "edn.html.stub"),
    (test_erlang, erlang, "erlang.erl.stub", "erlang.html.stub"),
    (test_golang, go, "golang.go.stub", "golang.html.stub"),
    (test_groovy, groovy, "Groovy.groovy.stub", "Groovy.html.stub"),
    (test_haskell, haskell, "haskell.hs.stub", "haskell.html.stub"),
    (test_html, html, "html.html.stub", "html.html.stub"),
    (test_java, java, "java.java.stub", "java.html.stub"),
    (test_javascript, javascript, "javascript.js.stub", "javascript.html.stub"),
    (test_json, json, "json.json.stub", "json.html.stub"),
    (test_kotlin, kotlin, "kotlin.kt.stub", "kotlin.html.stub"),
    (test_lua, lua, "lua.lua.stub", "lua.html.stub"),
    (test_markdown, markdown, "markdown.md.stub", "markdown.html.stub"),
    (test_php, php, "php.php.stub", "php.html.stub"),
    (test_python, python, "python.py.stub", "python.html.stub"),
    (test_ruby, ruby, "ruby.rb.stub", "ruby.html.stub"),
    (test_rust, rust, "rust.rs.stub", "rust.html.stub"),
    (test_toml, toml, "TOML.toml.stub", "TOML.html.stub"),
    (test_typescript, typescript, "typescript.ts.stub", "typescript.html.stub"),
    (test_vue, vue, "vue.vue.stub", "vue.html.stub"),
    (test_v, v, "v.v.stub", "v.html.stub"),
    (test_yaml, yaml, "yaml.yml.stub", "yaml.html.stub"),
    (test_zig, zig, "zig.zig.stub", "zig.html.stub"),
    (test_nim, nim, "nim.nim.stub", "nim.html.stub"),
    (test_proto, proto, "proto.proto.stub", "proto.html.stub"),
}
