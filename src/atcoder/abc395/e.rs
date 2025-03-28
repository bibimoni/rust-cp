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
  let x: usize = scan.next();
  let mut adj: Vec<Vec<(usize, usize)>> = vec![vec![]; 2 * n + 5];
  for _ in 0..m {
    let u: usize = scan.next();
    let v: usize = scan.next();
    adj[u].push((v, 1));
    adj[v + n].push((u + n, 1));
  }
  for u in 1..=n {
    adj[u].push((u + n, x));
    adj[u + n].push((u, x));
  }
  let mut dist = vec![usize::MAX; 2 * n + 5];
  let mut heap: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
  heap.push((Reverse(0), 1));
  dist[1] = 0;
  while !heap.is_empty() {
    let (Reverse(w), u) = heap.pop().unwrap();
    if dist[u] != w {
      continue;
    }
    for &(v, w_1) in &adj[u] {
      if dist[u] + w_1 < dist[v] {
        dist[v] = dist[u] + w_1;
        heap.push((Reverse(dist[v]), v));
      }
    }
  }
  println!("{}", dist[n].min(dist[2 * n]));
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
