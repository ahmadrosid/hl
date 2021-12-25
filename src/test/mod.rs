
use std::fs;
use crate::lexers::rust;

fn read_input(path: &str) -> Vec<char> {
    let source = fs::read(path).expect(&format!("Filed reading file {}", path));
    return source.iter().map(|c| *c as char).collect::<Vec<_>>();
}

fn read_file(path: &str) -> String {
    let source = fs::read_to_string(path).expect(&format!("Filed reading file {}", path));
    return source;
}

#[test]
fn valid_parse() {
    let input = read_input("src/test/testdata/input/rust.rs");
    let output = read_file("src/test/testdata/output/rust.html");
    let result = rust::render::render_html(input);
    assert_eq!(output, result);
}