
pub use crate::default::*;

pub type Allocator = snmalloc_rs::SnMalloc;

pub const fn allocator() -> Allocator {
  snmalloc_rs::SnMalloc
}