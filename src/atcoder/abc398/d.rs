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
  let mut r: i64 = scan.next();
  let mut c: i64 = scan.next();
  let mut offsn_1 = 0i64;
  let mut offsn_2 = 0i64;
  let mut offwe_1 = 0i64;
  let mut offwe_2 = 0i64;
  let mut lsn = '#';
  let mut lwe = '#';
  let s: Vec<char> = scan.next::<String>().chars().collect();
  for &i in &s {
    if i == 'W' {
      c += 1;
      if (lwe == '#') {
        lwe = 'W';
      } else if (lwe == 'E') {
        offwe_1 += 1;
        lwe = 'W'
      }
    } else if i == 'E' {
      c -= 1;
      if (lwe == 'E') {
        lwe = 'W';
      } else if (lwe == 'W') {
        offwe_2 += 1;
        lwe = 'E'
      }
    } else if i == 'S' {
      r -= 1;
      if (lsn == '#') {
        lsn = 'S';
      } else if (lsn == 'N') {
        offsn_1 += 1;
        lsn = 'S';
      }
    } else if i == 'N' {
      r += 1;
      if (lsn == '#') {
        lsn = 'N';
      } else if (lsn == 'S') {
        offsn_2 += 1;
        lsn = 'N';
      }
    }
    dbg!((r, c));
    print!("{}", (r - offsn_1 <= 0 && 0 <= r + offsn_2 && c - offwe_1 <= 0 && 0 <= c + offwe_2) as usize);
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
