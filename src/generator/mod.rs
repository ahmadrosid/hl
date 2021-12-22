extern crate yaml_rust;

use std::fmt::format;
use yaml_rust::{YamlLoader, Yaml, yaml::Hash};
use std::io::Write;
use std::path::Path;
use std::fs;

mod module;
mod token;
mod render;

pub fn parse(file_path: &str) -> String {
    let content = read_file(file_path);
    let docs = YamlLoader::load_from_str(&content).unwrap();
    let required_key = vec!["base", "constant", "keyword", "skip", "entity"];

    let mut token_stub = String::new();
    let mut module_stub = String::new();
    let mut render_stub = String::new();

    match *&docs[0] {
        Yaml::Hash(ref h) => {
            for (k, _v) in h {
                if !required_key.contains(&k.as_str().unwrap()) {
                    println!("Invalid key: {}", k.as_str().unwrap());
                    std::process::exit(0x01);
                }
            }
            token_stub.push_str(&token::generate_token(h));
            module_stub.push_str(&module::generate_module(h));
            render_stub.push_str(&render::generate_html(h, get_file_name(file_path)));
        }
        _ => {
            println!("{:?}", &docs[0]);
        }
    }

    let out_file_path = prepare_path(file_path);
    write_file(&token_stub, &out_file_path, &"token.rs");
    write_file(&module_stub, &out_file_path, &"mod.rs");
    write_file(&render_stub, &out_file_path, &"render.rs");

    let mut message = String::new();
    message.push_str("Success generate lexer for ");
    message.push_str(&format!("\"{}\" language!\n", get_file_name(file_path)));
    message.push_str(&format!("{}/token.rs\n", out_file_path));
    message.push_str(&format!("{}/mod.rs\n", out_file_path));
    message.push_str(&format!("{}/render.rs\n", out_file_path));
    message
}

fn read_file(file_path: &str) -> String {
    let file = fs::read(file_path).expect(&format!("Can not read file path: {}", file_path));
    let content = file.iter().map(|c| *c as char).collect::<Vec<_>>();
    String::from_iter(content)
}

fn prepare_path(file_path: &str) -> String {
    let file_name = get_file_name(file_path);
    let cwd = std::env::current_dir().unwrap();
    let out_file_path = format!("{}/src/lexers/{}", cwd.to_str().unwrap(), file_name);

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

fn get_base(h: &Hash) -> &Hash {
    h.get(&Yaml::String("base".to_string()))
        .unwrap()
        .as_hash()
        .unwrap()
}

fn get_constant(h: &Hash) -> &Hash {
    h.get(&Yaml::String("constant".to_string()))
        .unwrap()
        .as_hash()
        .unwrap()
}

fn get_keyword(h: &Hash) -> &Hash {
    h.get(&Yaml::String("keyword".to_string()))
        .unwrap()
        .as_hash()
        .unwrap()
}

fn get_entity(h: &Hash) -> &Hash {
    h.get(&Yaml::String("entity".to_string()))
        .unwrap()
        .as_hash()
        .unwrap()
}

fn get_skip(h: &Hash) -> &Hash {
    h.get(&Yaml::String("skip".to_string()))
        .unwrap()
        .as_hash()
        .unwrap()
}

fn get_file_name(file_path: &str) -> String {
    let ancestors = Path::new(& file_path).file_name().unwrap();
    ancestors.to_string_lossy().replace(&".yml", "")
}
