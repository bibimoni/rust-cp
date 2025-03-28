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
    dbg!(n);
    for i in 1..=n {
      for j in 1..=m {
        a[i][j] = scan.next();
      }
    }

    fn rect(arr: &[Vec<usize>], a: usize, b: usize, x: usize, y: usize) -> usize {
      let mut ans = 0;
      for i in x..=a {
        for j in y..=b {
          ans += arr[i][j];
        }
      }
      // dbg!(a, b, x, y, ans);
      ans
    };

    fn ok(a: usize, b: usize, c: usize, d: usize) -> bool {
      b < c || d < a
    }

    let mut ans = 0;
    for i in k..=n {
      for j in k..=m {
        for x in k..=n {
          for y in k..=m {
            if ok(i - k + 1, i, x - k + 1, x) || ok(j - k + 1, j, y - k + 1, y) {
              dbg!(i, j, x, y, rect(&a, i, j, i - k + 1, j - k + 1) + rect(&a, x, y, x - k + 1, y - k + 1));
              ans = ans
                .max(rect(&a, i, j, i - k + 1, j - k + 1) + rect(&a, x, y, x - k + 1, y - k + 1));
            }
          }
        }
      }
    }
    println!("{}", ans);
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
