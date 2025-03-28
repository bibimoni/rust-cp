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

fn p_function(s: &[char]) -> Vec<usize> {
  let n = s.len();
  let mut pi: Vec<usize> = vec![0; n];
  for i in 1..n {
    let mut j = pi[i - 1];
    while j > 0 && s[i] != s[j] {
      j = pi[j - 1];
    }
    if s[i] == s[j] {
      j += 1;
    }
    pi[i] = j;
  }
  pi
}

fn main() {
  let mut scan = Scan::default();
  let s: Vec<char> = scan.next::<String>().chars().collect();
  let mut t: Vec<char> = Vec::new();
  for &c in s.iter().rev() {
    t.push(c);
  }
  for &c in s.iter() {
    t.push(c);
  }

  let n = s.len();
  let p = p_function(&t);
  let len = p[t.len() - 1].min(n);
  for c in &s {
    print!("{c}");
  }
  for i in (n..(t.len() - len)).rev() {
    print!("{}", t[i]);
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
