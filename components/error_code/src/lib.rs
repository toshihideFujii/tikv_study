
use std::fmt::{self, Display, Formatter};

#[macro_use]
extern crate lazy_static;

macro_rules! define_error_codes {
  ($prefix:literal,
    $($name:ident => ($suffix:literal, $description:literal, $workaround:literal)),+
  ) => {
    use crate::ErrorCode;
    $(pub const$name: ErrorCode = ErrorCode {
        code: concat!($prefix, $suffix),
        description: $description,
        workaround: $workaround
    };)+
    lazy_static! {
      pub static ref ALL_ERROR_CODES: Vec<ErrorCode> = vec![$($name,)+];
    }      
  };
}

pub const UNKNOWN: ErrorCode = ErrorCode {
  code: "KV:Unknown",
  description: "",
  workaround: ""
};

pub mod codec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErrorCode {
  pub code: &'static str,
  pub description: &'static str,
  pub workaround: &'static str
}

impl Display for ErrorCode {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.code)
  }
}

pub trait ErrorCodeExt {
  fn error_code(&self) -> ErrorCode;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_define_error_code() {
    define_error_codes!(
      "KV:Raftstore:",
      ENTRY_TOO_LARGE => ("EntryTooLarge", "", ""),
      NOT_LEADER => ("NotLeader", "", "")
    );

    assert_eq!(ENTRY_TOO_LARGE,
      ErrorCode { code: "KV:Raftstore:EntryTooLarge", description: "", workaround: ""});
    assert_eq!(NOT_LEADER,
      ErrorCode { code: "KV:Raftstore:NotLeader", description: "", workaround: ""});
  }
}