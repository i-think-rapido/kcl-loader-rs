
use kclvm_ast::ast::{Module, NodeRef, Stmt, BasicType, Type};
use kclvm_parser::parse_file;

use std::{fs::OpenOptions, io::Write};

fn main() {
    let filename = "./data/schema.k";
    let content = load_file(filename);
    let m = get_module(filename, &content);

    let mut file = OpenOptions::new().write(true).create(true).open("./generated/schema.rs").unwrap();

    traverse(&mut file, m.body);
}

fn get_module(filename: &str, src: &str) -> Module{
    parse_file(filename, Some(src.into())).unwrap()    
}


fn traverse(file: &mut dyn Write, v: Vec<NodeRef<Stmt>>) {
    for stmt in v {
        match stmt.node {
            Stmt::Schema(s) => { 
                write!(file, r"#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]").unwrap();
                writeln!(file).unwrap();
                write!(file, "{}", format!("pub struct {:?} {{", s.name.node).replace("\"", "")).unwrap();
                writeln!(file).unwrap();
                traverse(file, s.body);
                write!(file, "}}").unwrap();
                writeln!(file).unwrap();
            },
            Stmt::SchemaAttr(s) => {
                write!(file, "{}", format!("    pub {:?}: ", s.name.node).replace("\"", "")).unwrap();
                if s.is_optional {
                    write!(file, "Option<").unwrap()
                }
                match s.ty.node {
                    Type::Basic(BasicType::Int) => write!(file, "i64").unwrap(),
                    Type::Basic(BasicType::Str) => write!(file, "String").unwrap(),
                    Type::Basic(BasicType::Float) => write!(file, "f64").unwrap(),
                    _ => {}
                }
                if s.is_optional {
                    write!(file, ">").unwrap()
                }
                write!(file, ",").unwrap();
                writeln!(file).unwrap();
            }
            _ => {}
        }
    }
}

fn load_file(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

