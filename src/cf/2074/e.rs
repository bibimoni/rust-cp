#![allow(unexpected_cfgs, unused_macros, unused_imports)]
#![allow(clippy::needless_range_loop)]
/**
 * Author: distiled
 */
use std::{
  cmp::*,
  collections::*,
  io::{stderr, stdin, stdout, Write},
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

  let tt: usize = scan.next();
  for _ in 0..tt {
    let mut ask = |a: usize, b: usize, c: usize| {
      println!("? {} {} {}", a, b, c);
      stdout().flush().expect("panic");
      let x: usize = scan.next();
      x
    };
    let mut i = 1;
    let mut j = 2;
    let mut k = 3;
    let mut rnd = Random::new(787788);
    let mut v = ask(i, j, k);
    while v != 0 {
      let x = rnd.next_in_range(0, 3);
      if x == 1 {
        i = v
      } else if x == 2 {
        j = v
      } else {
        k = v
      }
      v = ask(i, j, k);
    }
    println!("! {} {} {}", i, j, k);
    stdout().flush().expect("panic");
  }
}

#[allow(dead_code)]
struct Random {
  state: usize,
}

impl Random {
  fn next(&mut self) -> usize {
    let mut x = self.state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    self.state = x;
    x
  }

  #[allow(dead_code)]
  fn next_in_range(&mut self, from: usize, to: usize) -> usize {
    assert!(from < to);
    from + self.next() % (to - from)
  }

  #[allow(dead_code)]
  fn next_double(&mut self) -> f64 {
    (self.next() as f64) / (std::usize::MAX as f64)
  }

  #[allow(dead_code)]
  fn new(seed: usize) -> Self {
    assert_ne!(seed, 0);
    Self { state: seed }
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
