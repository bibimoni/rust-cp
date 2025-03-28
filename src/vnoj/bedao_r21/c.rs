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
  let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();

  let mut hs: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
  let mut ans: (i32, i32, i32, i32) = (-1, -1, -1, -1);
  for i in 0..n {
    for j in i + 1..min(n, i + 10) {
      if let Some(arr) = hs.get(&(a[i] + a[j])) {
        for &(u, v) in arr {
          if u != i && u != j && v != i && v != j {
            ans = (i as i32, j as i32, u as i32, v as i32);
          }
        }
      }
      hs.entry(a[i] + a[j]).or_default().push((i, j));
      if hs.entry(a[i] + a[j]).or_default().len() > 2 {
        hs.entry(a[i] + a[j]).or_default().pop();
      }
    }
  }
  if ans.0 == -1 {
    println!("-1");
  } else {
    println!("{} {} {} {}", ans.0 + 1, ans.1 + 1, ans.2 + 1, ans.3 + 1);
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
