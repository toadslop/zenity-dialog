#![deny(missing_docs)]

//! A thin wrapper arround Zenity, a tool for rendering dialog boxes in Linux.
//! This mvp version supports only a limitted number of Zenity options.
//!
//! ## Usage
//!
//! ```
//! let result = ZenityDialog::new(dialog::Error::default().with_text("An error happened!"))
//! .with_icon(Icon::Error)
//! .show()?;
//!
//! match result {
//! ZenityOutput::Affirmed { .. } => {
//!     println!("The user clicked the affirmative response")
//! }
//! ZenityOutput::Rejected { .. } => println!("The user clicked the rejection response"),
//! ZenityOutput::Unknown {
//!     exit_code,
//!     stdout,
//!     stderr,
//! } => println!("Something weird happened. {exit_code} {stdout} {stderr}"),
//! };
//! ```
//!
//! ## Features
//!
//! ### Chrono
//!
//! Enable automatic date parsing for [Calendar] using Chrono. When this feature is
//! enabled, you won't be able to pass custom date formats to Zenity as this can interfere
//! with Chrono's ability to properly parse the date.

#[cfg(feature = "chrono")]
extern crate chrono;

mod arg;
/// Contains configuration structs for the various types of Zenity dialogs.
pub mod dialog;
mod error;

/// Alias for the common [Result] produced by operations in this crate.
pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub use crate::arg::Arg;
pub use crate::dialog::ZenityDialog;
pub use crate::dialog::ZenityDialogExtButton;
pub use crate::dialog::ZenityOutput;
pub use crate::dialog::ZenityOutputExtButton;
pub use crate::error::Error;
