use anyhow::Result;
use schemars::{schema_for, JsonSchema};
use std::{fs::File, io::Write};

#[allow(dead_code)]
#[derive(JsonSchema)]
struct SchemaTypes {
    points: Vec<Point>,
}

#[allow(dead_code)]
#[derive(JsonSchema, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> Result<()> {
    let schema = schema_for!(SchemaTypes);
    let content = serde_json::to_string_pretty(&schema)?;

    let dir_path = String::from("support/schemas/schema_types");
    let type_name = stringify!(SchemaTypes).to_string();
    let file_name = type_name
        .split("::")
        .last()
        .unwrap()
        .to_string()
        .trim_end_matches('>')
        .to_string()
        + ".json";

    let _ = std::fs::create_dir_all(&dir_path);
    let mut file = File::create(format!("{}/{}", dir_path, file_name))?;
    writeln!(&mut file, "{}", &content)?;

    Ok(())
}
