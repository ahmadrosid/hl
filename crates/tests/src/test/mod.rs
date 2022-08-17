use hl_core::lexers::*;

#[test]
fn test_action_script() {
    let input = include_str!("testdata/input/ActionScript.as.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/ActionScript.html.stub");
    let actual = actionscript::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_bash() {
    let input = include_str!("testdata/input/bash.sh.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/bash.html.stub");
    let actual = bash::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_c() {
    let input = include_str!("testdata/input/c.c.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/c.html.stub");
    let actual = c::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_clojure() {
    let input = include_str!("testdata/input/clojure.clj.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/clojure.html.stub");
    let actual = clojure::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_css() {
    let input = include_str!("testdata/input/css.css.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/css.html.stub");
    let actual = css::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_cuda() {
    let input = include_str!("testdata/input/cuda.cu.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/cuda.html.stub");
    let actual = cuda::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_cpp() {
    let input = include_str!("testdata/input/cpp.cpp.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/cpp.html.stub");
    let actual = cpp::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_cs() {
    let input = include_str!("testdata/input/cs.cs.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/cs.html.stub");
    let actual = cs::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_edn() {
    let input = include_str!("testdata/input/edn.edn.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/edn.html.stub");
    let actual = edn::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_erlang() {
    let input = include_str!("testdata/input/erlang.erl.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/erlang.html.stub");
    let actual = erlang::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_golang() {
    let input = include_str!("testdata/input/golang.go.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/golang.html.stub");
    let actual = go::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_groovy() {
    let input = include_str!("testdata/input/Groovy.groovy.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/Groovy.html.stub");
    let actual = groovy::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_haskell() {
    let input = include_str!("testdata/input/haskell.hs.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/haskell.html.stub");
    let actual = haskell::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_html() {
    let input = include_str!("testdata/input/html.html.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/html.html.stub");
    let actual = html::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_java() {
    let input = include_str!("testdata/input/java.java.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/java.html.stub");
    let actual = java::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_javascript() {
    let input = include_str!("testdata/input/javascript.js.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/javascript.html.stub");
    let actual = javascript::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_json() {
    let input = include_str!("testdata/input/json.json.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/json.html.stub");
    let actual = json::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_kotlin() {
    let input = include_str!("testdata/input/kotlin.kt.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/kotlin.html.stub");
    let actual = kotlin::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_lua() {
    let input = include_str!("testdata/input/lua.lua.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/lua.html.stub");
    let actual = lua::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_markdown() {
    let input = include_str!("testdata/input/markdown.md.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/markdown.html.stub");
    let actual = markdown::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_php() {
    let input = include_str!("testdata/input/php.php.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/php.html.stub");
    let actual = php::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_python() {
    let input = include_str!("testdata/input/python.py.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/python.html.stub");
    let actual = python::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_ruby() {
    let input = include_str!("testdata/input/ruby.rb.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/ruby.html.stub");
    let actual = ruby::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_rust() {
    let input = include_str!("testdata/input/rust.rs.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/rust.html.stub");
    let actual = rust::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_toml() {
    let input = include_str!("testdata/input/TOML.toml.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/TOML.html.stub");
    let actual = toml::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_typescript() {
    let input = include_str!("testdata/input/typescript.ts.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/typescript.html.stub");
    let actual = typescript::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_v() {
    let input = include_str!("testdata/input/v.v.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/v.html.stub");
    let actual = v::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_yaml() {
    let input = include_str!("testdata/input/yaml.yml.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/yaml.html.stub");
    let actual = yaml::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_zig() {
    let input = include_str!("testdata/input/zig.zig.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/zig.html.stub");
    let actual = zig::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_nim() {
    let input = include_str!("testdata/input/nim.nim.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/nim.html.stub");
    let actual = nim::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_proto() {
    let input = include_str!("testdata/input/proto.proto.stub")
        .chars()
        .collect::<Vec<char>>();
    let expected = include_str!("testdata/output/proto.html.stub");
    let actual = proto::render_html(input);
    assert_eq!(expected, actual);
}
