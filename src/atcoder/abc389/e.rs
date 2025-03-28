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
  let m: i128 = scan.next();
  let p: Vec<usize> = (0..n).map(|_| scan.next()).collect();

  let mut lo = 1i128;
  let mut hi = m + 1;
  let mut ans = 0;
  let mut ss: i128 = 0;
  let mut xx: i128 = 0;
  while lo <= hi {
    let mid = lo + (hi - lo) / 2;
    // (2k - 1) * p[i] <= mid
    // => k = ((mid / p[i]) + 1) / 2 | ((mid + 1) / p[i] + 1) / 2
    let mut total_unit = 0;
    let mut spent: i128 = 0;
    for &x in p.iter() {
      let k = (mid / x as i128 + 1) / 2;
      spent += k * k * (x as i128);
      total_unit += k;
    }
    if spent > m {
      hi = mid - 1;
    } else {
      lo = mid + 1;
      ans = total_unit;
      xx = mid;
      ss = spent;
    }
  }
  ans += (m - ss) / (xx + 1);
  writeln!(writer, "{}", ans).ok();
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
