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
  let n: usize = scan.next();
  let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();
  let mut cnt_1: Vec<usize> = vec![0; n + 1];
  let mut cnt_2: Vec<usize> = vec![0; n + 1];
  let mut sum_1 = 0;
  let mut sum_2 = 1;
  for i in 0..n - 1 {
    if cnt_1[a[i]] == 0 {
      sum_1 += 1;
    }
    cnt_1[a[i]] += 1;
  }
  cnt_2[a[n - 1]] += 1;
  let mut ans = sum_1 + sum_2;
  for i in (1..n - 1).rev() {
    cnt_1[a[i]] -= 1;
    if cnt_1[a[i]] == 0 {
      sum_1 -= 1;
    }
    if cnt_2[a[i]] == 0 {
      sum_2 += 1;
    }
    cnt_2[a[i]] += 1;
    ans = ans.max(sum_1 + sum_2);
  }
  println!("{}", ans);
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
