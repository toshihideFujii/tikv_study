

#[derive(Debug, Clone, Copy)]
pub struct IoStat {
  pub read: u64,
  pub write: u64
}

#[derive(Debug, Clone, Copy)]
pub struct DiskStat {
  pub reads: u64,
  pub time_reading: u64,
  pub writes: u64,
  pub time_writing: u64,
  pub sectors_read: u64,
  pub sectors_write: u64
}

pub trait ThreadInspector {
  type DiskID;

  fn io_stat(&self) -> Result<Option<IoStat>, String> {
    Ok(None)
  }

  fn get_device(_path: &str) -> Result<Option<Self::DiskID>, String> {
    Ok(None)
  }

  fn disk_stat(_dev: &Self::DiskID) -> Result<Option<DiskStat>, String> {
    Ok(None)
  }
}

#[cfg(not(target_os = "linux"))]
mod notlinux {
  use super::ThreadInspector;
  pub struct Impl;
  impl ThreadInspector for Impl {
    type DiskID = ();
  }

  //pub fn self_thread_inspector() -> Result<Impl, String> {
    //Ok(Impl)
  //}
}

//#[cfg(not(target_os = "linux"))]
//pub use self::notlinux::{self_thread_inspector, Impl as ThreadInspector};