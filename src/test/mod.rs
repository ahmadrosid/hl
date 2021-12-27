use crate::lexers::{css, go, java, javascript, rust, typescript};
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
fn test_css() {
    let input = read_input("src/test/testdata/input/css.css.stub");
    let expected = read_file("src/test/testdata/output/css.html.stub");
    let actual = css::render::render_html(input);
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
