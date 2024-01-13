# dot properties parser

[![Tests](https://github.com/ill-usion/dot_properties_parser/actions/workflows/tests.yml/badge.svg)](https://github.com/ill-usion/dot_properties_parser/actions/workflows/tests.yml)

A simple `.properties` file parser. Docs can be found on [Github pages](https://ill-usion.github.io/dot_properties_parser).
### Note:
This only supports reading, for now.

## Getting started
Clone this repo.
```
git clone https://github.com/ill-usion/dot_properties_parser.git
```

Run the example:
```
cargo run --example parse
```


Or you can add the following to your `Cargo.toml`
```toml
[dependencies]
dot_properties_parser = { git = "https://github.com/ill-usion/dot_properties_parser.git" }
```


Take for example the file given in `/.common/server.properties`:
```rust
use std::{error::Error, path::Path};

use dot_properties_parser::*;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(".common/server.properties");
    // Parse the file contents
    let props = parse_properties_file(path, None)?;

    // Iterate through every key value pair and print them
    for (k, v) in props {
        println!("{k}: {v}");
    }

    Ok(())
}
```

If your properties file uses a custom syntax like different delimiters, you can specify your own configuration:
```rust
use std::{error::Error, path::Path};

use dot_properties_parser::*;

fn main() -> Result<(), Box<dyn Error>> {
    let config = DotPropertiesConfig {
        // comments start with `!` instead of `#`
        comment_prefix: "!".to_string(),
        // keys value pairs are separated with `:` instead of `=`
        delimiter: ":".to_string(),
    };

    let path = Path::new(".common/custom.server.properties");
    let props = parse_properties_file(path, Some(config))?;

    for (k, v) in props {
        println!("{k}: {v}");
    }

    Ok(())
}

```

# License
Licensed under [MIT License](https://opensource.org/license/mit/).

# Contribution
You're welcome.