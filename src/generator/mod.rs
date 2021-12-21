extern crate yaml_rust;

use yaml_rust::{YamlLoader, Yaml};
use std::io::Write;
use std::path::Path;
use std::fs;

pub mod processor;

pub fn parse(file_path: &str) -> String {
    let content = read_file(file_path);
    let docs = YamlLoader::load_from_str(&content).unwrap();
    let required_key = vec!["base", "constant", "keyword"];

    let mut token = String::new();
    let mut module = String::new();

    match *&docs[0] {
        Yaml::Hash(ref h) => {
            for (k, _v) in h {
                if !required_key.contains(&k.as_str().unwrap()) {
                    println!("Invalid keys: {}", k.as_str().unwrap());
                    std::process::exit(0x01);
                }
            }
            token.push_str(&processor::process_token(h));
            module.push_str(&processor::process_module(h));
        }
        _ => {
            println!("{:?}", &docs[0]);
        }
    }

    let out_file_path = prepare_path(file_path);
    write_file(&token, &out_file_path, &"token.rs");
    write_file(&module, &out_file_path, &"mod.rs");

    module
}

fn read_file(file_path: &str) -> String {
    let file = fs::read(file_path).expect(&format!("Can not read file path: {}", file_path));
    let content = file.iter().map(|c| *c as char).collect::<Vec<_>>();
    String::from_iter(content)
}

fn prepare_path(file_path: &str) -> String {
    let ancestors = Path::new(& file_path).file_name().unwrap();
    let name = ancestors.to_string_lossy().replace(&".yml", "");

    let cwd = std::env::current_dir().unwrap();
    let out_file_path = format!("{}/src/lexers/{}", cwd.to_str().unwrap(), name);
    println!("{}", out_file_path);
    let create_dir = fs::create_dir_all(&out_file_path);
    if !create_dir.is_ok() {
        println!("Failed to create directory for : {}", out_file_path)
    }

    out_file_path
}

fn write_file(content: &String, path: &String, file_name: &str) {
    let mut file = fs::File::create(format!("{}/{}", path, file_name)).unwrap();
    write!(&mut file, "{}", content).unwrap();
}