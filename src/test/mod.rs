use crate::lexers::*;
use std::fs;

fn read_input(path: &str) -> Vec<char> {
    let source = fs::read(path).expect(&format!("Filed reading file {}", path));
    return source.iter().map(|c| *c as char).collect::<Vec<_>>();
}

fn read_file(path: &str) -> String {
    let source = fs::read_to_string(path).expect(&format!("Filed reading file {}", path));
    return source;
}

#[test]
fn test_bash() {
    let input = read_input("src/test/testdata/input/bash.sh.stub");
    let expected = read_file("src/test/testdata/output/bash.html.stub");
    let actual = bash::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_c() {
    let input = read_input("src/test/testdata/input/c.c.stub");
    let expected = read_file("src/test/testdata/output/c.html.stub");
    let actual = c::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_clojure() {
    let input = read_input("src/test/testdata/input/clojure.clj.stub");
    let expected = read_file("src/test/testdata/output/clojure.html.stub");
    let actual = clojure::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_css() {
    let input = read_input("src/test/testdata/input/css.css.stub");
    let expected = read_file("src/test/testdata/output/css.html.stub");
    let actual = css::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_cuda() {
    let input = read_input("src/test/testdata/input/cuda.cu.stub");
    let expected = read_file("src/test/testdata/output/cuda.html.stub");
    let actual = cuda::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_edn() {
    let input = read_input("src/test/testdata/input/edn.edn.stub");
    let expected = read_file("src/test/testdata/output/edn.html.stub");
    let actual = edn::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_cpp() {
    let input = read_input("src/test/testdata/input/cpp.cpp.stub");
    let expected = read_file("src/test/testdata/output/cpp.html.stub");
    let actual = cpp::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_cs() {
    let input = read_input("src/test/testdata/input/cs.cs.stub");
    let expected = read_file("src/test/testdata/output/cs.html.stub");
    let actual = cs::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_golang() {
    let input = read_input("src/test/testdata/input/golang.go.stub");
    let expected = read_file("src/test/testdata/output/golang.html.stub");
    let actual = go::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_haskell() {
    let input = read_input("src/test/testdata/input/haskell.hs.stub");
    let expected = read_file("src/test/testdata/output/haskell.html.stub");
    let actual = haskell::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_html() {
    let input = read_input("src/test/testdata/input/html.html.stub");
    let expected = read_file("src/test/testdata/output/html.html.stub");
    let actual = html::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_java() {
    let input = read_input("src/test/testdata/input/java.java.stub");
    let expected = read_file("src/test/testdata/output/java.html.stub");
    let actual = java::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_javascript() {
    let input = read_input("src/test/testdata/input/javascript.js.stub");
    let expected = read_file("src/test/testdata/output/javascript.html.stub");
    let actual = javascript::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_json() {
    let input = read_input("src/test/testdata/input/json.json.stub");
    let expected = read_file("src/test/testdata/output/json.html.stub");
    let actual = json::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_lua() {
    let input = read_input("src/test/testdata/input/lua.lua.stub");
    let expected = read_file("src/test/testdata/output/lua.html.stub");
    let actual = lua::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_php() {
    let input = read_input("src/test/testdata/input/php.php.stub");
    let expected = read_file("src/test/testdata/output/php.html.stub");
    let actual = php::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_python() {
    let input = read_input("src/test/testdata/input/python.py.stub");
    let expected = read_file("src/test/testdata/output/python.html.stub");
    let actual = python::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_rust() {
    let input = read_input("src/test/testdata/input/rust.rs.stub");
    let expected = read_file("src/test/testdata/output/rust.html.stub");
    let actual = rust::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_typescript() {
    let input = read_input("src/test/testdata/input/typescript.ts.stub");
    let expected = read_file("src/test/testdata/output/typescript.html.stub");
    let actual = typescript::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_yaml() {
    let input = read_input("src/test/testdata/input/yaml.yml.stub");
    let expected = read_file("src/test/testdata/output/yaml.html.stub");
    let actual = yaml::render::render_html(input);
    assert_eq!(expected, actual);
}
