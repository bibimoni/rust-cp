#![allow(unexpected_cfgs, unused_macros, unused_imports)]
#![allow(clippy::needless_range_loop)]
/**
 * Author: distiled
 */
use std::{
  cmp::*,
  collections::*,
  io::{stderr, stdin, Write},
  mem::*,
  str::*,
};

macro_rules! dbg {
  ($($arg:tt)*) => { #[cfg(DEBUG)] { std::dbg!($($arg)*); } };
}
macro_rules! eprintln {
  ($($arg:tt)*) => { #[cfg(DEBUG)] { std::eprintln!($($arg)*); } };
}

fn main() {
  let mut scan = Scan::default();

  let h: usize = scan.next();
  let w: usize = scan.next();
  let mut f: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];
  for i in 1..=h {
    for j in 1..=w {
      f[i][j] = scan.next();
    }
  }
  let q: usize = scan.next();
  let mut que: Vec<Vec<usize>> = vec![vec![]; q];
  for i in 0..q {
    que[i] = (0..6).map(|_| scan.next()).collect();
  }

  let mut edges: Vec<Vec<(usize, usize)>> = vec![];
  for i in 1..=h {
    for j in 1..=w {
      if i > 1 {
        edges.push(vec![(i, j), (i - 1, j)]);
      }
      if j > 1 {
        edges.push(vec![(i, j), (i, j - 1)]);
      }
    }
  }

  let calc =
    |arr: &[(usize, usize)]| -> usize { min(f[arr[0].0][arr[0].1], f[arr[1].0][arr[1].1]) };
  edges.sort_by_key(|b| std::cmp::Reverse(calc(b)));

  let conv = |p: (usize, usize)| -> usize { p.0 * w + p.1 };

  let mx: i32 = edges.len() as i32 - 1i32;
  // range of the smallest building in the path connecting 2 buildings at query i
  let (mut l, mut r): (Vec<i32>, Vec<i32>) = (vec![0; q], vec![mx; q]);
  let mut ret: Vec<usize> = vec![mx as usize, q];
  loop {
    let mut ok = false;
    let mut check: Vec<Vec<usize>> = vec![vec![]; mx as usize + 1];
    let mut uf = UnionFind::new(conv((h, w)) + 1);
    for req in 0..q {
      if l[req] > r[req] {
        continue;
      }
      ok = true;
      let mid = (l[req] + r[req]) / 2;
      check[mid as usize].push(req);
    }
    if !ok {
      break;
    }
    for (idx, p) in edges.iter().enumerate() {
      uf.unite(conv(p[0]), conv(p[1]));
      for &req in &check[idx] {
        let b_1 = (que[req][0], que[req][1]);
        let b_2 = (que[req][3], que[req][4]);
        if uf.is_same_set(conv(b_1), conv(b_2)) {
          ret[req] = idx;
          r[req] = idx as i32 - 1;
        } else {
          l[req] = idx as i32 + 1;
        }
      }
    }
  }

  for i in 0..q {
    let f_1 = que[i][2];
    let f_2 = que[i][5];
    let h_min = calc(&edges[ret[i]]).min(f_1).min(f_2);
    println!("{}", f_1 + f_2 - 2 * h_min);
  }
}

use union_find::*;
pub mod union_find {
  pub struct UnionFind {
    parent: Vec<i32>,
    size: usize,
  }
  impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
      let parent = vec![-1; size];
      UnionFind { parent, size }
    }
    /// Returns a (usize, usize) tuple that represents `0` is a new root and `1` is a merged root.
    /// Returns a `None` if `x` and `y` is already merged.        
    pub fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
      let x_root = self.root(x);
      let y_root = self.root(y);
      if x_root == y_root {
        return None;
      }
      let size1 = self.parent[x_root];
      let size2 = self.parent[y_root];
      let (new_root, merged_root) = if size1 >= size2 {
        self.parent[x_root] += size2;
        self.parent[y_root] = x_root as i32;
        (x_root, y_root)
      } else {
        self.parent[y_root] += size1;
        self.parent[x_root] = y_root as i32;
        (y_root, x_root)
      };
      self.size -= 1;
      Some((new_root, merged_root))
    }
    pub fn is_root(&mut self, x: usize) -> bool {
      self.root(x) == x
    }
    pub fn is_same_set(&mut self, x: usize, y: usize) -> bool {
      self.root(x) == self.root(y)
    }
    pub fn root(&mut self, x: usize) -> usize {
      if self.parent[x] < 0 {
        return x;
      }
      let parent = self.parent[x] as usize;
      let root = self.root(parent);
      self.parent[x] = root as i32;
      root
    }
    pub fn union_size(&mut self, x: usize) -> usize {
      let root = self.root(x);
      let set_size = -self.parent[root];
      set_size as usize
    }
    pub fn size(&self) -> usize {
      self.size
    }
  }
  pub struct UndoUnionFind {
    parent: Vec<u32>,
    size: Vec<u32>,
    stack: Vec<Option<(u32, u32)>>,
  }
  impl UndoUnionFind {
    pub fn new(n: usize) -> UndoUnionFind {
      assert!(n < std::u32::MAX as usize);
      let mut res = UndoUnionFind {
        parent: vec![0; n],
        size: vec![1; n],
        stack: vec![],
      };
      res.init();
      res
    }
    pub fn init(&mut self) {
      self.stack.clear();
      for (i, (parent, size)) in self.parent.iter_mut().zip(self.size.iter_mut()).enumerate() {
        *parent = i as u32;
        *size = 1;
      }
    }
    pub fn is_root(&self, x: usize) -> bool {
      x == self.root(x)
    }
    pub fn root(&self, mut x: usize) -> usize {
      assert!(x < self.parent.len());
      while self.parent[x] != x as u32 {
        x = self.parent[x] as usize;
      }
      x
    }
    pub fn is_same_set(&self, x: usize, y: usize) -> bool {
      assert!(x < self.parent.len());
      assert!(y < self.parent.len());
      self.root(x) == self.root(y)
    }
    pub fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
      assert!(x < self.parent.len());
      assert!(y < self.parent.len());
      let mut x = self.root(x);
      let mut y = self.root(y);
      if x == y {
        self.stack.push(None);
        return None;
      }
      if (self.size[x], x) < (self.size[y], y) {
        std::mem::swap(&mut x, &mut y);
      }
      self.size[x] += self.size[y];
      self.parent[y] = x as u32;
      self.stack.push(Some((x as u32, y as u32)));
      Some((x, y))
    }
    pub fn union_size(&self, x: usize) -> usize {
      assert!(x < self.parent.len());
      let r = self.root(x);
      self.size[r] as usize
    }
    pub fn undo(&mut self) -> Option<(usize, usize)> {
      self.stack.pop().unwrap().map(|(x, y)| {
        let x = x as usize;
        let y = y as usize;
        self.size[x] -= self.size[y];
        self.parent[y] = y as u32;
        (x, y)
      })
    }
    pub fn snap(&mut self) {
      self.stack.clear();
    }
    pub fn rollback(&mut self) {
      while !self.stack.is_empty() {
        self.undo();
      }
    }
  }
  /// Weighted Union Find
  pub struct WeightedUnionFind<Abel> {
    par: Vec<usize>,
    rank: Vec<usize>,
    diff_weight: Vec<Abel>,
  }
  impl<Abel> WeightedUnionFind<Abel>
  where
    Abel: std::ops::Add<Output = Abel>
      + std::ops::Sub<Output = Abel>
      + std::ops::AddAssign
      + std::ops::SubAssign
      + std::ops::Neg<Output = Abel>
      + Copy,
  {
    pub fn new(n: usize, sum_unity: Abel) -> Self {
      let mut par = Vec::with_capacity(n);
      let mut rank = Vec::with_capacity(n);
      let mut diff_weight = Vec::with_capacity(n);
      for i in 0..n {
        par.push(i);
        rank.push(0);
        diff_weight.push(sum_unity);
      }
      WeightedUnionFind {
        par,
        rank,
        diff_weight,
      }
    }
    /// return root of x
    pub fn root(&mut self, x: usize) -> usize {
      if self.par[x] == x {
        x
      } else {
        let r = self.root(self.par[x]);
        let w = self.diff_weight[self.par[x]];
        self.diff_weight[x] += w;
        self.par[x] = r;
        r
      }
    }
    /// return weight of x    
    pub fn weight(&mut self, x: usize) -> Abel {
      self.root(x);
      self.diff_weight[x]
    }
    /// return true if x and y are in same set
    pub fn is_same_set(&mut self, x: usize, y: usize) -> bool {
      self.root(x) == self.root(y)
    }
    /// merge x and y with weight(y) = weight(x) + w
    /// return true if x and y are in different set
    pub fn merge(&mut self, mut x: usize, mut y: usize, mut w: Abel) -> bool {
      w += self.weight(x);
      w -= self.weight(y);
      x = self.root(x);
      y = self.root(y);
      if x == y {
        return false;
      }
      if self.rank[x] < self.rank[y] {
        std::mem::swap(&mut x, &mut y);
        w = -w;
      }
      if self.rank[x] == self.rank[y] {
        self.rank[x] += 1;
      }
      self.par[y] = x;
      self.diff_weight[y] = w;
      true
    }
    /// return weight(y) - weight(x)
    /// require x and y are in same set
    pub fn diff(&mut self, x: usize, y: usize) -> Abel {
      self.weight(y) - self.weight(x)
    }
  }
}

#[derive(Default)]
struct Scan {
  buff: Vec<String>,
}
#[allow(dead_code)]
#[allow(unused_variables)]
impl Scan {
  fn next<T: FromStr>(&mut self) -> T {
    self.next_opt().unwrap()
  }

  fn next_opt<T: FromStr>(&mut self) -> Option<T> {
    if let Some(token) = self.buff.pop() {
      return token.parse().ok();
    }
    if let Some(line) = self.read_line() {
      self.buff = line
        .split_ascii_whitespace()
        .map(String::from)
        .rev()
        .collect();
      self.next_opt()
    } else {
      None
    }
  }

  fn read_line(&mut self) -> Option<String> {
    let mut line = String::new();
    stdin().read_line(&mut line).map(|_| line).ok()
  }

  // empty line will be consumed too
  fn read_line_till_empty(&mut self) -> Option<String> {
    self.read_line().filter(|line| !line.is_empty())
  }
} // template source (darkkcyan)
