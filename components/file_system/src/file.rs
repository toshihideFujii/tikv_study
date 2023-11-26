
use std::{
  fs,
  io::{self, Seek},
  //path::Path,
  //sync::Arc,
};

//use fs2::FileExt;

pub struct File {
  inner: fs::File,
}

impl File {
  /*
  pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
    let inner = fs::File::open(path)?;
    Ok(File {
      inner
    })
  }

  pub fn allocated_size(&self) -> io::Result<u64> {
    self.inner.allocated_size()
  }

  pub fn allocate(&self, len: u64) -> io::Result<()> {
    self.inner.allocate(len)
  }

  pub fn lock_shared(&self) -> io::Result<()> {
    self.inner.lock_shared()
  }

  pub fn lock_exclusive(&self) -> io::Result<()> {
    self.inner.lock_exclusive()
  }

  pub fn try_lock_shared(&self) -> io::Result<()> {
    self.inner.try_lock_shared()
  }

  pub fn try_lock_exclusive(&self) -> io::Result<()> {
    self.inner.try_lock_exclusive()
  }

  pub fn unlock(&self) -> io::Result<()> {
    self.inner.unlock()
  }
  */
}

impl Seek for File {
  fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
    self.inner.seek(pos)
  }
}

