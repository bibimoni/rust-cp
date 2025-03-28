#![allow(unexpected_cfgs, unused_macros, unused_imports)]
#![allow(clippy::needless_range_loop)]
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
  let n: usize = scan.next();
  let m: usize = scan.next();
  let edges: Vec<(usize, usize, usize)> = (0..m)
    .map(|_| (scan.next(), scan.next(), scan.next()))
    .collect();
  let mut adj: Vec<Vec<(usize, usize)>> = vec![vec![]; n + 1];
  for &(u, v, w) in edges.iter() {
    adj[u].push((v, w));
    adj[v].push((u, w));
  }
  struct Env {
    adj: Vec<Vec<(usize, usize)>>,
    vis: Vec<bool>,
    ans: usize,
  };
  let mut e: Env = Env {
    adj,
    vis: vec![false; n + 1],
    ans: usize::MAX,
  };
  fn dfs(e: &mut Env, u: usize, cur_xor: usize) {
    e.vis[u] = true;
    if u == e.adj.len() - 1 {
      e.ans = e.ans.min(cur_xor);
      return;
    }
    let child = e.adj[u].to_vec();
    for &(v, w) in &child {
      if !e.vis[v] {
        dfs(e, v, cur_xor ^ w);
        e.vis[v] = false;
      }
    }
  }
  dfs(&mut e, 1, 0);
  println!("{:?}", e.ans);
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
