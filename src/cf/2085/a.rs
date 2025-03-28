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
  let tt: usize = scan.next();
  for _ in 0..tt {
    let n: usize = scan.next();
    let k: usize = scan.next();
    let mut s = scan.next::<String>().chars().collect::<Vec<_>>();
    fn find(st: usize, s: &[char]) -> usize {
      let mut idx = st;
      let mut cur = s[st];
      for i in st..s.len() {
        if s[i] < cur {
          idx = i;
          cur = s[i];
        }
      }
      idx
    }
    let mut t = s.clone();
    t.reverse();
    fn cmp(a: &[char], b: &[char]) -> bool {
      for i in 0..a.len() {
        if a[i] != b[i] {
          return a[i] < b[i];
        }
      }
      false
    }
    for i in 0..min(k, n) {
      let mut tmp = s.clone();
      let mut idx = find(i, &s);
      while idx == i {
        idx = find(i, &s);
      }
      dbg!(idx);
      tmp[idx] = s[i];
      tmp[i] = s[idx];
      s = tmp;
    }
    println!("{}", if cmp(&s, &t) { "YES" } else { "NO" });
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
