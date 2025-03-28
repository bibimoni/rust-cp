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
  let q: usize = scan.next();

  let mut pos = (0..=n).collect::<Vec<_>>();
  let mut house = (0..=n).collect::<Vec<_>>();
  let mut mp_house = (0..=n).collect::<Vec<_>>();
  for _ in 0..q {
    let t: usize = scan.next();
    if t == 1 {
      let p: usize = scan.next();
      let a: usize = scan.next();
      pos[p] = mp_house[a];
    } else if t == 2 {
      let org_a = scan.next::<usize>();
      let org_b = scan.next::<usize>();
      let a: usize = mp_house[org_a];
      let b: usize = mp_house[org_b];
      (house[a], house[b]) = (house[b], house[a]);
      (mp_house[org_b], mp_house[org_a]) = (mp_house[org_a], mp_house[org_b]);
    } else {
      let a: usize = scan.next();
      println!("{}", house[pos[a]]);
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
