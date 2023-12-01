pub use std::time::Duration;
use std::{
  cmp::Ordering,
  ops::{Add, AddAssign, Sub, SubAssign},
  //sync::mpsc::{self, Sender},
  //thread::{self, Builder, JoinHandle},
  time::{SystemTime, UNIX_EPOCH}
};
use time::{Duration as TimeDuration, Timespec};

use self::inner::monotonic_coarse_now;
pub use self::inner::monotonic_now;
pub use self::inner::monotonic_raw_now;

const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;
const MILLISECOND_PER_SECOND: i64 = 1_000;
const NANOSECONDS_PER_MILLISECOND: i64 = 1_000_000;
const DEFAULT_SLOW_SECS: u64 = 1;
//const DEFAULT_WAIT_MS: u64 = 100;

#[inline]
pub fn duration_to_ms(d: Duration) -> u64 {
  let nanos = u64::from(d.subsec_nanos());
  d.as_secs() * 1_000 + (nanos / 1_000_000)
}

#[inline]
pub fn duration_to_sec(d: Duration) -> f64 {
  let nanos = f64::from(d.subsec_nanos());
  d.as_secs() as f64 + (nanos / 1_000_000_000.0)
}

#[inline]
pub fn duration_to_us(d: Duration) -> u64 {
  let nanos = u64::from(d.subsec_nanos());
  d.as_secs() * 1_000_000 + (nanos / 1_000)
}

#[inline]
pub fn duration_to_ns(d: Duration) -> u64 {
  let nanos = u64::from(d.subsec_nanos());
  d.as_secs() * 1_000_000_000 + nanos
}

#[inline]
pub fn timespec_to_ns(t: Timespec) -> u64 {
  (t.sec as u64) * NANOSECONDS_PER_SECOND + t.nsec as u64
}

pub trait InstantExt {
  fn saturating_elapsed(&self) -> Duration;
}

impl InstantExt for std::time::Instant {
  #[inline]
  fn saturating_elapsed(&self) -> Duration {
    std::time::Instant::now().saturating_duration_since(*self)
  }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnixSecs(u64);

impl UnixSecs {
  pub fn now() -> Self {
    UnixSecs(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
  }

  pub fn zero() -> Self { UnixSecs(0) }

  pub fn into_inner(self) -> u64 { self.0 }

  pub fn is_zero(self) -> bool { self.0 == 0 }
}

pub struct SlowTimer {
    slow_time: Duration,
    t: Instant
}

impl SlowTimer {
  pub fn new() -> Self { SlowTimer::default() }

  pub fn from(slow_time: Duration) -> Self {
    SlowTimer { slow_time: slow_time, t: Instant::now_coarse()  }
  }

  pub fn from_secs(secs: u64) -> Self {
    SlowTimer::from(Duration::from_secs(secs))
  }

  pub fn from_millis(millis: u64) -> Self {
    SlowTimer::from(Duration::from_millis(millis))
  }

  pub fn saturating_elapsed(&self) -> Duration {
    self.t.saturating_elapsed()
  }

  pub fn is_slow(&self) -> bool {
    self.saturating_elapsed() >= self.slow_time
  }
}

impl Default for SlowTimer {
  fn default() -> Self {
    SlowTimer::from_secs(DEFAULT_SLOW_SECS)
  }
}

/*
pub struct Monitor {
  tx: Sender<bool>,
  handle: Option<JoinHandle<()>>
}

impl Monitor {
  pub fn new<D, N>(on_jumped: D, now: N) -> Self
    where
      D: Fn() + Send + 'static,
      N: Fn() -> SystemTime + Send + 'static
  {
    let props = crate::thread_group::current_properties();
    let (tx, rx) = mpsc::channel();
    //let h = Builder::new().name(thd_name!("time-monitor")).s
  }
}
*/

#[cfg(not(target_os = "linux"))]
mod inner {
  use time::{self, Timespec};
  use super::NANOSECONDS_PER_SECOND;

  pub fn monotonic_raw_now() -> Timespec {
    let ns = time::precise_time_ns();
    let s = ns / NANOSECONDS_PER_SECOND;
    let ns = ns % NANOSECONDS_PER_SECOND;
    Timespec::new(s as i64, ns as i32)
  }

  pub fn monotonic_now() -> Timespec {
    monotonic_raw_now()
  }

