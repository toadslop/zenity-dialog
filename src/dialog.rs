mod application;
mod calendar;
mod entry;
mod error;
mod info;

use crate::Arg;
pub use dialog::application::ZenityApplication;

#[cfg(feature = "calendar")]
pub use dialog::calendar::{Calendar, Month};
#[cfg(feature = "entry")]
pub use dialog::entry::Entry;
#[cfg(feature = "error")]
pub use dialog::error::Error;
#[cfg(feature = "info")]
pub use dialog::info::Info;
use std::{fmt::Display, io, path::PathBuf, process::Command, time::Duration};

/// The configuration for a Zenity dialog.
#[derive(Debug, Clone, PartialEq)]
pub struct ZenityDialog<T = Info>
where
    T: ZenityApplication,
{
    /// The type of dialog to display
    pub application: T,
    /// The title displayed at the top of the dialog
    pub title: Option<String>,
    /// Override for default icon
    pub icon: Option<Icon>,
    /// Override default width of dialog
    pub width: Option<usize>,
    /// Override default height of dialog
    pub height: Option<usize>,
    /// Duration after which the dialog automatically closes
    pub timeout: Option<Duration>,
    /// Provide extra hint text to the user.
    pub modal_hint: Option<String>,
    additional_args: Vec<String>,
}

impl<T> Default for ZenityDialog<T>
where
    T: ZenityApplication,
{
    fn default() -> Self {
        Self {
            application: T::default(),
            title: Default::default(),
            icon: Default::default(),
            width: Default::default(),
            height: Default::default(),
            timeout: Default::default(),
            modal_hint: Default::default(),
            additional_args: Default::default(),
        }
    }
}

// TODO: refactor so extra button has its own variant for response
// TODO: ensure that the extra button variant only is only returnable when an extra button is present

impl<T> ZenityDialog<T>
where
    T: ZenityApplication + Default,
{
    /// Zenity returns zero when users select an afirmitive response.
    const SUCCESS_CODE: i32 = 0;
    /// Zenity returns 256 when users select a negative response.
    const ERROR_CODE_1: i32 = 256;
    const ERROR_CODE_2: i32 = 1;

    /// Construct a new Zenity instance. It expects an [Application], which determines which
    /// kind of dialog will be displayed.
    pub fn new(application: T) -> Self {
        Self {
            application,
            ..Default::default()
        }
    }

    /// Provide a custom title for the dialog.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Override the default icon.
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set a specific width for the dialog.
    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Set a specific height for the dialog.
    pub fn with_height(mut self, height: usize) -> Self {
        self.height = Some(height);
        self
    }

    /// Make the dialog close automatically after the duration has passed.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Render an extra button with the provided text as a label.
    pub fn with_extra_button(
        self,
        extra_button_label: impl Into<String>,
    ) -> ZenityDialogExtButton<T> {
        ZenityDialogExtButton {
            inner: self,
            extra_button_label: extra_button_label.into(),
        }
    }

    /// Render a hint displaying the provided text.
    pub fn with_modal_hint(mut self, modal_hint: impl Into<String>) -> Self {
        self.modal_hint = Some(modal_hint.into());
        self
    }

    /// Attach an additional custom argument. Used to handle arguments that aren't currently statically
    /// supported. Use at your own risk. Note that this function will automatically prepend -- to the argument
    /// so there is no need to provide it. However, if you do provide it, it will still work.
    pub fn with_additional_arg(mut self, arg: impl Into<Arg>) -> Self {
        let arg: Arg = arg.into();
        self.additional_args.push(arg.to_string());
        self
    }

    /// Like `with_additional_arg`, but takes a [Vec<Arg>]
    pub fn with_additional_args(mut self, args: Vec<Arg>) -> Self {
        self.additional_args
            .append(&mut args.iter().map(Arg::to_string).collect::<Vec<String>>());
        self
    }

    /// Convert the settings into an argument vector.
    fn get_argv(&mut self) -> Vec<String> {
        let mut args = self.application.to_argv();

        if let Some(ref title) = self.title {
            args.push(format!("--title={title}"));
        }

        if let Some(ref icon) = self.icon {
            args.push(format!("--icon-name={icon}"));
        }

        if let Some(ref width) = self.width {
            args.push(format!("--width={width}"));
        }

        if let Some(ref height) = self.height {
            args.push(format!("--height={height}"));
        }

        if let Some(ref timeout) = self.timeout {
            args.push(format!("--timeout={}", timeout.as_secs()));
        }

        if let Some(ref modal_hint) = self.modal_hint {
            args.push(format!("--modal={modal_hint}"));
        };

        args.append(&mut self.additional_args);

        args
    }

    /// Render the dialog and wait for user response.
    pub fn show(mut self) -> crate::Result<ZenityOutput<T::Return>> {
        let args = self.get_argv();

        let output =
            Command::new("zenity")
                .args(args)
                .output()
                .map_err(|err| match err.kind() {
                    io::ErrorKind::NotFound => crate::error::Error::ZenityNotInstalled(err),
                    _ => crate::Error::UnexpectedIoError(err),
                })?;

        let stdout = String::from_utf8(output.stdout)
            .map_err(crate::Error::InvalidUtf8FromStdout)?
            .trim()
            .to_owned();

        let code = output.status.code().ok_or(crate::Error::MissingExitCode)?;

        let result = match (stdout.is_empty(), code) {
            (true, Self::SUCCESS_CODE) => ZenityOutput::Affirmed { content: None },
            (false, Self::SUCCESS_CODE) => ZenityOutput::Affirmed {
                content: Some(self.application.parse(&stdout)?),
            },
            (true, Self::ERROR_CODE_1 | Self::ERROR_CODE_2) => {
                ZenityOutput::Rejected { content: None }
            }
            (false, Self::ERROR_CODE_1 | Self::ERROR_CODE_2) => ZenityOutput::Rejected {
                content: Some(stdout),
            },
            _ => ZenityOutput::Unknown {
                exit_code: code,
                stdout,
                stderr: String::from_utf8(output.stderr).unwrap_or_default(),
            },
        };

        Ok(result)
    }
}

