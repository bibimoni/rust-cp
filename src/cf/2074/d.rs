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
    let (n, _): (usize, usize) = (scan.next(), scan.next());
    let x: Vec<i64> = (0..n).map(|_| scan.next()).collect();
    let r: Vec<i64> = (0..n).map(|_| scan.next()).collect();
    let mut mp: BTreeMap<i64, i64> = BTreeMap::new();
    for i in 0..n {
      for p in (x[i] - r[i])..=(x[i] + r[i]) {
        let to = i64::abs(x[i] - p);
        let mut dist = f64::sqrt((r[i] * r[i] - to * to) as f64) as i64;
        dbg!(p, dist);
        mp.entry(p)
          .and_modify(|d| *d = *d.max(&mut dist))
          .or_insert(dist);
      }
    }
    let mut ans = 0;
    for (_, v) in mp.into_iter() {
      ans += 2 * v + 1;
    }
    println!("{ans}");
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
