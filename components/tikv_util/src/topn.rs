use std::{
  cmp::Reverse,
  collections::{binary_heap, BinaryHeap},
  iter
};

pub struct TopN<T> {
  capacity: usize,
  heap: BinaryHeap<Reverse<T>>
}

impl<T: Ord> TopN<T> {
  pub fn new(capacity: usize) -> TopN<T> {
    TopN { capacity: capacity, heap: BinaryHeap::with_capacity(capacity) }
  }

  pub fn push(&mut self, item: T) {
    self.heap.push(Reverse(item));
    if self.heap.len() > self.capacity {
      self.heap.pop();
    }
  }

  pub fn pop(&mut self) -> Option<T> {
    self.heap.pop().map(|Reverse(x)| x)
  }

  pub fn len(&self) -> usize {
    self.heap.len()
  }

  pub fn is_empty(&self) -> bool {
    self.heap.is_empty()
  }

  pub fn peek(&self) -> Option<&T> {
    self.heap.peek().map(|Reverse(x)| x)
  }
}

impl<T> IntoIterator for TopN<T> {
  type Item = T;

  #[allow(clippy::type_complexity)]
  type IntoIter = iter::Map<binary_heap::IntoIter<Reverse<T>>, fn(Reverse<T>) -> T>;

  fn into_iter(self) -> Self::IntoIter {
    self.heap.into_iter().map(|Reverse(x)| x)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_0_capacity() {
    let mut cap_0_topn = TopN::new(0);
    cap_0_topn.push(1);
    assert_eq!(cap_0_topn.pop(), None);
    assert_eq!(cap_0_topn.len(), 0);
    assert_eq!(cap_0_topn.is_empty(), true);
  }
}

#[test]
fn test_1_capacity() {
  let mut cap_1_topn = TopN::new(1);
  cap_1_topn.push(1);
  assert_eq!(cap_1_topn.peek(), Some(&1));
  assert_eq!(cap_1_topn.len(), 1);
  assert_eq!(cap_1_topn.is_empty(), false);
}

#[test]
fn test_trivial() {
  let mut fix_topn = TopN::new(5);
  fix_topn.push(1);
  fix_topn.push(2);
  fix_topn.push(3);
  fix_topn.push(6);
  fix_topn.push(5);
  fix_topn.push(4);
  assert_eq!(fix_topn.len(), 5);
  assert_eq!(fix_topn.pop(), Some(2));
  assert_eq!(fix_topn.pop(), Some(3));
  assert_eq!(fix_topn.pop(), Some(4));
  assert_eq!(fix_topn.pop(), Some(5));
  assert_eq!(fix_topn.pop(), Some(6));
  assert_eq!(fix_topn.pop(), None);

  let mut fix_topn = TopN::new(5);
  fix_topn.push(1);
  fix_topn.push(2);
  fix_topn.push(3);
  fix_topn.push(6);
  fix_topn.push(5);
  fix_topn.push(4);
  let mut v: Vec<_> = fix_topn.into_iter().collect();
  v.sort_unstable();
  assert_eq!(v.len(), 5);
  assert_eq!(v, vec![2, 3, 4, 5, 6]);
}