use std::path::Path;

use hl::render_html;
use std::fs::read_to_string;

//cargo run --package hl --example html examples/html.rs
fn main() {
    let file_path = std::env::args().nth(1).unwrap_or(String::new());
    let path = Path::new(&file_path);
    if !path.exists() {
        println!("Require correct file path!");
        std::process::exit(1);
    }

    let input = read_to_string(path).unwrap().chars().collect::<Vec<_>>();
    let result = render_html(input, "rust");
    println!("{}", result)
}