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

  let s = scan.next::<String>().chars().collect::<Vec<_>>();

  let check_i = |i| -> (bool, usize) {
    let mut ok = true;
    let l = (s[i] as usize - 'a' as usize) + 1;
    if i + l >= s.len() {
      return (false, 0);
    }
    for j in i..i + l {
      ok &= s[i] == s[j];
    }
    (ok, if ok { l } else { 0 })
  };
  let mut dp: Vec<Vec<usize>> = vec![vec![0; s.len()]; 26];
  let mut ans = 0;
  for c in 'a'..='z' {
    let j = c as usize - 'a' as usize;
    for i in 0..s.len() {
      if i > 0 {
        dp[j][i] = dp[j][i].max(dp[j][i - 1]);
      }
      let (ok, l) = check_i(i);
      if ok && c == s[i] {
        dp[j][i + l - 1] = max(dp[j][i + l - 1], l);
        if i > 0 {
          for k in 0..j {
            dbg!(i, j, k, l, dp[k][i - 1]);
            dp[j][i + l - 1] = dp[j][i + l - 1].max(dp[k][i - 1] + l);
          }
        }
      }
    }
    dbg!(c, &dp[j]);
    ans = ans.max(*dp[j].iter().max().unwrap());
  }
  println!("{ans}");
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