/// Represents an instance of Zenity Dialog with an extra button configured.
#[derive(Debug, Clone, Default)]
pub struct ZenityDialogExtButton<T>
where
    T: ZenityApplication,
{
    inner: ZenityDialog<T>,
    extra_button_label: String,
}

impl<T> ZenityDialogExtButton<T>
where
    T: ZenityApplication,
{
    /// Provide a custom title for the dialog.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.inner.title = Some(title.into());
        self
    }

    /// Override the default icon.
    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.inner.icon = Some(icon);
        self
    }

    /// Set a specific width for the dialog.
    pub fn with_width(mut self, width: usize) -> Self {
        self.inner.width = Some(width);
        self
    }

    /// Set a specific height for the dialog.
    pub fn with_height(mut self, height: usize) -> Self {
        self.inner.height = Some(height);
        self
    }

    /// Make the dialog close automatically after the duration has passed.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.inner.timeout = Some(timeout);
        self
    }

    /// Render a hint displaying the provided text.
    pub fn with_modal_hint(mut self, modal_hint: impl Into<String>) -> Self {
        self.inner.modal_hint = Some(modal_hint.into());
        self
    }

    /// Attach an additional custom argument. Used to handle arguments that aren't currently statically
    /// supported. Use at your own risk. Note that this function will automatically prepend -- to the argument
    /// so there is no need to provide it. However, if you do provide it, it will still work.
    pub fn with_additional_arg(mut self, arg: impl Into<Arg>) -> Self {
        let arg: Arg = arg.into();
        self.inner.additional_args.push(arg.to_string());
        self
    }

    /// Like `with_additional_arg`, but takes a [Vec<Arg>]
    pub fn with_additional_args(mut self, args: Vec<Arg>) -> Self {
        self.inner
            .additional_args
            .append(&mut args.iter().map(Arg::to_string).collect::<Vec<String>>());
        self
    }

    /// Display the dialog and wait for user response.
    pub fn show(self) -> crate::Result<ZenityOutputExtButton<T::Return>> {
        let inner = self
            .inner
            .with_additional_arg(("--extra-button", self.extra_button_label.as_str()));
        let extra_button_label = self.extra_button_label;
        let result = inner.show();

        let output = match result {
            Ok(result) => result,
            Err(err) => Err(err)?,
        };

        let content = match output {
            ZenityOutput::Rejected { ref content } => content,
            other => return Ok(other.into()),
        };

        let content = match content {
            Some(content) => content.to_owned(),
            None => return Ok(output.into()),
        };

        match content == extra_button_label {
            true => Ok(ZenityOutputExtButton::ExtButton { content }),
            false => Ok(output.into()),
        }
    }
}

