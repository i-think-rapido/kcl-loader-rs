
#[path = "../generated/schema.rs"]
mod schema;

use kcl_loader::to_instance;
use schema::Person;
use std::collections::HashMap;




fn main() -> anyhow::Result<()> {
    let result: HashMap<String, Person> = to_instance(vec!["./data/schema.k".to_string()])?;

    println!("{:?}", result);

    Ok(())
}

