use std::{error::Error, path::Path};

use dot_properties_parser::*;

fn main() -> Result<(), Box<dyn Error>> {
    let config = DotPropertiesConfig {
        comment_prefix: "!".to_string(),
        delimiter: ":".to_string(),
    };
    let path = Path::new(".common/custom.server.properties");
    let props = parse_properties_file(path, Some(config))?;

    for (k, v) in props {
        println!("{k}: {v}");
    }

    Ok(())
}
