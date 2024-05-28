//! A thin wrapper arround Zenity, a tool for rendering dialog boxes in Linux.
//! This mvp version supports only a limitted number of Zenity options.

mod arg;
mod dialog;
mod error;

pub type Result = std::result::Result<ZenityOutput, crate::error::Error>;
pub use crate::arg::Arg;
pub use crate::dialog::Application;
pub use crate::dialog::Icon;
pub use crate::dialog::InfoOptions;
pub use crate::dialog::ZenityDialog;
pub use crate::dialog::ZenityOutput;
pub use crate::error::Error;
