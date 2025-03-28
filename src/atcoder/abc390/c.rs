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
  
  let h: usize = scan.next();
  let w: usize = scan.next();
  
  let mut pos_1 = (h, w);
  let mut pos_2 = (0, 0);
  let board : Vec<String> = (0..h).map(|_| scan.next::<String>()).collect();
  
  dbg!(&board);
  
  for i in 0..h {
    for j in 0..w {
      if board[i].chars().nth(j).unwrap() == '#' {
        pos_1 = (pos_1.0.min(i), pos_1.1.min(j));
        pos_2 = (pos_2.0.max(i), pos_2.1.max(j));
      }
    }
  }
  
  let mut ok = true;
  for i in 0..h {
    for j in 0..w {
      if board[i].chars().nth(j).unwrap() == '.' && pos_1.0 <= i && i <= pos_2.0 && pos_1.1 <= j && j <= pos_2.1 {
        ok = false
      }
    }
  }
  writeln!(writer, "{}", if ok { "Yes\n" } else { "No\n" }).ok();
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