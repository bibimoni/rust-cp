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
  let (n, m): (usize, usize) = (scan.next(), scan.next());
  let mut b: Vec<i64> = (0..n).map(|_| scan.next()).collect();
  let mut w: Vec<i64> = (0..m).map(|_| scan.next()).collect();
  b.sort_by(|a, b| b.cmp(a));
  w.sort_by(|a, b| b.cmp(a));
  let mut sum = 0;
  for i in 0..m {
    if w[i] < 0 {
      w[i] = if i == 0 { 0 } else { w[i - 1] };
    } else if i > 0 {
      w[i] += w[i - 1];
    }
  }
  let mut ans = 0;
  let mut cur = 0;
  for i in 0..n {
    cur += b[i];
    ans = ans.max(cur + w[min(i, m - 1)]);
  }
  println!("{ans}");
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
