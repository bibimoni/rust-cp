#![allow(unexpected_cfgs, unused_macros, unused_imports)]
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
  let stdout = std::io::stdout();
  let mut writer = std::io::BufWriter::new(stdout.lock());

  let n: usize = scan.next();
  let m: usize = scan.next();
  let mut deg: Vec<usize> = vec![0; n + 1];
  let mut to: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; n + 1];
  let mut from: Vec<Vec<usize>> = vec![vec![]; n + 1];
  for _ in 0..m {
    let u: usize = scan.next();
    let v: usize = scan.next();
    let b: usize = scan.next();
    let c: usize = scan.next();
    deg[v] += 1;
    to[v].push((u, b, c));
    from[u].push(v);
  }

  let mut q: VecDeque<usize> = VecDeque::new();
  for (u, &val) in deg.iter().enumerate().skip(1) {
    if val == 0 {
      q.push_back(u);
    }
  }
  let mut cands: Vec<usize> = vec![];
  while !q.is_empty() {
    let u = q.pop_front().unwrap();
    cands.push(u);
    for &v in from[u].iter() {
      deg[v] -= 1;
      if deg[v] == 0 {
        q.push_back(v);
      }
    }
  }
  assert_eq!(cands.len(), n);

  #[derive(Copy, Clone)]
  struct I {
    b: usize,
    c: usize,
  }

  impl I {
    fn new(b: usize, c: usize) -> I {
      I { b, c }
    }

    fn val(&self) -> f64 {
      if self.b == 0 || self.c == 0 {
        0f64
      } else {
        self.b as f64 / self.c as f64
      }
    }

    fn new_add(&self, n_b: usize, n_c: usize) -> I {
      I {
        b: self.b + n_b,
        c: self.c + n_c,
      }
    }

    fn cmp(&self, other: &I) -> I {
      if other.val() > self.val() {
        *other
      } else {
        *self
      }
    }
  }

  let mut dp: Vec<I> = vec![I::new(0, 0); n + 1];
  for &cand in cands.iter() {
    for &(v, b_cand, c_cand) in to[cand].iter() {
      dp[cand] = dp[cand].cmp(&dp[v].new_add(b_cand, c_cand));
    }
  }
  writeln!(writer, "{}", dp[n].val()).ok();
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
