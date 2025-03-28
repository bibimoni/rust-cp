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

  let n: usize = scan.next();
  let m: usize = scan.next();
  let edges: Vec<(usize, usize, usize)> = (0..m)
    .map(|_| (scan.next(), scan.next(), scan.next()))
    .collect();
  let mut floyd: Vec<Vec<usize>> = vec![vec![usize::MAX; n + 1]; n + 1];
  for &(u, v, w) in edges.iter() {
    floyd[u][v] = floyd[u][v].min(w);
    floyd[v][u] = floyd[v][u].min(w);
  }
  for u in 1..=n {
    floyd[u][u] = 0usize;
  }
  for k in 1..=n {
    for u in 1..=n {
      for v in 1..=n {
        if floyd[u][k] == usize::MAX || floyd[k][v] == usize::MAX {
          continue;
        }
        floyd[u][v] = min(floyd[u][v], floyd[u][k] + floyd[k][v]);
      }
    }
  }

  let q: usize = scan.next();
  for _ in 0..q {
    let k: usize = scan.next();
    let mut bridges: Vec<usize> = (0..k).map(|_| scan.next::<usize>() - 1).collect();
    bridges.sort();
    let mut ans = usize::MAX;
    loop {
      for mask in 0..(1 << k) {
        let mut cur = 0usize;
        let mut start = 1;
        for (j, &bridge) in bridges.iter().enumerate() {
          let (mut u, mut v, w) = edges[bridge];
          if (mask >> j) & 1 == 1 {
            swap(&mut u, &mut v);
          }
          cur += floyd[start][u] + w;
          start = v;
        }
        cur += floyd[start][n];
        ans = ans.min(cur);
      }
      if !next_permutation(&mut bridges) {
        break;
      }
    }
    writeln!(writer, "{}", ans).ok();
  }
}

/// The type whose permutation is required must implement [Ord](std::cmp::Ord) trait.
/// source: satylogin
pub fn next_permutation<T>(arr: &mut [T]) -> bool
where
  T: std::cmp::Ord,
{
  use std::cmp::Ordering;

  // find 1st pair (x, y) from back which satisfies x < y
  let last_ascending = match arr.windows(2).rposition(|w| w[0] < w[1]) {
    Some(i) => i,
    None => {
      arr.reverse();
      return false;
    }
  };

  let swap_with = arr[last_ascending + 1..]
    .binary_search_by(|n| match arr[last_ascending].cmp(n) {
      Ordering::Equal => Ordering::Greater,
      ord => ord,
    })
    .unwrap_err();
  arr.swap(last_ascending, last_ascending + swap_with);
  arr[last_ascending + 1..].reverse();
  true
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
