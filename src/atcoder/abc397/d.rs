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

fn is_perfect_square(n: i128) -> bool {
  let root = (n as f64).sqrt() as i128;
  root.pow(2) == n
}

fn main() {
  let mut scan = Scan::default();
  let n: i128 = scan.next();
  let mut ans_1 = -1;
  let mut ans_2 = -1;

  let bound = ((4i128 * n) as f64).cbrt() as i128 + 10;
  for d in 1..=bound {
    if n % d != 0 {
      continue;
    }
    let k = n / d;
    let delta = -3 * d * d + 12 * k;
    if delta < 0 || !is_perfect_square(delta) {
      continue;
    }
    let mut y = -3 * d + ((delta as f64).sqrt() as i128);
    if y <= 0 || y % 6 != 0 {
      continue;
    }
    y /= 6i128;
    if y <= 0 {
      continue;
    }
    ans_1 = y + d;
    ans_2 = y;
    dbg!(n, d, delta, ans_1, ans_2);
  }

  if ans_1 == -1 {
    println!("-1");
  } else {
    println!("{} {}", ans_1, ans_2);
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
