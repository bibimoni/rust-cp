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
  let x: usize = scan.next();
  let mut v: Vec<usize> = vec![0; n + 1];
  let mut a: Vec<usize> = vec![0; n + 1];
  let mut c: Vec<usize> = vec![0; n + 1];
  
  for i in 1..=n {
    v[i] = scan.next::<usize>() - 1;
    a[i] = scan.next();
    c[i] = scan.next();
  }
  
  let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; x + 1];
  for i in 1..=n {
    let mut ndp: Vec<Vec<usize>> = vec![vec![0; 3]; x + 1];
    for w in 1..=x {
      for vit in 0..3 {
        ndp[w][vit] = max(ndp[w][vit], dp[w][vit]);
      }
      if w >= c[i] {
        ndp[w][v[i]] = max(ndp[w][v[i]], dp[w - c[i]][v[i]] + a[i]);
      }
    }
    dp = ndp;
  }
  
  let mut ans = 0;
  for w_1 in 0..=x {
    for w_2 in 0..=(x - w_1) {
      let w_3 = x - w_1 - w_2;
      ans = max(ans, dp[w_1][0].min(dp[w_2][1].min(dp[w_3][2])));
    }
  }
  
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