/// Represents the user's response to the dialog.
#[derive(Debug, Clone, PartialEq)]
pub enum ZenityOutput<T>
where
    T: Sized,
{
    /// The user clicked the button that indicated an affirmative response
    Affirmed {
        /// If configured with custom text for the affirmative button, this
        /// value will be [Some] and will contain the custom text. Otherwise,
        /// it is [None] for default values.
        content: Option<T>,
    },
    /// The user clicked a button indicating rejection.
    Rejected {
        /// If configured with custom text for the rejection button, this
        /// value will be [Some] and will contain the custom text. Otherwise,
        /// it is [None] for default values.
        content: Option<String>,
    },
    /// In the case that Zenity returned an unexpected response, this contains
    /// the full details of the response so that the user may respond to it
    /// as needed. If you get this output, it indicates a bug in this library so please report it.
    Unknown {
        /// The returned exit code.
        exit_code: i32,
        /// The content Zenity returned to stdout.
        stdout: String,
        /// The content Zenity returned to stderr.
        stderr: String,
    },
}

/// Represents the user's response to the dialog.
#[derive(Debug, Clone, PartialEq)]
pub enum ZenityOutputExtButton<T>
where
    T: Sized,
{
    /// The user clicked an affirmative
    Affirmed {
        /// If configured with custom text for the affirmative button, this
        /// value will be [Some] and will contain the custom text. Otherwise,
        /// it is [None] for default values.
        content: Option<T>,
    },
    /// The user clicked a button indicating rejection.
    Rejected {
        /// If configured with custom text for the rejection button, this
        /// value will be [Some] and will contain the custom text. Otherwise,
        /// it is [None] for default values.
        content: Option<String>,
    },
    /// If configured with an extra button, this indicates that the user clicked that button.
    ExtButton {
        /// The content of the extra button.
        content: String,
    },
    /// In the case that Zenity returned an unexpected response, this contains
    /// the full details of the response so that the user may respond to it
    /// as needed. If you get this output, it indicates a bug in this library so please report it.
    Unknown {
        /// The returned exit code.
        exit_code: i32,
        /// The content Zenity returned to stdout.
        stdout: String,
        /// The content Zenity returned to stderr.
        stderr: String,
    },
}

impl<T> From<ZenityOutput<T>> for ZenityOutputExtButton<T> {
    fn from(value: ZenityOutput<T>) -> Self {
        match value {
            ZenityOutput::Affirmed { content } => Self::Affirmed { content },
            ZenityOutput::Rejected { content } => Self::Rejected { content },
            ZenityOutput::Unknown {
                exit_code,
                stdout,
                stderr,
            } => Self::Unknown {
                exit_code,
                stdout,
                stderr,
            },
        }
    }
}

/// Represents an icon. [Icon::Error], [Icon::Info], [Icon::Question], and [Icon::Warning] represent
/// standard icons, while [Icon::IconPath] allows you to pass the path of a custom icon.
#[derive(Debug, Clone, PartialEq)]
pub enum Icon {
    /// An error icon
    Error,
    /// An info icon
    Info,
    /// A question icon
    Question,
    /// A warning icon
    Warning,
    /// A path to a custom icon
    IconPath(PathBuf),
}

impl Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = match self {
            Icon::Error => "error",
            Icon::Info => "info",
            Icon::Question => "question",
            Icon::Warning => "warning",
            Icon::IconPath(path) => path.to_str().ok_or(std::fmt::Error)?,
        };

        write!(f, "{base}")
    }
}
