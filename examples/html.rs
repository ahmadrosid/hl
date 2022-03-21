use std::path::Path;

use hl::render_html;
use std::fs::read_to_string;

// cargo run --package hl --example html examples/html.rs
fn main() {
    let file_path = std::env::args().nth(1).unwrap_or(String::new());
    let language_flag = std::env::args().nth(2).unwrap_or(String::new());
    let lang = std::env::args().nth(3).unwrap_or(String::from("raw"));

    let path = Path::new(&file_path);
    if !path.exists() {
        println!("File path not found! {}", &file_path);
        std::process::exit(1);
    }

    if language_flag.is_empty() {
        println!("Language flag is required! Example: -l javascript");
        std::process::exit(1);
    }

    let input = read_to_string(path).unwrap().chars().collect::<Vec<_>>();
    let result = render_html(input, &lang);
    println!("{}", result)
}
