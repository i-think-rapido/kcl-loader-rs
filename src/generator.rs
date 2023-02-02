
use kclvm_ast::ast::{Module, NodeRef, Stmt, BasicType, Type};
use kclvm_parser::parse_file;
use std::{fs::OpenOptions, io::Write};

pub struct Container<T> {
    container: T,
}
impl Container<Module> {
    fn new(m: Module) -> Self {
        Self { container: m }
    }
    pub fn to_schema(&self, filename: &str) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).open(filename).unwrap();
        traverse(&mut file, &self.container.body);
        Ok(())
    }
}

pub fn from_kcl(filename: &str) -> anyhow::Result<Container<Module>>{
    let content = load_file(filename);
    let m = get_module(filename, &content)?;
    Ok(Container::new(m))
}


fn get_module(filename: &str, src: &str) -> anyhow::Result<Module> {
    parse_file(filename, Some(src.into())).map_err(|err| anyhow::anyhow!(err))
}


fn traverse(file: &mut dyn Write, v: &Vec<NodeRef<Stmt>>) {
    for stmt in v {
        match stmt.node {
            Stmt::Schema(ref s) => { 
                write!(file, r"#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]").unwrap();
                writeln!(file).unwrap();
                write!(file, "{}", format!("pub struct {:?} {{", s.name.node).replace("\"", "")).unwrap();
                writeln!(file).unwrap();
                traverse(file, &s.body);
                write!(file, "}}").unwrap();
                writeln!(file).unwrap();
            },
            Stmt::SchemaAttr(ref s) => {
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

