use std::process::ExitCode;
use std::{io, process::Termination, result};
use thiserror::Error;

pub type Result<T> = result::Result<T, SimulationError>;

#[allow(unused)]
#[derive(Error, Debug)]
pub enum SimulationError {
    #[error("IO Error")]
    Io(#[from] io::Error),
    #[error("unknown data store error")]
    Unknown,
}

impl Termination for SimulationError {
    fn report(self) -> ExitCode {
        match self {
            SimulationError::Io(_) => ExitCode::FAILURE,
            SimulationError::Unknown => ExitCode::FAILURE,
        }
    }
}
