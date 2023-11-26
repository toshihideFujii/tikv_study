/*
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};
use fail::fail_point;
pub use kvproto::disk_usage::DiskUsage;

static DISK_RESERVED_SPACE: AtomicU64 = AtomicU64::new(0);
static RAFT_DISK_RESERVED_SPACE: AtomicU64 = AtomicU64::new(0);
static DISK_STATUS: AtomicI32 = AtomicI32::new(0);

pub fn set_disk_reserved_space(v: u64) {
  DISK_RESERVED_SPACE.store(v, Ordering::Release)
}

pub fn get_disk_reserved_space() -> u64 {
  DISK_RESERVED_SPACE.load(Ordering::Acquire)
}

pub fn set_raft_disk_reserved_space(v: u64) {
  RAFT_DISK_RESERVED_SPACE.store(v, Ordering::Release)
}

pub fn get_raft_disk_reserved_space() -> u64 {
  RAFT_DISK_RESERVED_SPACE.load(Ordering::Acquire)
}

pub fn set_disk_status(status: DiskUsage) {
  let v = match status {
    DiskUsage::Normal => 0,
    DiskUsage::AlmostFull => 1,
    DiskUsage::AlreadyFull => 2
  };
  DISK_STATUS.store(v, Ordering::Release)
}

pub fn get_disk_status(store_id: u64) -> DiskUsage {
  fail_point!("disk_almost_full_peer_1", store_id == 1, |_| {
    DiskUsage::AlmostFull
  });
  fail_point!("disk_almost_full_peer_2", store_id == 2, |_| {
    DiskUsage::AlmostFull
  });
  fail_point!("disk_almost_full_peer_3", store_id == 3, |_| {
    DiskUsage::AlmostFull
  });
  fail_point!("disk_almost_full_peer_4", store_id == 4, |_| {
    DiskUsage::AlmostFull
  });
  fail_point!("disk_almost_full_peer_5", store_id == 5, |_| {
    DiskUsage::AlmostFull
  });
  fail_point!("disk_already_full_peer_1", store_id == 1, |_| {
    DiskUsage::AlreadyFull
  });
  fail_point!("disk_already_full_peer_2", store_id == 2, |_| {
    DiskUsage::AlreadyFull
  });
  fail_point!("disk_already_full_peer_3", store_id == 3, |_| {
    DiskUsage::AlreadyFull
  });
  fail_point!("disk_already_full_peer_4", store_id == 4, |_| {
    DiskUsage::AlreadyFull
  });
  fail_point!("disk_already_full_peer_5", store_id == 5, |_| {
    DiskUsage::AlreadyFull
  });

  let s = DISK_STATUS.load(Ordering::Acquire);
  match s {
    0 => DiskUsage::Normal,
    1 => DiskUsage::AlmostFull,
    2 => DiskUsage::AlreadyFull,
    _ => panic!("Disk Status value not meet expectations")
  }
}
*/