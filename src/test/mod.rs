use crate::lexers::{css, go, rust};
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
    let input = read_input("src/test/testdata/input/css.css");
    let expected = read_file("src/test/testdata/output/css.html");
    let actual = css::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_golang() {
    let input = read_input("src/test/testdata/input/golang.go");
    let expected = read_file("src/test/testdata/output/golang.html");
    let actual = go::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_rust() {
    let input = read_input("src/test/testdata/input/rust.rs");
    let expected = read_file("src/test/testdata/output/rust.html");
    let actual = rust::render::render_html(input);
    assert_eq!(expected, actual);
}
