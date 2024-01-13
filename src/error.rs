use std::{
    error::Error,
    fmt::{Debug, Display},
};

macro_rules! format_error_missing_kv {
    ($f:ident, $self:ident) => {
        if let Some(filename) = &$self.file {
            write!(
                $f,
                "Expected key value pair in file `{}` at line {}",
                filename, $self.line
            )
        } else {
            write!($f, "Expected key value pair at line {}", $self.line)
        }
    };
}

macro_rules! impl_trait_for_error {
    ($trait:ident, $error_type:ty) => {
        impl $trait for $error_type {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                format_error_missing_kv!(f, self)
            }
        }
    };
}

pub struct MissingKeyValuePairError {
    pub line: usize,
    pub file: Option<String>,
}

impl Error for MissingKeyValuePairError {}
impl_trait_for_error!(Display, MissingKeyValuePairError);
impl_trait_for_error!(Debug, MissingKeyValuePairError);
