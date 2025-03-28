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

  let tt: usize = scan.next();
  for _ in 0..tt {
    let n: usize = scan.next();
    let m: usize = scan.next();
    let k: usize = scan.next();
    let mut a: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
      for j in 1..=m {
        a[i][j] = scan.next();
      }
    }

    fn rect_1(pre: &[Vec<i64>], a: usize, b: usize, x: usize, y: usize) -> i64 {
      pre[a][b] - pre[x - 1][b] - pre[a][y - 1] + pre[x - 1][y - 1]
    }

    fn rect_2(pre: &[Vec<i64>], a: usize, b: usize, x: usize, y: usize) -> i64 {
      pre[a][b] - pre[x + 1][b] - pre[a][y + 1] + pre[x + 1][y + 1]
    }

    let mut p1: Vec<Vec<i64>> = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
      for j in 1..=m {
        p1[i][j] = p1[i - 1][j] + p1[i][j - 1] - p1[i - 1][j - 1] + a[i][j] as i64;
      }
    }
    let mut p2: Vec<Vec<i64>> = vec![vec![0; m + 2]; n + 2];
    for i in (1..=n).rev() {
      for j in (1..=m).rev() {
        p2[i][j] = p2[i + 1][j] + p2[i][j + 1] - p2[i + 1][j + 1] + a[i][j] as i64;
      }
    }

    let mut p1k: Vec<Vec<i64>> = vec![vec![0; m + 1]; n + 1];
    for i in k..=n {
      for j in k..=m {
        p1k[i][j] = rect_1(&p1, i, j, i - k + 1, j - k + 1)
          .max(p1k[i - 1][j])
          .max(p1k[i][j - 1]);
      }
    }
    let mut p2k: Vec<Vec<i64>> = vec![vec![0; m + 2]; n + 2];
    for i in (1..=(n - k + 1)).rev() {
      for j in (1..=(m - k + 1)).rev() {
        p2k[i][j] = rect_2(&p2, i, j, i + k - 1, j + k - 1)
          .max(p2k[i + 1][j])
          .max(p2k[i][j + 1]);
      }
    }

    let mut ans = 0;
    for i in k..=(n - k) {
      ans = ans.max(p1k[i][m] + p2k[i + 1][1]);
    }
    for j in k..=(m - k) {
      ans = ans.max(p1k[n][j] + p2k[1][j + 1]);
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
