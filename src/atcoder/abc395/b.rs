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
  let n = scan.next::<usize>();
  let mut arr: Vec<Vec<char>> = vec![vec!['.'; n]; n];
  let mut modify = |len: usize| {
    let lt = n / 2 - len / 2;
    let rt = n / 2 + len / 2 - if n % 2 == 0 { 1 } else { 0 };
    for i in lt..=rt {
      arr[lt][i] = '#';
      arr[rt][i] = '#';
      arr[i][lt] = '#';
      arr[i][rt] = '#';
    }
  };
  let mut len: i32 = n as i32;
  while len > 0 {
    modify(len as usize);
    len -= 4;
  }
  for i in 0..n {
    for j in 0..n {
      print!("{}", arr[i][j]);
    }
    println!();
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
