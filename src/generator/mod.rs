extern crate yaml_rust;

use std::fs;
use std::io::Write;
use std::path::Path;
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

#[path = "../color.rs"]
mod color;
mod module;
mod render;
mod string;
mod token;

macro_rules! get_hash (
    ($name:ident, $key:ident) => (
pub fn $name(h: &Hash) -> Hash {
    let constant = h.get(&Yaml::String(stringify!($key).to_string()));
    let map = Hash::new();
    return match constant {
        None => map,
        Some(val) => {
            return match val.as_hash() {
                None => map,
                Some(h) => h.clone()
            }
        }
    }
}
    );
);

macro_rules! get_bool (
    ($name:ident, $key:ident) => (
pub fn $name(h: &Hash) -> bool {
    return match h.get(&Yaml::String(stringify!($key).to_string())) {
        None => false,
        Some(val) => val.as_bool().unwrap()
    }
}
    );
);

get_hash!(get_constant, constant);
get_hash!(get_var, var);
get_hash!(get_keyword, keyword);
get_hash!(get_double_keyword, double_keyword);
get_hash!(get_entity, entity);
get_hash!(get_entity_prefix, entity_prefix);
get_hash!(get_entity_suffix, entity_suffix);
get_hash!(get_entity_tag, entity_tag);
get_hash!(get_condition, condition);
get_hash!(get_xml_entity_tag, xml_entity_tag);
get_bool!(slash_star_comment_enable, slash_star_comment);
get_bool!(xml_comment_enable, xml_comment);
get_bool!(hashtag_comment_enable, hashtag_comment);
get_bool!(bracket_dash_comment_enable, bracket_dash_comment);

pub trait ConditionExt {
    fn get_some_condition(&self, key: &str) -> Option<Yaml>;
}

impl ConditionExt for Hash {
    fn get_some_condition(&self, key: &str) -> Option<Yaml> {
        return match get_condition(self).get(&Yaml::String(key.to_string())) {
            None => None,
            Some(val) => Some(val.clone()),
        };
    }
}

pub fn parse(file_path: &str) -> String {
    let content = read_file(file_path);
    let docs = YamlLoader::load_from_str(&content).unwrap();
    let required_key = vec![
        "constant",
        "single_constant",
        "keyword",
        "single_keyword",
        "double_keyword",
        "entity",
        "entity_tag",
        "xml_entity_tag",
        "prefix",
        "bracket_dash_comment",
        "hashtag_comment",
        "slash_star_comment",
        "xml_comment",
        "condition",
        "var",
        "entity_prefix",
        "entity_suffix",
        "raw",
    ];

    let mut token_stub = String::new();
    let mut module_stub = String::new();
    let mut render_stub = String::new();

    if docs.len() == 0 {
        let mut message = String::new();
        message.push_str(&color::red(&format!(
            "Failed processing {} file is empty!",
            file_path
        )));
        return message;
    }

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
            render_stub.push_str(&render::generate_render_html(h, get_file_name(file_path)));
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
    message.push_str(&color::green("Success generate lexer for \""));
    message.push_str(&color::bold_green(&get_file_name(file_path)));
    message.push_str(&color::green("\" language!\n"));
    message.push_str(&color::cyan(&format!("- {}/token.rs\n", out_file_path)));
    message.push_str(&color::cyan(&format!("- {}/mod.rs\n", out_file_path)));
    message.push_str(&color::cyan(&format!("- {}/render.rs\n", out_file_path)));
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

fn get_file_name(file_path: &str) -> String {
    let ancestors = Path::new(&file_path).file_name().unwrap();
    ancestors.to_string_lossy().replace(&".yml", "")
}
