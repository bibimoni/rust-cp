use std::ops::{Bound, RangeBounds};
pub struct Fenwick<T> {
  n: usize,
  arr: Vec<T>,
  default: T,
}

impl<T: Clone + std::ops::AddAssign<T>> Fenwick<T> {
  pub fn new(n: usize, default: T) -> Self {
    Self {
      n,
      arr: vec![default.clone(); n],
      default,
    }
  }
  // return a[1] + a[2] + ... + a[i]
  pub fn get(&self, mut i: usize) -> T {
    let mut ret: T = self.default.clone();
    while i <= self.n {
      ret += self.arr[i - 1].clone();
      i &= i - 1;
    }
    ret
  }
  // a[i] += x
  pub fn add<U: Clone>(&mut self, mut i: usize, x: U)
  where
    T: std::ops::AddAssign<U>,
  {
    while i > 0 {
      self.arr[i - 1] += x.clone();
      i += i & i.wrapping_neg();
    }
  }
  // return a[l] + a[l + 1] + ... + a[r]
  pub fn sum<R>(&self, range: R) -> T
  where
    T: std::ops::Sub<Output = T>,
    R: RangeBounds<usize>,
  {
    let r = match range.end_bound() {
      Bound::Included(r) => *r,
      Bound::Excluded(r) => r - 1,
      Bound::Unbounded => self.n,
    };
    let l = match range.start_bound() {
      Bound::Included(l) => l - 1,
      Bound::Excluded(l) => *l,
      Bound::Unbounded => self.n,
    };
    self.get(r) - self.get(l)
  }
}
