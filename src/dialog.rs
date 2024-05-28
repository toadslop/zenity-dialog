use crate::Arg;
use std::{fmt::Display, io, path::PathBuf, process::Command, time::Duration};

/// The configuration for a Zenity dialog.
#[derive(Debug, Clone, PartialEq)]
pub struct ZenityDialog {
    /// The type of dialog to display
    pub application: Application,
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
    /// If present, renders an extra button with a custom label.
    pub extra_button_label: Option<String>,
    /// Provide extra hint text to the user.
    pub modal_hint: Option<String>,
    additional_args: Vec<String>,
}

impl Default for ZenityDialog {
    fn default() -> Self {
        Self {
            application: Application::Info(Default::default()),
            title: Default::default(),
            icon: Default::default(),
            width: Default::default(),
            height: Default::default(),
            timeout: Default::default(),
            extra_button_label: Default::default(),
            modal_hint: Default::default(),
            additional_args: Default::default(),
        }
    }
}

impl ZenityDialog {
    /// Zenity returns zero when users select an afirmitive response.
    const SUCCESS_CODE: i32 = 0;
    /// Zenity returns 256 when users select a negative response.
    const ERROR_CODE_1: i32 = 256;
    const ERROR_CODE_2: i32 = 1;

    /// Construct a new Zenity instance. It expects an [Application], which determines which
    /// kind of dialog will be displayed.
    pub fn new(application: Application) -> Self {
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
    pub fn with_extra_button(mut self, extra_button_label: impl Into<String>) -> Self {
        self.extra_button_label = Some(extra_button_label.into());
        self
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
    fn get_argv(mut self) -> Vec<String> {
        let mut args = self.application.get_argv();

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

        if let Some(ref extra_button_label) = self.extra_button_label {
            args.push(format!("--extra-button={extra_button_label}"));
        }

        if let Some(ref modal_hint) = self.modal_hint {
            args.push(format!("--modal={modal_hint}"));
        };

        args.append(&mut self.additional_args);

        args
    }

    /// Render the dialog and wait for user response.
    pub fn show(self) -> crate::Result {
        let args = self.get_argv();
        dbg!(&args);
        let output =
            Command::new("zenity")
                .args(args)
                .output()
                .map_err(|err| match err.kind() {
                    io::ErrorKind::NotFound => crate::error::Error::ZenityNotInstalled(err),
                    _ => crate::Error::UnexpectedIoError(err),
                })?;
        dbg!(&output);
        let stdout =
            String::from_utf8(output.stdout).map_err(crate::Error::InvalidUtf8FromStdout)?;
        let code = output.status.code().ok_or(crate::Error::MissingExitCode)?;

        let result = match (stdout.is_empty(), code) {
            (true, Self::SUCCESS_CODE) => ZenityOutput::Affirmed,
            (false, Self::SUCCESS_CODE) => ZenityOutput::AffirmedWithContent(stdout),
            (true, Self::ERROR_CODE_1 | Self::ERROR_CODE_2) => ZenityOutput::Rejected,
            (false, Self::ERROR_CODE_1 | Self::ERROR_CODE_2) => {
                ZenityOutput::RejectedWithContent(stdout)
            }
            _ => ZenityOutput::Unknown {
                exit_code: code,
                stdout,
                stderr: String::from_utf8(output.stderr).unwrap_or_default(),
            },
        };

        Ok(result)
    }
}

/// Represents the user's response to the dialog.
#[derive(Debug, Clone, PartialEq)]
pub enum ZenityOutput {
    /// The user clicked an affirmative, or possibly neutral button
    Affirmed,
    /// Affirmed with additional content. This is usually returned when using custom button labels.
    AffirmedWithContent(String),
    /// The user clicked a button indicating rejection.
    Rejected,
    /// The user clicked a button indicating rejection and additional content was returned. This usually
    /// means a custom label was provided for the reject button.
    RejectedWithContent(String),
    /// Zenity returned an unexpected value.
    Unknown {
        exit_code: i32,
        stdout: String,
        stderr: String,
    },
}

/// The different types of dialogs that Zenity can render.
#[derive(Debug, Clone, PartialEq)]
pub enum Application {
    /// Merely display information with an affirmative button and an optional secondary button.
    Info(InfoOptions),
    /// Display a date select dialog.
    Calendar(CalendarOptions),
    /// Collect information from the user.
    Entry,
    /// Display an error dialog
    Error,
    /// Select a file
    FileSelection,
    /// Collect a list in inputs from the user
    List,
    /// Provide a simple notification with no interaction.
    Notification,
    /// Display progress to the end user
    Progress,
    /// Pose a question to the user and collect a response.
    Question,
    /// Display a warning message
    Warning,
    /// Display a slider to the user and get input.
    Scale,
    /// Display a large amount of text input, possibly from a file.
    TextInfo,
    /// Display a color picker to the user.
    ColorSelection,
    /// Display a password input.
    Password,
    /// Display a complex form with multiple types of input.
    Forms,
}

impl Application {
    fn get_argv(self) -> Vec<String> {
        match self {
            Application::Calendar(args) => args.get_argv(),
            Application::Entry => vec!["--entry".to_string()],
            Application::Error => vec!["--error".to_string()],
            Application::Info(args) => args.get_argv(),
            Application::FileSelection => vec!["--file-selection".to_string()],
            Application::List => vec!["--list".to_string()],
            Application::Notification => vec!["--notification".to_string()],
            Application::Progress => vec!["--progress".to_string()],
            Application::Question => vec!["--question".to_string()],
            Application::Warning => vec!["--warning".to_string()],
            Application::Scale => vec!["--scale".to_string()],
            Application::TextInfo => vec!["--text-info".to_string()],
            Application::ColorSelection => vec!["--color-selection".to_string()],
            Application::Password => vec!["--password".to_string()],
            Application::Forms => vec!["--forms".to_string()],
        }
    }
}

/// Represents an icon. [Icon::Error], [Icon::Info], [Icon::Question], and [Icon::Warning] represent
/// standard icons, while [Icon::IconPath] allows you to pass the path of a custom icon.
#[derive(Debug, Clone, PartialEq)]
pub enum Icon {
    Error,
    Info,
    Question,
    Warning,
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

/// Options specific to an [Application::Info].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InfoOptions {
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

impl InfoOptions {
    /// The default options
    pub fn new() -> Self {
        Default::default()
    }

    /// Set body text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set a custom ok label
    pub fn with_ok_label(mut self, ok_label: impl Into<String>) -> Self {
        self.ok_label = Some(ok_label.into());
        self
    }

    /// Set no-wrap to true
    pub fn set_no_wrap(mut self) -> Self {
        self.no_wrap = true;
        self
    }

    /// Disable markup support
    pub fn set_no_markup(mut self) -> Self {
        self.no_markup = true;
        self
    }

    /// Display ellipses for very long texts.
    pub fn set_ellipsize(mut self) -> Self {
        self.ellipsize = true;
        self
    }

    /// Convert to a vec of formatted argument strings
    fn get_argv(self) -> Vec<String> {
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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalendarOptions {
    /// The body text
    pub text: Option<String>,

    /// The numeric day of the month to display as the default input. If it is larger than what is possible for the
    /// selected month, it is ignored.
    pub day: Option<usize>,

    /// The month to display as default input
    pub month: Option<Month>,

    /// The year to display as default input
    pub year: Option<isize>,

    /// The output format for the date the user selects
    pub format: Option<String>,
}

impl CalendarOptions {
    /// Default implementation
    pub fn new() -> Self {
        Default::default()
    }

    /// Set body text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Set the day
    pub fn with_day(mut self, day: impl Into<usize>) -> Self {
        self.day = Some(day.into());
        self
    }

    /// Set the month
    pub fn with_month(mut self, month: impl Into<Month>) -> Self {
        self.month = Some(month.into());
        self
    }

    /// Set the month
    pub fn with_year(mut self, year: impl Into<isize>) -> Self {
        self.year = Some(year.into());
        self
    }

    /// Set the format for the returned date.
    /// The default depends on the user locale or be set with the strftime style.
    /// For example %A %d/%m/%y
    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    /// Convert to a vec of formatted argument strings
    fn get_argv(self) -> Vec<String> {
        let mut args = vec!["--calendar".to_string()];

        if let Some(ref text) = self.text {
            args.push(format!("--text={text}"))
        };

        if let Some(ref day) = self.day {
            args.push(format!("--day={day}"))
        };

        if let Some(ref month) = self.month {
            args.push(format!("--month={month}"))
        };

        if let Some(ref year) = self.year {
            args.push(format!("--year={year}"))
        };

        if let Some(ref format) = self.format {
            args.push(format!("--date-format={format}"))
        };

        args
    }
}

/// Represents a calendar month for [Application::Calendar]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Month {
    January = 1,
    Feburary = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_int = match self {
            Month::January => 1,
            Month::Feburary => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        };

        write!(f, "{as_int}")
    }
}
