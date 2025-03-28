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
  let mut dq: VecDeque<char> = VecDeque::new();
  let mut ok = true;

  for &c in s.iter() {
    if c == '[' || c == '(' || c == '<' {
      dq.push_back(c);
    }
    macro_rules! check {
      ($cx: expr , $cxx: expr) => {
        if c == $cx {
          if dq.is_empty() {
            ok = false;
            break;
          } else {
            let x = dq.pop_back().unwrap();
            if x != $cxx {
              ok = false;
              break;
            }
          }
        }
      };
    }
    check!(']', '[');
    check!(')', '(');
    check!('>', '<');
  }
  println!("{}", if ok && dq.is_empty() { "Yes" } else { "No" });
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
