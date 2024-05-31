use super::{application::ToArgVector, ZenityApplication};

#[derive(Debug, Clone, Default)]
pub struct Info {
    /// The body text
    pub text: Option<String>,
    /// Custom label for ok button
    pub ok_label: Option<String>,
    /// Prevent word wrap
    pub no_wrap: bool,
    /// Disable markup support
    pub no_markup: bool,
    /// Show ellipses for texts that are too long to display
    pub ellipsize: bool,
}

impl ZenityApplication for Info {
    type Return = String;

    fn parse(&self, stdout: &str) -> Result<Self::Return, crate::Error> {
        Ok(stdout.to_owned())
    }
}

impl ToArgVector for Info {
    fn to_argv(&self) -> Vec<String> {
        let mut args = vec!["--info".to_string()];
        if let Some(ref text) = self.text {
            args.push(format!("--text={text}"))
        };

        if let Some(ref ok_label) = self.ok_label {
            args.push(format!("--ok-label={ok_label}"))
        };

        if self.no_wrap {
            args.push("--no-wrap".to_string());
        }

        if self.no_markup {
            args.push("--no-markup".to_string())
        }

        if self.ellipsize {
            args.push("--ellipsize".to_string())
        }

        args
    }
}

impl Info {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_ok_label(mut self, ok_label: impl Into<String>) -> Self {
        self.ok_label = Some(ok_label.into());
        self
    }

    pub fn set_no_wrap(mut self) -> Self {
        self.no_wrap = true;
        self
    }

    pub fn set_no_markup(mut self) -> Self {
        self.no_markup = true;
        self
    }

    pub fn set_ellipsize(mut self) -> Self {
        self.ellipsize = true;
        self
    }
}
