
use std::io::{self, ErrorKind};
use error_code::{self, ErrorCode, ErrorCodeExt};
use thiserror::Error;

pub mod number;
pub mod stream_event;

pub type BytesSlice<'a> = &'a [u8];

#[derive(Debug, Error)]
pub enum Error {
  #[error("{0}")]
  Io(#[from] io::Error),
  #[error("bad format key(length)")]
  KeyLength,
  #[error("bad format key(padding)")]
  KeyPadding,
  #[error("key not found")]
  KeyNotFound,
  #[error("bad format value(length)")]
  ValueLength,
  #[error("bad format value(meta)")]
  ValueMeta
}

impl Error {
  pub fn maybe_clone(&self) -> Option<Error> {
    match *self {
      Error::KeyLength => Some(Error::KeyLength),
      Error::KeyPadding => Some(Error::KeyPadding),
      Error::KeyNotFound => Some(Error::KeyNotFound),
      Error::ValueLength => Some(Error::ValueLength),
      Error::ValueMeta => Some(Error::ValueMeta),
      Error::Io(_) => None
    }
  }

  pub fn unexpected_eof() -> Error {
    Error::Io(io::Error::new(ErrorKind::UnexpectedEof, "eof"))
  }
}

pub type Result<T> = std::result::Result<T, Error>;


impl ErrorCodeExt for Error {
  fn error_code(&self) -> ErrorCode {
    match self {
      Error::Io(_) => error_code::codec::IO,
      Error::KeyLength => error_code::codec::KEY_LENGTH,
      Error::KeyPadding => error_code::codec::BAD_PADDING,
      Error::KeyNotFound => error_code::codec::KEY_NOT_FOUND,
      Error::ValueLength => error_code::codec::VALUE_LENGTH,
      Error::ValueMeta => error_code::codec::VALUE_META
    }
  }
}