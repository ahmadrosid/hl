extern crate yaml_rust;

#[path = "../color.rs"]
mod color;
mod module;
mod render;
mod string;
mod token;

use color::ColorExt;
use std::fs;
use std::io::Write;
use std::path::Path;
use yaml_rust::{yaml::Hash, Yaml, YamlLoader};

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

macro_rules! get_str (
    ($name:ident, $key:ident) => (
pub fn $name(h: &Hash) -> String {
    return match h.get(&Yaml::String(stringify!($key).to_string())) {
        None => String::new(),
        Some(val) => val.as_str().unwrap().to_string()
    }
}
    );
);

get_hash!(get_constant, constant);
get_hash!(get_var, var);
get_hash!(get_keyword, keyword);
get_hash!(get_double_keyword, double_keyword);
get_hash!(get_entity, entity);
get_hash!(get_constant_prefix, constant_prefix);
get_hash!(get_constant_suffix, constant_suffix);
get_hash!(get_var_suffix, var_suffix);
get_hash!(get_entity_prefix, entity_prefix);
get_hash!(get_entity_suffix, entity_suffix);
get_hash!(get_entity_tag, entity_tag);
get_hash!(get_condition, condition);
get_hash!(get_xml_entity_tag, xml_entity_tag);
get_str!(get_multi_line_comment, multi_line_comment);
get_str!(get_multi_line_string, multi_line_string);
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

    let mut token_stub = String::new();
    let mut module_stub = String::new();
    let mut render_stub = String::new();

    if docs.len() == 0 {
        let mut message = String::new();
        message.push_str(&(&format!("Failed processing {} file is empty!", file_path).red()));
        return message;
    }

    match *&docs[0] {
        Yaml::Hash(ref h) => {
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

    let name = get_file_name(file_path);
    update_lexer_mod(
        &name,
        &out_file_path.replace(&format!("/{}", name), "/mod.rs"),
    );
    update_lib_mod(
        &name,
        &out_file_path.replace(&format!("lexers/{}", name), "lib.rs"),
    );

    let mut message = String::new();
    message.push_str(&"Success generate lexer for \"".green());
    message.push_str(&(&get_file_name(file_path)).bold_green());
    message.push_str(&"\" language!\n".green());
    message.push_str(&(&format!("- {}/token.rs\n", out_file_path).cyan()));
    message.push_str(&(&format!("- {}/mod.rs\n", out_file_path).cyan()));
    message.push_str(&(&format!("- {}/render.rs\n", out_file_path).cyan()));
    message
}

fn update_lexer_mod(name: &str, path: &str) {
    let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    let mut source = include_str!("../lexers/mod.rs").to_string();
    if !source.contains(&format!("pub mod {};", name)) {
        source.push_str(&format!("pub mod {};", name));
        if let Err(e) = writeln!(file, "{}", source) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

fn update_lib_mod(name: &str, path: &str) {
    let mut file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
    let mut source = include_str!("../lib.rs").to_string();
    if !source.contains(&format!("{}::", name)) {
        source = source.replace(
            &format!("_ => raw::render::render_html(input),"),
            &format!(
                r#""{}" => {}::render::render_html(input),
                _ => raw::render::render_html(input),"#,
                name, name
            ),
        );

        if let Err(e) = writeln!(file, "{}", source) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
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
