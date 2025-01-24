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
  
  let tt = scan.next();
  for _ in 0..tt {
    let n: usize = scan.next();
    let l: usize = scan.next::<usize>() - 1;
    let r: usize = scan.next::<usize>() - 1;
    let a: Vec<i64> = (0..n).map(|_| scan.next()).collect();
    
    let mut seg: Vec<i64> = vec![];
    let mut ls: Vec<i64> = vec![];
    let mut rs: Vec<i64> = vec![];
    
    for i in 0..=r {
      ls.push(a[i]);
    }
    for i in l..n {
      rs.push(a[i]);
    }
    
    ls.sort();
    rs.sort();
    let s_1: i64 = ls.iter().take(r - l + 1).sum();
    let s_2: i64 = rs.iter().take(r - l + 1).sum();
    writeln!(writer, "{}", min(s_1, s_2)).unwrap();
  }
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