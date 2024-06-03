use super::{application::ToArgVector, ZenityApplication};

/// Configuration for a dialog that warns the user of an error.
#[derive(Debug, Clone, Default)]
pub struct Error {
    /// The body text
    pub text: Option<String>,
    /// Label for the entry
    pub no_wrap: bool,
    /// Prevent word wrap
    pub no_markup: bool,
}

impl ZenityApplication for Error {
    type Return = String;

    fn parse(&self, stdout: &str) -> Result<Self::Return, crate::Error> {
        Ok(stdout.to_owned())
    }
}

impl ToArgVector for Error {
    fn to_argv(&self) -> Vec<String> {
        let mut args = vec!["--error".to_string()];
        if let Some(ref text) = self.text {
            args.push(format!("--text={text}"))
        };

        if self.no_wrap {
            args.push("--no-wrap".to_string());
        }

        if self.no_markup {
            args.push("--no-markup".to_string());
        }

        args
    }
}

impl Error {
    /// The default settings.
    pub fn new() -> Self {
        Default::default()
    }

    /// Override default input label.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Prefill the input with the given text.
    pub fn set_no_wrap(mut self) -> Self {
        self.no_wrap = true;
        self
    }

    /// Hide the content of the text input, as for a password input.
    pub fn set_no_markup(mut self) -> Self {
        self.no_markup = true;
        self
    }
}
