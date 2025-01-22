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
  let k: i64 = scan.next();
  let mut a: Vec<i64> = vec![0; n + 2];
  for val in a.iter_mut().take(n + 1).skip(1) {
    *val = scan.next();
  }
  
  const MOD: i64 = 998244353;
  let mut map: BTreeMap<i64, i64> = BTreeMap::new();

  map.insert(0, 1);
  let mut ans: i64 = 0;
  let mut sum_so_far: i64 = 0;
  let mut total_ans_so_far: i64 = 1;

  for v in a.into_iter().take(n + 1).skip(1) {
    sum_so_far += v;
    let equal_k = match map.get(&(sum_so_far - k)) {
      Some(val) => *val,
      None => 0,
    };
    ans = (total_ans_so_far - equal_k + MOD) % MOD;
    total_ans_so_far += ans;
    total_ans_so_far %= MOD;
    *map.entry(sum_so_far).or_insert(0) += ans;    
    *map.entry(sum_so_far).or_insert(0) %= MOD;    
  }
  
  writeln!(writer, "{ans}").ok();
}

#[derive(Default)] //{{{
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
}
 
// Helpers
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
struct IOrd<T>(T);
impl<T: Ord> Ord for IOrd<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    other.0.cmp(&self.0)
  }
}
 
impl<T: PartialOrd> PartialOrd for IOrd<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.0.partial_cmp(&self.0)
  }
} //}}}
// template source (darkkcyan)