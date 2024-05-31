use super::{application::ToArgVector, ZenityApplication};

#[derive(Debug, Clone, Default)]
pub struct Entry {
    /// The body text
    pub text: Option<String>,
    /// Label for the entry
    pub entry_text: Option<String>,
    /// Prevent word wrap
    pub hide_text: bool,
}

impl ZenityApplication for Entry {
    type Return = String;

    fn parse(&self, stdout: &str) -> Result<Self::Return, crate::Error> {
        Ok(stdout.to_owned())
    }
}

impl ToArgVector for Entry {
    fn to_argv(&self) -> Vec<String> {
        let mut args = vec!["--entry".to_string()];
        if let Some(ref text) = self.text {
            args.push(format!("--text={text}"))
        };

        if let Some(ref entry_text) = self.entry_text {
            args.push(format!("--entry-text={entry_text}"))
        };

        if self.hide_text {
            args.push("--hide-text".to_string());
        }

        args
    }
}

impl Entry {
    pub fn new() -> Self {
        Default::default()
    }

    /// Override default input label.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Prefill the input with the given text.
    pub fn with_entry_text(mut self, entry_text: impl Into<String>) -> Self {
        self.entry_text = Some(entry_text.into());
        self
    }

    /// Hide the content of the text input, as for a password input.
    pub fn set_hide_text(mut self) -> Self {
        self.hide_text = true;
        self
    }
}
