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
    let q: usize = scan.next();
    let mut dq: VecDeque<(usize, usize)> = VecDeque::new();
    for _ in 0..q {
      let t: usize = scan.next();
      dbg!(&dq);
      if t == 1 {
        let (x, a): (usize, usize) = (scan.next(), scan.next());
        dq.push_front((x, a));
      } else {
        let mut c: usize = scan.next();
        let mut ans = 0;
        while c > 0 {
          let (mut x, a): (usize, usize) = dq.pop_back().unwrap();
          if x <= c {
            c -= x;
            ans += x * a;
          } else {
            x -= c;
            ans += c * a;
            c = 0;
            dq.push_back((x, a));
          }
        }
        println!("{}", ans);
      }
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
