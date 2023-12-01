use fail::fail_point;
use super::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct DeadlineError;

impl std::error::Error for DeadlineError {
  fn description(&self) -> &str {
    "deadline has elapsed"
  }
}

impl std::fmt::Display for DeadlineError {
  fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(fmt, "deadline has elapsed")
  }
}

#[derive(Debug, Clone, Copy)]
pub struct Deadline {
  deadline: Instant
}

impl Deadline {
  pub fn new(deadline: Instant) -> Self {
    Deadline { deadline: deadline }
  }

  pub fn from_now(after_duration: Duration) -> Self {
    let deadline = Instant::now_coarse() + after_duration;
    Deadline { deadline: deadline }
  }

  pub fn inner(&self) -> Instant {
    self.deadline
  }

  pub fn check(&self) -> std::result::Result<(), DeadlineError> {
    fail_point!("deadline_check_fail", |_| Err(DeadlineError));

    let now = Instant::now_coarse();
    if self.deadline <= now {
      return Err(DeadlineError);
    }
    Ok(())
  }

  pub fn to_std_instant(&self) -> std::time::Instant {
    std::time::Instant::now() + self.deadline.duration_since(Instant::now_coarse())
  }
}