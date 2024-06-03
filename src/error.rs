use std::{io, string::FromUtf8Error};

/// The errors that may occur when trying to launch a Zenity dialog.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to find Zenity
    #[error("Zenity is not installed")]
    ZenityNotInstalled(#[source] io::Error),
    /// A currently untracked error type occured when trying to invoke Zenity.
    #[error("Unexpected io error occured: {0}")]
    UnexpectedIoError(#[source] io::Error),
    /// If Zenity response with a non-utf8-compliant response, we won't be able to decode and
    /// respond. Typically, this error should never actually occur and if it does, please
    /// report it.
    #[error("Failed to decode stdout as utf-8: {0}")]
    InvalidUtf8FromStdout(#[source] FromUtf8Error),
    /// An error that should never occur. Yet, the [std::process::Command] does not guarantee that
    /// a code is produced, so we may potentially this error in the off chance that one is not returned.
    #[error("Zenity failed to return an exit code.")]
    MissingExitCode,
    /// Occurs if the output from Zenity could not be converted to [crate::ZenityOutput]. This would
    /// typically indicate a bug in this crate, so if it occurs, please open an issue!
    #[error("Failed to parse the output: {0}")]
    ParseResultFailure(#[from] anyhow::Error),
}
