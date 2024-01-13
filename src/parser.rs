use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::error::*;
use crate::DotPropertiesConfig;

/// A struct that holds a value to a key
pub struct PropertyValue(pub String);

impl PropertyValue {
    /// Parses the property value to a certain type.
    /// # Examples:
    /// ```
    /// use dot_properties_parser::PropertyValue;
    /// let property = PropertyValue("2024".to_string());
    ///
    /// if let Some(value) = property.try_as::<i32>() {
    ///     println!("Got value: {}", value);
    /// }
    /// ```
    pub fn try_as<T>(&self) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.0.parse().ok()
    }

    /// Parses the property value to a certain type ignoring error handling.
    ///
    /// Use this when you're sure the underlying value will have no problems parsing.
    /// # Examples:
    /// ```
    /// use dot_properties_parser::PropertyValue;
    /// let property = PropertyValue("2024".to_string());
    ///
    /// let value = property.value_as::<i32>();
    /// dbg!(value); // value = 2024
    /// ```
    pub fn value_as<T>(&self) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.0.parse().unwrap()
    }
}

impl AsRef<str> for PropertyValue {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for PropertyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Parses a `.properties` file.
/// # Arguments
///
/// * `path` - Path to the `.properties` file.
/// * `config` - Parsing config, pass `None` if you want to use the default configuration.
///
/// # Returns
///
/// A `Result` containing a `HashMap` of the parsed data.
///
/// # Examples:
/// ```
/// use std::path::Path;
/// use dot_properties_parser::parse_properties_file;
/// 
/// let props = parse_properties_file(Path::new(".common/server.properties"), None).unwrap();
/// for (k, v) in props {
///     println!("{}: {}", k, v);
/// }
/// ```
pub fn parse_properties_file(
    path: &Path,
    config: Option<DotPropertiesConfig>,
) -> Result<HashMap<String, PropertyValue>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config = config.unwrap_or_default();

    let mut properties: HashMap<String, PropertyValue> = HashMap::new();
    for (line_num, line) in reader.lines().flatten().enumerate() {
        if line.is_empty() || line.starts_with(&config.comment_prefix) {
            continue;
        }

        let (key, val) = line.split_once(&config.delimiter).ok_or(
            MissingKeyValuePairError {
                line: line_num + 1,
                file: path
                    .file_name()
                    .map(|s| s.to_str().unwrap().to_string()),
            },
        )?;

        let val = PropertyValue(val.to_string());

        properties.insert(key.to_string(), val);
    }

    Ok(properties)
}
