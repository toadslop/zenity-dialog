use std::{io, string::FromUtf8Error};

/// The errors that may occur when trying to launch a Zenity dialog.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Zenity is not installed")]
    ZenityNotInstalled(#[source] io::Error),
    #[error("Unexpected io error occured: {0}")]
    UnexpectedIoError(#[source] io::Error),
    #[error("Failed to decode stdout as utf-8: {0}")]
    InvalidUtf8FromStdout(#[source] FromUtf8Error),
    #[error("Zenity failed to return an exit code.")]
    MissingExitCode,
    #[error("Failed to parse the output: {0}")]
    ParseResultFailure(#[from] anyhow::Error),
}
