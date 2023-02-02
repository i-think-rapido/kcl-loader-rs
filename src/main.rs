
#[path = "../generated/schema.rs"]
mod schema;

use kclvm_runner::{exec_program, ExecProgramArgs};
use schema::Person;
use std::collections::HashMap;




fn main() {
    let filename = "./data/schema.k";

    let args = ExecProgramArgs {
        k_filename_list: vec![filename.to_string()],
        ..ExecProgramArgs::default()
    };

    if let Ok(result) = exec_program(&args, 1) {
        let content = result.yaml_result;
        let yaml: HashMap<String, Person> = serde_yaml::from_str(&content).unwrap();
        println!("{:?}", yaml);
    }

}

