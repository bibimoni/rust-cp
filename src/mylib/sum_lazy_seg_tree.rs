#[derive(Clone)]
pub struct SumLazySegTree<T, F, U>
where
  T: Clone + Copy,
  U: Clone + Copy,
  F: Fn(T, T) -> T,
  U: Fn(T, T, i32, i32) -> T,
{
  n: i32,
  default: T,
  trash_val: T,
  st: Vec<T>,
  lazy: Vec<T>,
  op: F,
  up: U,
}

impl<T, F, U> SumLazySegTree<T, F, U>
where
  T: Clone + Copy + std::cmp::PartialEq + std::fmt::Debug,
  U: Clone + Copy,
  F: Fn(T, T) -> T,
  U: Fn(T, T, i32, i32) -> T,
{
  /// Constructs a new Sum Lazy Segment Tree with default values. The tree can store a range of [1, n].
  /// support sum add update and sum query
  ///
  /// ### Usage
  // let mut st = SumLazySegTree::new(
  //     n,
  //     0,
  //     0,
  //     |left, right| left + right,
  //     |left, right, start, end| left + right * (end - start + 1),
  //   );
  pub fn new(n: usize, default: T, trash_val: T, op: F, up: U) -> Self {
    Self {
      n: n as i32,
      default,
      trash_val,
      st: vec![default; 4 * n + 5],
      lazy: vec![default; 4 * n + 5],
      op,
      up,
    }
  }

  fn _from(&mut self, array: &Vec<T>, id: usize, start: i32, end: i32) {
    self.lazy[id] = self.default;
    if start == end {
      if start - 1 < array.len() as i32 {
        self.st[id] = array[(start - 1) as usize];
      }
      return;
    }
    let mid = (start + end) >> 1;
    self._from(array, id * 2, start, mid);
    self._from(array, id * 2 | 1, mid + 1, end);
    self.st[id] = (self.op)(self.st[id * 2], self.st[id * 2 | 1]);
  }

  fn _push(&mut self, id: usize, start: i32, end: i32) {
    if self.lazy[id] == self.default {
      return;
    }
    let mid = (start + end) >> 1;
    self.st[id * 2] = (self.up)(self.st[id * 2], self.lazy[id], start, mid);
    self.st[id * 2 | 1] = (self.up)(self.st[id * 2 | 1], self.lazy[id], mid + 1, end);
    self.lazy[id * 2] = (self.op)(self.lazy[id * 2], self.lazy[id]);
    self.lazy[id * 2 | 1] = (self.op)(self.lazy[id * 2 | 1], self.lazy[id]);
    self.lazy[id] = self.default;
  }

  fn _update(&mut self, id: usize, start: i32, end: i32, l: i32, r: i32, v: T) {
    if start > end || r < start || l > end {
      return;
    } else if start >= l && end <= r {
      self.lazy[id] = (self.op)(self.lazy[id], v);
      self.st[id] = (self.up)(self.st[id], v, start, end);
      return;
    }
    self._push(id, start, end);
    let mid = (start + end) >> 1;
    self._update(id * 2, start, mid, l, r, v);
    self._update(id * 2 | 1, mid + 1, end, l, r, v);
    self.st[id] = (self.op)(self.st[id * 2], self.st[id * 2 | 1]);
  }

  fn _query(&mut self, id: usize, start: i32, end: i32, l: i32, r: i32) -> T {
    if start > end || start > r || end < l {
      return self.trash_val;
    }
    if start >= l && end <= r {
      return self.st[id];
    }
    self._push(id, start, end);
    let mid = (start + end) >> 1;
    let left = self._query(id * 2, start, mid, l, r);
    let right = self._query(id * 2 | 1, mid + 1, end, l, r);
    (self.op)(left, right)
  }

  fn _max_right<G>(
    &mut self,
    id: usize,
    start: i32,
    end: i32,
    left_bound: i32,
    g: G,
    val: T,
  ) -> Option<usize>
  where
    G: Clone + Copy,
    G: Fn(T, T) -> bool,
  {
    if !g(self.st[id], val) || end < left_bound {
      return None;
    }
    if start == end {
      return Some(start as usize);
    }
    self._push(id, start, end);
    let mid = (start + end) >> 1;
    let mut find_val: Option<usize> = None;
    if g(self.st[id * 2 | 1], val) {
      find_val = self._max_right(id * 2 | 1, mid + 1, end, left_bound, g, val);
    }
    if find_val.is_none() {
      find_val = self._max_right(id * 2, start, mid, left_bound, g, val);
    }
    find_val
  }

  fn _min_left<G>(
    &mut self,
    id: usize,
    start: i32,
    end: i32,
    right_bound: i32,
    g: G,
    val: T,
  ) -> Option<usize>
  where
    G: Clone + Copy,
    G: Fn(T, T) -> bool,
  {
    if !g(self.st[id], val) || start > right_bound {
      return None;
    }
    if start == end {
      return Some(start as usize);
    }
    self._push(id, start, end);
    let mid = (start + end) >> 1;
    let mut find_val: Option<usize> = None;
    if g(self.st[id * 2], val) {
      find_val = self._min_left(id * 2, start, mid, right_bound, g, val);
    }
    if find_val.is_none() {
      find_val = self._min_left(id * 2 | 1, mid + 1, end, right_bound, g, val);
    }
    find_val
  }

  pub fn update(&mut self, i: usize, v: T) {
    self._update(1, 1, self.n, i as i32, i as i32, v);
  }

  pub fn update_range(&mut self, l: usize, r: usize, v: T) {
    self._update(1, 1, self.n, l as i32, r as i32, v);
  }

  pub fn query(&mut self, l: usize, r: usize) -> T {
    self._query(1, 1, self.n, l as i32, r as i32)
  }

  // F usually min function (g should return true when first enter the function, compare with op(st))
  pub fn max_right<G>(&mut self, l: usize, g: G, val: T) -> Option<usize>
  where
    G: Clone + Copy,
    G: Fn(T, T) -> bool,
  {
    self._max_right(1, 1, self.n, l as i32, g, val)
  }

  // F usually max function (g should return true when first enter the function, compare with op(st))
  pub fn min_left<G>(&mut self, r: usize, g: G, val: T) -> Option<usize>
  where
    G: Clone + Copy,
    G: Fn(T, T) -> bool,
  {
    self._min_left(1, 1, self.n, r as i32, g, val)
  }

  // zero-indexed array
  pub fn from(&mut self, array: &Vec<T>) {
    self._from(array, 1, 1, self.n);
  }

  pub fn get_array(&mut self) -> Vec<T> {
    let mut return_array: Vec<T> = vec![self.default; self.n as usize];
    for (idx, val) in return_array.iter_mut().enumerate() {
      *val = self.query(idx + 1, idx + 1);
    }
    return_array
  }
}
