use std::fmt::Display;

/// Represents a generic argument. For use with [crate::ZenityDialog::with_additional_arg], which allows
/// you to pass in arguments that aren't currently supported statically. See [From] implementations for
/// methods of constructing.
#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    name: String,
    value: Option<String>,
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.name.starts_with("--") {
            true => &self.name[2..],
            false => &self.name,
        };
        if let Some(ref value) = self.value {
            write!(f, "--{}={}", name, value)
        } else {
            write!(f, "--{}", name)
        }
    }
}

impl From<String> for Arg {
    fn from(value: String) -> Self {
        Self {
            name: value,
            value: None,
        }
    }
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        Self {
            name: value.into(),
            value: None,
        }
    }
}

impl From<(String, String)> for Arg {
    fn from(value: (String, String)) -> Self {
        Self {
            name: value.0,
            value: Some(value.1),
        }
    }
}

impl From<(&str, String)> for Arg {
    fn from(value: (&str, String)) -> Self {
        Self {
            name: value.0.into(),
            value: Some(value.1),
        }
    }
}

impl From<(&str, &str)> for Arg {
    fn from(value: (&str, &str)) -> Self {
        Self {
            name: value.0.into(),
            value: Some(value.1.into()),
        }
    }
}
