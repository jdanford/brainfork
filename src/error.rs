use std::{io, process::ExitCode};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("maximum code size exceeded")]
    MaxCodeSizeExceeded,

    #[error("encountered loop start without matching end")]
    UnmatchedLoopStart,

    #[error("encountered loop end without matching start")]
    UnmatchedLoopEnd,

    #[error(transparent)]
    Io(#[from] io::Error),
}

pub fn handle_error<T>(result: Result<T>) -> ExitCode {
    if let Err(err) = result {
        eprintln!("{err}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
