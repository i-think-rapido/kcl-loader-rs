
#[path = "./src/lib.rs"]
mod kcl_loader;

use kcl_loader::*;

fn main() -> anyhow::Result<()> {
    let filename = "./data/schema.k";
    let schema = "./generated/schema.rs";

    from_kcl(filename)?.to_schema(schema)
}

