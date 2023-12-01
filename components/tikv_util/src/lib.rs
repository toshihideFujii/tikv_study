use std::thread;

pub mod callback;
pub mod codec;
pub mod deadline;
pub mod keybuilder;
#[macro_use]
pub mod log;
#[macro_use]
pub mod macros;
pub mod math;
pub mod store;
pub mod sys;
pub mod thread_group;
pub mod time;
pub mod topn;

pub fn get_tag_from_thread_name() -> Option<String> {
  thread::current()
    .name()
    .and_then(|name| name.split("::").skip(1).last())
    .map(From::from)
}

#[derive(Debug, Clone, Copy)]
pub enum Either<L, R> {
  Left(L),
  Right(R)
}

impl<L, R> Either<L, R> {
  #[inline]
  pub fn as_ref(&self) -> Either<&L, &R> {
    match *self {
      Either::Left(ref l) => Either::Left(l),
      Either::Right(ref r) => Either::Right(r)
    }
  }

  #[inline]
  pub fn as_mut(&mut self) -> Either<&mut L, &mut R> {
    match *self {
      Either::Left(ref mut l) => Either::Left(l),
      Either::Right(ref mut r) => Either::Right(r)
    }
  }

  #[inline]
  pub fn left(self) -> Option<L> {
    match self {
      Either::Left(l) => Some(l),
      _ => None
    }
  }

  #[inline]
  pub fn right(self) -> Option<R> {
    match self {
      Either::Right(r) => Some(r),
      _ => None
    }
  }

  #[inline]
  pub fn is_left(&self) -> bool {
    match *self {
      Either::Left(_) => true,
      Either::Right(_) => false
    }
  }

  #[inline]
  pub fn is_right(&self) -> bool {
    !self.is_left()
  }
}

impl<L, R, T> AsRef<T> for Either<L, R>
  where T: ?Sized, L: AsRef<T>, R: AsRef<T>
{
  fn as_ref(&self) -> &T {
    match self {
      Either::Left(l) => l.as_ref(),
      Either::Right(r) => r.as_ref()
    }
  }
}