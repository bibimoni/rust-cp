// time-limit: 100000
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

fn cmp(v1: Option<usize>, v2: Option<usize>) -> Option<usize> {
  match v1 {
    Some(t1) => {
      match v2 {
        Some(t2) => Some(min(t1, t2)),
        None => Some(t1)
      }
    },
    None => {
      v2
    }
  }
}

fn main() {
  let mut scan = Scan::default();
  let stdout = std::io::stdout();
  let mut writer = std::io::BufWriter::new(stdout.lock());

  let n: usize = scan.next();
  let m: usize = scan.next();
  let q: usize = scan.next();

  let edges: Vec<(usize, usize, usize)> = (0..m)
    .map(|_| (scan.next(), scan.next(), scan.next())).collect();
  
  let que: Vec<(usize, usize, usize)> = (0..q)
    .map(|_| {
      let t: usize = scan.next();
      if t == 1 {
        (t, scan.next::<usize>() - 1, 0)
      } else {
        (t, scan.next(), scan.next())
      }
    }).collect();
  
  let mut mark: Vec<bool> = vec![true; m];
  que.iter()
    .filter(|(first, _, _)| *first == 1)
    .for_each(|(_, i, _)| {
      mark[*i] = false;
    });

  let mut dist: Vec<Vec<Option<usize>>> = vec![vec![None; n + 1]; n + 1];

  for (idx, (u, v, w)) in edges.iter().enumerate() {
    if mark[idx] {
      dist[*u][*v] = Some(*w);
      dist[*v][*u] = Some(*w);
    }
  }

  for (u, row) in dist.iter_mut().take(n + 1).skip(1).enumerate() {
    row[u + 1] = Some(0);
  }

  for k in 1..=n {
    for u in 1..=n {
      for v in 1..n {
        if let (Some(d_uk), Some(d_kv)) = (dist[u][k], dist[k][v]) {
          dist[u][v] = cmp(dist[u][v], Some(d_uk + d_kv));
        }
      }
    }
  }

  let mut ans: Vec<i64> = Vec::new();
  for eq in que.into_iter().rev() {
    let t = eq.0;
    if t == 1 {
      let i = eq.1;
      let (u_e, v_e, w_e) = edges[i];
      dist[u_e][v_e] = cmp(dist[u_e][v_e], Some(w_e));
      dist[v_e][u_e] = dist[u_e][v_e];
      for u in 1..=n {
        for v in 1..=n {
          if let (Some(d_uu), Some(d_vv)) = (dist[u][u_e], dist[v_e][v]) {
            dist[u][v] = cmp(dist[u][v], Some(d_uu + d_vv + w_e));
          }
          if let (Some(d_uv), Some(d_vu)) = (dist[u][v_e], dist[u_e][v]) {
            dist[u][v] = cmp(dist[u][v], Some(d_uv + d_vu + w_e));
          }
        }
      }
    } else {
      let (u, v) = (eq.1, eq.2);
      match dist[u][v] {
        Some(val) => ans.push(val as i64),
        None => ans.push(-1)
      }
    }
  }

  for res in ans.iter().rev() {
    writeln!(writer, "{res}").ok();
  }
}

#[derive(Default)] //{{{
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
}
 
// Helpers
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
struct IOrd<T>(T);
impl<T: Ord> Ord for IOrd<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    other.0.cmp(&self.0)
  }
}
 
impl<T: PartialOrd> PartialOrd for IOrd<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.0.partial_cmp(&self.0)
  }
} //}}}
// template source (darkkcyan)