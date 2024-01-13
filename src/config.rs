
/// Represents parsing configuration
pub struct DotPropertiesConfig {
    /// Separates the key and value.
    pub delimiter: String,
    /// Denotes the character used at the beginning of a comment.
    pub comment_prefix: String,
}

impl Default for DotPropertiesConfig {
    fn default() -> Self {
        Self {
            delimiter: "=".to_string(),
            comment_prefix: "#".to_string(),
        }
    }
}
