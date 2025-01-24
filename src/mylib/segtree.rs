pub struct SegTree<T, F, U>
where
  T: Clone + Copy,
  U: Clone + Copy,
  F: Fn(T, T) -> T,
  U: Fn(T, T) -> T,
{
  n: i32,
  default: T,
  trash_val: T,
  st: Vec<T>,
  lazy: Vec<T>,
  op: F,
  up: U,
}

impl<T, F, U> SegTree<T, F, U>
where
  T: Clone + Copy + std::cmp::PartialEq + std::fmt::Debug,
  U: Clone + Copy,
  F: Fn(T, T) -> T,
  U: Fn(T, T) -> T,
{
  /// Constructs a new Segment Tree with default values. The tree can store a range of [1, n].
  ///
  /// ### Usage
  /// let mut tree = SegTree::new(10, i32::MIN, std::cmp::max, |a, b| a + b);
  /// tree.insert(1, 10);
  /// tree.insert(2, 20);
  /// assert_eq!(tree.query(1, 10), 20);
  pub fn new(n: usize, default: T, trash_val: T, op: F, up: U) -> Self {
    Self {
      n: n as i32,
      default,
      trash_val,
      st: vec![default; 4 * n + 5],
      lazy: vec![default; 4 * n + 5],
      op,
      up
    }
  }
  
  fn _push(&mut self, id: usize) {
    if self.lazy[id] == self.default {
      return;
    }
    self.st[id * 2] = (self.up)(self.st[id * 2], self.lazy[id]);
    self.st[id * 2 | 1] = (self.up)(self.st[id * 2 | 1], self.lazy[id]);
    self.lazy[id * 2] = (self.up)(self.lazy[id * 2], self.lazy[id]);
    self.lazy[id * 2 | 1] = (self.up)(self.lazy[id * 2 | 1], self.lazy[id]);
    self.lazy[id] = self.default;
  }

  fn _update(&mut self, id: usize, start: i32, end: i32, l: i32, r: i32, v: T) {
    if start > end || r < start || l > end {
      return;
    } else if start >= l && end <= r {
      self.lazy[id] = (self.up)(self.lazy[id], v);
      self.st[id] = (self.up)(self.st[id], v);
      return;
    }
    self._push(id);
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
    self._push(id);
    let mid = (start + end) >> 1;
    let left = self._query(id * 2, start, mid, l, r);
    let right = self._query(id * 2 | 1, mid + 1, end, l, r);
    (self.op)(left, right)
  }

  pub fn _max_right<G>(&mut self, id: usize, start: i32, end: i32, left_bound: i32, g: G, val: T) -> Option<usize>
  where
    G: Clone + Copy,
    G: Fn(T, T) -> bool
  {
    if !g(self.st[id], val) || end < left_bound {
      return None;
    }
    if start == end {
      return Some(start as usize);
    }
    self._push(id);
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

  pub fn _min_left<G>(&mut self, id: usize, start: i32, end: i32, right_bound: i32, g: G, val: T) -> Option<usize>
  where
    G: Clone + Copy,
    G: Fn(T, T) -> bool
  {
    if !g(self.st[id], val) || start > right_bound {
      return None;
    }
    if start == end {
      return Some(start as usize);
    }
    self._push(id);
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
    G: Fn(T, T) -> bool
  {
    self._max_right(1, 1, self.n, l as i32, g, val)
  }
  
  // F usually max function (g should return true when first enter the function, compare with op(st))
  pub fn min_left<G>(&mut self, r: usize, g: G, val: T) -> Option<usize>
  where
    G: Clone + Copy,
    G: Fn(T, T) -> bool
  {
    self._min_left(1, 1, self.n, r as i32, g, val)
  }
  
  pub fn get_array(&mut self) -> Vec<T> {
    let mut return_array: Vec<T> = vec![self.default; self.n as usize];
    for (idx, val) in return_array.iter_mut().enumerate() {
      *val = self.query(idx + 1, idx + 1);
    }
    return_array
  }
}