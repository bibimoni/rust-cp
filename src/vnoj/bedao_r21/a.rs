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

  let is_p = |x| -> bool {
    if x <= 1 {
      return false;
    }
    let mut ok = true;
    let mut i = 2;
    while i * i <= x {
      if x % i == 0 {
        ok = false;
      }
      i += 1;
    }
    ok
  };

  let t: usize = scan.next();
  for _ in 0..t {
    let x: usize = scan.next();
    let s_x = (x as f64).sqrt() as usize;
    let mut ok = false;
    for i in 2..=100 {
      if !is_p(i) {
        continue;
      }
      let mut pw8 = 1usize;
      for _ in 0..8 {
        pw8 *= i;
      }
      ok |= pw8 == x;
    }
    if ok || s_x * s_x != x {
      println!("{}", if ok { "YES" } else { "NO" });
    } else {
      let ss_x = (s_x as f64).sqrt() as usize;
      for val in 2..=ss_x {
        if is_p(val) && s_x % val == 0 && is_p(s_x / val) && s_x != val * val {
          ok |= true;
          break;
        }
      }
      println!("{}", if ok { "YES" } else { "NO" });
    }
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
