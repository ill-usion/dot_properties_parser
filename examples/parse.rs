use std::{error::Error, path::Path};

use dot_properties_parser::*;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(".common/server.properties");
    let props = parse_properties_file(path, None)?;

    for (k, v) in props {
        println!("{k}: {v}");
    }

    Ok(())
}
