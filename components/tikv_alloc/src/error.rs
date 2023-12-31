
use std::{error, fmt};

#[derive(Debug)]
pub enum ProfError {
  MemProfilingNotEnabled,
  IoError(std::io::Error),
  JemallocError(String),
  PathEncodingError(std::ffi::OsString),
  PathWithNulError(std::ffi::NulError)
}

pub type ProfResult<T> = std::result::Result<T, ProfError>;

impl fmt::Display for ProfError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ProfError::MemProfilingNotEnabled => write!(f, "mem-profiling was not enabled"),
      ProfError::IoError(e) => write!(f, "io error occurred {:?}", e),
      ProfError::JemallocError(e) => write!(f, "jemalloc error {}", e),
      ProfError::PathEncodingError(path) =>
        write!(f, "Dump target path {:?} is not unicode encoding", path),
      ProfError::PathWithNulError(path) =>
        write!(f, "Dump target path {:?} contain an internal 0 byte", path)
    }
  }
}

impl From<std::io::Error> for ProfError {
  fn from(e: std::io::Error) -> Self {
    ProfError::IoError(e)
  }
}

impl From<std::ffi::NulError> for ProfError {
  fn from(e: std::ffi::NulError) -> Self {
    ProfError::PathWithNulError(e)
  }
}

impl error::Error for ProfError {}