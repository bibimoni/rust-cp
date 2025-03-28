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
  let (a, (b, z)): (Vec<usize>, (Vec<usize>, Vec<usize>)) = (0..m)
    .map(|_| (scan.next(), (scan.next(), scan.next())))
    .collect::<Vec<(usize, (usize, usize))>>()
    .iter()
    .cloned()
    .unzip();
  let adj: Vec<Vec<(usize, usize)>> = (0..m).fold(vec![vec![]; n + 1], |mut acc, i| {
    acc[a[i]].push((b[i], z[i]));
    acc[b[i]].push((a[i], z[i]));
    acc
  });

  let mut ans: Vec<usize> = vec![0; n + 1];
  let mut ok = true;

  for bit in 0..30usize {
    if !ok {
      break;
    }
    let mut was: Vec<bool> = vec![false; n + 1];
    let mut val: Vec<usize> = vec![0; n + 1];
    for src in 1..=n {
      if was[src] {
        continue;
      }
      let mut comps: Vec<usize> = vec![];
      let mut dq: VecDeque<usize> = VecDeque::new();
      dq.push_back(src);
      while !dq.is_empty() {
        let u: usize = dq.pop_front().unwrap();
        if was[u] || !ok {
          continue;
        }
        comps.push(u);
        was[u] = true;
        for &(v, w) in adj[u].iter() {
          let bw = (w >> bit) & 1;
          if was[v] && bw ^ val[u] != val[v] {
            ok = false;
            break;
          } else {
            val[v] = bw ^ val[u];
            dq.push_back(v);
          }
        }
      }

      if !ok {
        break;
      }
      let mut sum = 0;
      for &u in &comps {
        sum += val[u];
      }
      if sum < comps.len() - sum {
        for &u in &comps {
          if val[u] > 0 {
            ans[u] |= 1 << bit;
          }
        }
      } else {
        for &u in &comps {
          if val[u] == 0 {
            ans[u] |= 1 << bit;
          }
        }
      }
    }
  }

  if !ok {
    println!("-1");
  } else {
    for u in 1..=n {
      print!("{} ", ans[u]);
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
