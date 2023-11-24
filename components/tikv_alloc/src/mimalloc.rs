
pub use crate::default::*;

pub type Allocator = mimalloc::MiMalloc;

pub const fn allocator() -> Allocator {
  mimalloc::MiMalloc
}