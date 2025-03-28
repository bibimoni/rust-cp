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
    let _: usize = scan.next();
    let s = scan.next::<String>().chars().collect::<Vec<_>>();
    let mut bkac: Vec<char> = vec![];
    let mut gdsc: Vec<char> = vec![];

    let mut state = 0;

    for &c in s.iter() {
      for &x in ['B', 'K', 'A', 'C'].iter() {
        if c == x {
          bkac.push(c);
        }
      }
      for &x in ['G', 'D', 'S', 'C'].iter() {
        if c == x {
          gdsc.push(c);
        }
      }
      bkac.sort();
      bkac.dedup();
      gdsc.sort();
      gdsc.dedup();
      if bkac == ['A', 'B', 'C', 'K'] {
        state |= 1;
      }
      if gdsc == ['C', 'D', 'G', 'S'] {
        state |= 2;
      }
      if state > 0 {
        break;
      }
    }
    println!(
      "{}",
      if state == 1 {
        "BKAC"
      } else if state == 2 {
        "GDSC"
      } else {
        "DRAW"
      }
    );
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
