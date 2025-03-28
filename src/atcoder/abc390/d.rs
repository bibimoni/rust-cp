#![allow(unexpected_cfgs, unused_macros, unused_imports)]
/**
 * Author: distiled
 */
use std::{
  cmp::*,
  io::{stdin, Write},
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
  let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();

  struct Env {
    arr: Vec<usize>,
    res: Vec<usize>,
    groups: Vec<usize>,
  }

  let res: Vec<usize> = vec![];
  let gr: Vec<usize> = a.clone();
  let mut env = Env {
    arr: a,
    res,
    groups: gr,
  };
  fn dfs(env: &mut Env, i: usize) {
    if i == env.arr.len() {
      env.groups.resize(env.arr.len(), 0);
      env.res.push(env.groups.iter().fold(0, |acc, &x| acc ^ x));
      return;
    }

    for j in 0..env.groups.len() {
      env.groups[j] += env.arr[i];
      env.groups[i] -= env.arr[i];
      dfs(env, i + 1);
      env.groups[j] -= env.arr[i];
      env.groups[i] += env.arr[i];
    }
  }

  dfs(&mut env, 0);
  env.res.sort();
  env.res.dedup();
  writeln!(writer, "{}", env.res.len()).ok();
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
