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
    let input = read_input("src/test/testdata/input/css.stub");
    let expected = read_file("src/test/testdata/output/css.stub");
    let actual = css::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_golang() {
    let input = read_input("src/test/testdata/input/golang.stub");
    let expected = read_file("src/test/testdata/output/golang.stub");
    let actual = go::render::render_html(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_rust() {
    let input = read_input("src/test/testdata/input/rust.stub");
    let expected = read_file("src/test/testdata/output/rust.stub");
    let actual = rust::render::render_html(input);
    assert_eq!(expected, actual);
}
