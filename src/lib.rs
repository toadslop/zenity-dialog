#![doc = include_str!("./../README.md")]
#![deny(missing_docs)]

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
