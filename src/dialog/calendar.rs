use super::{application::ToArgVector, ZenityApplication};
#[cfg(feature = "chrono")]
use chrono::NaiveDate;
use std::fmt::{Debug, Display};

#[derive(Default, Clone)]
pub struct Calendar {
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

impl Debug for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Calendar")
            .field("text", &self.text)
            .field("day", &self.day)
            .field("month", &self.month)
            .field("year", &self.year)
            .field("format", &self.format)
            .field("parse_fn", &"(&str) -> String")
            .finish()
    }
}

impl ZenityApplication for Calendar {
    #[cfg(feature = "chrono")]
    type Return = NaiveDate;
    #[cfg(not(feature = "chrono"))]
    type Return = String;

    fn parse(&self, stdout: &str) -> Result<Self::Return, crate::Error> {
        #[cfg(feature = "chrono")]
        return NaiveDate::parse_from_str(
            stdout.trim(),
            self.format.as_deref().unwrap_or(Self::DEFAULT_DATE_FORMAT),
        )
        .map_err(|err| crate::Error::ParseResultFailure(anyhow::Error::new(err)));

        #[cfg(not(feature = "chrono"))]
        Ok(stdout.to_owned())
    }
}

impl ToArgVector for Calendar {
    fn to_argv(&self) -> Vec<String> {
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
            args.push(format!("--date-format={format}"));
        }

        args
    }
}

impl Calendar {
    #[cfg(feature = "chrono")]
    const DEFAULT_DATE_FORMAT: &'static str = "%d/%m/%y";

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

    #[cfg(not(feature = "chrono"))]
    /// Set the format for the returned date.
    /// The default depends on the user locale or be set with the strftime style.
    /// For example %A %d/%m/%y. Note that this feature is disabled when using
    /// the "chrono" features as it can interfere with Chrono parsing the output.
    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
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
