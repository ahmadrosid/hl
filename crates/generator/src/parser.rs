extern crate yaml_rust;
use crate::color::ColorExt;
use crate::module;
use crate::render;
use crate::token;

use std::fs::{create_dir_all, read, File, OpenOptions};
use std::io::{Read, Write};
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

macro_rules! get_string_vec (
    ($name:ident, $key:ident) => (
pub fn $name(h: &Hash) -> Vec<&str> {
    let constant = h.get(&Yaml::String(stringify!($key).to_string()));
    let data = Vec::new();
    return match constant {
        None => data,
        Some(val) => {
            return match val.as_str() {
                None => data,
                Some(val) => val.split(",").collect::<Vec<&str>>()
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

macro_rules! get_string (
    ($name:ident, $key:ident) => (
pub fn $name(h: &Hash) -> String {
    return match h.get(&Yaml::String(stringify!($key).to_string())) {
        None => String::new(),
        Some(val) => val.as_str().unwrap().to_string()
    }
}
    );
);

get_string_vec!(get_constant, constant);
get_string_vec!(get_entity, entity);
get_string_vec!(get_entity_tag, entity_tag);
get_string_vec!(get_keyword, keyword);
get_string_vec!(get_xml_entity_tag, xml_entity_tag);
get_string_vec!(get_var, var);

get_hash!(get_double_keyword, double_keyword);
get_hash!(get_constant_prefix, constant_prefix);
get_hash!(get_constant_suffix, constant_suffix);
get_hash!(get_var_prefix, var_prefix);
get_hash!(get_var_suffix, var_suffix);
get_hash!(get_entity_prefix, entity_prefix);
get_hash!(get_entity_suffix, entity_suffix);
get_hash!(get_condition, condition);

get_string!(get_multi_line_comment, multi_line_comment);
get_string!(get_multi_line_string, multi_line_string);
get_bool!(bracket_dash_comment_enable, bracket_dash_comment);

pub trait ConditionExt {
    fn check_condition(&self, key: &str) -> Option<Yaml>;
}

impl ConditionExt for Hash {
    fn check_condition(&self, key: &str) -> Option<Yaml> {
        return match get_condition(self).get(&Yaml::String(key.to_string())) {
            None => None,
            Some(val) => Some(val.clone()),
        };
    }
}

#[allow(dead_code)]
fn debug_val(data: &Hash, key: &Yaml) -> String {
    if data.get(key).is_none() {
        std::process::exit(0);
    }

    let xml_tag = data
        .get(key)
        .unwrap()
        .as_hash()
        .unwrap()
        .values()
        .collect::<Vec<_>>();

    let mut values = vec![];
    for tag in xml_tag.iter() {
        values.push(tag.as_str().unwrap());
    }
    println!("\nconstant: \"{}\"\n", values.join(","));
    return values.join(",");
}

#[allow(dead_code)]
fn refactor_yaml(h: &Hash, file_path: &str) {
    let k = Yaml::String("var".to_string());
    let keyword = debug_val(h, &k);
    let mut out_str = String::new();
    let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
    let mut dh = h.clone();
    dh.remove(&k).unwrap();
    dh.insert(k, Yaml::String(keyword));
    emitter.dump(&Yaml::Hash(dh)).unwrap();
    let name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
    out_str = out_str.replace("---\n", "");
    write_file(&out_str, &"rules".to_string(), name);
    println!("{}\n", out_str);
    std::process::exit(0)
}

pub fn parse(file_path: &str, output_path: &str) -> String {
    let content = read_file(file_path);
    let docs = YamlLoader::load_from_str(&content).unwrap();

    let mut token_stub = String::new();
    let mut module_stub = String::new();
    let mut render_stub = String::new();

    if docs.len() == 0 {
        let mut message = String::new();
        message.push_str(&format!("Failed processing {} file is empty!", file_path).red());
        return message;
    }

    match *&docs[0] {
        Yaml::Hash(ref h) => {
            token_stub.push_str(&token::generate_token(h));
            module_stub.push_str(&module::generate_module(h));
            render_stub.push_str(&render::generate_html(h, get_file_name(file_path)));
        }
        _ => {
            println!("{:?}", &docs[0]);
        }
    }

    let out_file_path = prepare_path(file_path, output_path);
    module_stub.push_str(&token_stub);

    write_file(&module_stub, &out_file_path, &"mod.rs");
    write_file(&render_stub, &out_file_path, &"render.rs");

    let name = get_file_name(file_path);
    update_lexer_mod(
        &name,
        &out_file_path.replace(&format!("lexers/{}", name), "lexers/mod.rs"),
    );
    update_lib_mod(
        &name,
        &out_file_path.replace(&format!("lexers/{}", name), "lib.rs"),
    );

    let mut message = String::new();
    message.push_str(&"Success generate lexer for \"".green());
    message.push_str(&(&get_file_name(file_path)).bold_green());
    message.push_str(&"\" language!\n".green());
    message.push_str(&format!("- {}/token.rs\n", out_file_path).cyan());
    message.push_str(&format!("- {}/mod.rs\n", out_file_path).cyan());
    message.push_str(&format!("- {}/render.rs\n", out_file_path).cyan());
    message
}

fn update_lexer_mod(name: &str, path: &str) {
    let dir_path = path.clone().replace("/mod.rs", "").to_owned();
    let file_name = "mod.rs";
    if !Path::new(&path).exists() {
        create_dir_all(&dir_path).unwrap();
        write_file(
            &include_str!("stub/base_lexer_mod.stub").replace("raw", name),
            &dir_path,
            &file_name,
        );
        return;
    }

    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    if !source.contains(&format!("pub mod {};", name)) {
        let mut content = format!("pub mod {};\n", name);
        content.push_str(&source);
        write_file(&content, &dir_path, &file_name);
    }
}

fn update_lib_mod(name: &str, path: &str) {
    let dir_path = path.clone().replace("/lib.rs", "").to_owned();
    let file_name = "lib.rs";
    if !Path::new(path).exists() {
        create_dir_all(&dir_path).unwrap();
        write_file(
            &include_str!("stub/base_lib_mod.stub")
                .replace("::{", &format!("{},", name))
                .replace("name", name),
            &dir_path,
            &file_name,
        );
        return;
    }

    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();
    if !source.contains(&format!("{}::", name)) {
        source = source
            .replace(
                "_ => raw::render::html(input),",
                &format!(
                    r#""{}" => {}::render::html(input),
                _ => raw::render::html(input),"#,
                    name, name
                ),
            )
            .replace("::{", &format!("::{{ {},", name));

        write_file(&source, &dir_path, &file_name);
    }
}

fn read_file(file_path: &str) -> String {
    let file = read(file_path).expect(&format!("Can not read file path: {}", file_path));
    let content = file.iter().map(|c| *c as char).collect::<Vec<_>>();
    String::from_iter(content)
}

fn prepare_path(file_path: &str, output_path: &str) -> String {
    let file_name = get_file_name(file_path);
    let out_file_path = format!("{}/src/lexers/{}", output_path, file_name);
    create_dir_all(Path::new(&out_file_path)).unwrap();
    out_file_path
}

fn write_file(content: &String, path: &String, file_name: &str) {
    let path = format!("{}/{}", path, file_name);
    let mut file = File::create(path).unwrap();
    write!(&mut file, "{}", content).unwrap();
}

fn get_file_name(file_path: &str) -> String {
    let ancestors = Path::new(&file_path).file_name().unwrap();
    ancestors.to_string_lossy().replace(&".yml", "")
}