  pub fn monotonic_coarse_now() -> Timespec {
    monotonic_raw_now()
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Instant {
  Monotonic(Timespec),
  MonotonicCoarse(Timespec),
}

impl Instant {
  pub fn now() -> Self {
    Instant::Monotonic(monotonic_now())
  }

  pub fn now_coarse() -> Self {
    Instant::MonotonicCoarse(monotonic_coarse_now())
  }

  pub fn saturating_elapsed(&self) -> Duration {
    match *self {
      Instant::Monotonic(t) => {
        let now = monotonic_now();
        Instant::saturating_elapsed_duration(now, t)
      }
      Instant::MonotonicCoarse(t) => {
        let now = monotonic_coarse_now();
        Instant::saturating_elapsed_duration_coarse(now, t)
      }
    }
  }

  pub fn saturating_elapsed_secs(&self) -> f64 {
    duration_to_sec(self.saturating_elapsed())
  }

  pub fn duration_since(&self, earlier: Instant) -> Duration {
    match (*self, earlier) {
      (Instant::Monotonic(later), Instant::Monotonic(earlier)) => {
        Instant::elapsed_duration(later, earlier)
      }
      (Instant::MonotonicCoarse(later), Instant::MonotonicCoarse(earlier)) => {
        Instant::saturating_elapsed_duration_coarse(later, earlier)
      }
      _ => panic!("duration between different types of Instants")
    }
  }

  pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
    match (*self, earlier) {
      (Instant::Monotonic(later), Instant::Monotonic(earlier)) => {
        Instant::saturating_elapsed_duration(later, earlier)
      }
      (Instant::MonotonicCoarse(later), Instant::MonotonicCoarse(earlier)) => {
        Instant::saturating_elapsed_duration_coarse(later, earlier)
      }
      _ => panic!("duration between different types of Instants")
    }
  }

  pub fn checked_sub(&self, other: Instant) -> Option<Duration> {
    if *self >= other {
      Some(self.duration_since(other))
    } else {
      None
    }
  }

  pub(crate) fn elapsed_duration(later: Timespec, earlier: Timespec) -> Duration {
    if later >= earlier {
      (later - earlier).to_std().unwrap()
    } else {
      panic!("monotonic time jumped back, {:.9} -> {:.9}",
        earlier.sec as f64 + f64::from(earlier.nsec) / NANOSECONDS_PER_SECOND as f64,
        later.sec as f64 + f64::from(later.nsec) / NANOSECONDS_PER_SECOND as f64  
      );
    }
  }

  pub(crate) fn saturating_elapsed_duration(later: Timespec, earlier: Timespec) -> Duration {
    if later >= earlier {
      (later - earlier).to_std().unwrap()
    } else {
      // TODO: error!
      Duration::from_millis(0)
    }
  }

  pub(crate) fn saturating_elapsed_duration_coarse(later: Timespec,
    earlier: Timespec) -> Duration
  {
    let later_ms = later.sec * MILLISECOND_PER_SECOND
      + i64::from(later.nsec) / NANOSECONDS_PER_MILLISECOND;
    let earlier_ms = earlier.sec * MILLISECOND_PER_SECOND
      + i64::from(earlier.nsec) / NANOSECONDS_PER_MILLISECOND;
    let dur = later_ms - earlier_ms;
    if dur >= 0 {
      Duration::from_millis(dur as u64)
    } else {
      // TODO: debug!
      Duration::from_millis(0)
    }
  }
}

impl PartialEq for Instant {
  fn eq(&self, other: &Instant) -> bool {
    match (*self, *other) {
      (Instant::Monotonic(this), Instant::Monotonic(other))
      | (Instant::MonotonicCoarse(this), Instant::MonotonicCoarse(other)) => {
        this.eq(&other)
      }
      _ => false
    }
  }
}

impl PartialOrd for Instant {
  fn partial_cmp(&self, other: &Instant) -> Option<Ordering> {
    match (*self, *other) {
      (Instant::Monotonic(this), Instant::Monotonic(other))
      | (Instant::MonotonicCoarse(this), Instant::MonotonicCoarse(other)) => {
        this.partial_cmp(&other)
      }
      _ => None   
    }
  }
}

impl Add<Duration> for Instant {
  type Output = Instant;
  fn add(self, other: Duration) -> Instant {
    match self {
      Instant::Monotonic(t) =>
        Instant::Monotonic(t + TimeDuration::from_std(other).unwrap()),
      Instant::MonotonicCoarse(t) =>
        Instant::MonotonicCoarse(t + TimeDuration::from_std(other).unwrap())
    }
  }
}

impl AddAssign<Duration> for Instant {
  fn add_assign(&mut self, rhs: Duration) {
    *self = self.add(rhs)
  }
}

impl Sub<Duration> for Instant {
  type Output = Instant;
  fn sub(self, other: Duration) -> Instant {
    match self {
      Instant::Monotonic(t) =>
        Instant::Monotonic(t - TimeDuration::from_std(other).unwrap()),
      Instant::MonotonicCoarse(t) =>
        Instant::MonotonicCoarse(t - TimeDuration::from_std(other).unwrap())
    }
  }
}

impl SubAssign<Duration> for Instant {
  fn sub_assign(&mut self, rhs: Duration) {
    *self = self.sub(rhs)
  }
}