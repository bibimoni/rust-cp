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
  let stdout = std::io::stdout();
  let mut writer = std::io::BufWriter::new(stdout.lock());

  let n: usize = scan.next();
  let c: Vec<Vec<char>> = (0..n)
    .map(|_| scan.next::<String>().chars().collect())
    .collect();

  let mut a: Vec<Vec<usize>> = vec![vec![usize::MAX; n]; n];
  let mut dq: VecDeque<(usize, usize)> = VecDeque::new();
  for i in 0..n {
    dq.push_back((i, i));
    a[i][i] = 0;
  }
  for i in 0..n {
    for j in 0..n {
      if c[i][j] != '-' {
        a[i][j] = a[i][j].min(1);
        dq.push_back((i, j));
      }
    }
  }

  while !dq.is_empty() {
    let (u, v) = dq.pop_front().unwrap();
    for i in 0..n {
      for j in 0..n {
        if c[i][u] != '-' && c[i][u] == c[v][j] && a[i][j] == usize::MAX {
          a[i][j] = a[i][j].min(a[u][v] + 2);
          dq.push_back((i, j));
        }
      }
    }
  }
  for i in 0..n {
    for j in 0..n {
      print!(
        "{} ",
        if a[i][j] == usize::MAX {
          -1
        } else {
          a[i][j] as i64
        }
      );
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
