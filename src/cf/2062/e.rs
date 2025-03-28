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
    let n: usize = scan.next();
    let mut w = vec![0; n + 1];
    let mut vert: Vec<(usize, usize)> = vec![];
    for (idx, w) in w.iter_mut().enumerate().skip(1) {
      *w = scan.next();
      vert.push((*w, idx));
    }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..(n - 1) {
      let u: usize = scan.next();
      let v: usize = scan.next();
      adj[u].push(v);
      adj[v].push(u);
    }

    let lca = Lca::new(1, &adj);
    let tour = EulerTour::new(1, &adj);
    vert.sort_by(|a, b| {
      if b.0.cmp(&a.0) == Ordering::Equal {
        tour.start[b.1].cmp(&tour.start[a.1])
      } else {
        b.0.cmp(&a.0)
      }
    });

    let mut current = 0;
    let mut prev = vert[0].0;
    let mut equals: Vec<usize> = vec![vert[0].1];
    let mut ans = 0;
    for (weight, u) in vert.iter().copied().skip(1) {
      if prev != weight {
        loop {
          if current == 0 {
            current = equals.pop().unwrap();
          } else {
            current = lca.lca(current, equals.pop().unwrap());
          }
          if equals.is_empty() {
            break;
          }
        }
      }
      if current != 0 && !tour.is_ancestor_of(u, current) {
        ans = u;
        break;
      }
      equals.push(u);
      prev = weight;
    }

    writeln!(writer, "{}", ans).ok();
  }
}

use euler_tour::*;
pub mod euler_tour {
  #[derive(Clone, Copy, Debug, PartialEq, Eq)]
  pub enum InOut {
    In(usize),
    Out(usize),
  }

  pub struct EulerTour {
    pub tour: Vec<InOut>,
    pub start: Vec<usize>,
    pub end: Vec<usize>,
  }

  impl EulerTour {
    // one-indexed
    pub fn new(root: usize, adj: &Vec<Vec<usize>>) -> Self {
      let n: usize = adj.len() - 1;

      let tour = {
        fn dfs(tour: &mut Vec<InOut>, u: usize, p: usize, adj: &Vec<Vec<usize>>) {
          tour.push(InOut::In(u));

          for &v in &adj[u] {
            if p == v {
              continue;
            }
            dfs(tour, v, u, adj);
          }

          tour.push(InOut::Out(u));
        }
        let mut tour: Vec<InOut> = vec![];
        dfs(&mut tour, root, root, adj);
        tour
      };

      let (start, end) = {
        let mut start = vec![0; n + 1];
        let mut end = vec![0; n + 1];
        for (time, current) in tour.iter().copied().enumerate() {
          match current {
            InOut::In(u) => start[u] = time,
            InOut::Out(u) => end[u] = time,
          }
        }
        (start, end)
      };

      Self { tour, start, end }
    }

    // is u ancestor of v
    pub fn is_ancestor_of(&self, u: usize, v: usize) -> bool {
      self.start[u] <= self.start[v] && self.end[v] <= self.end[u]
    }
  }
}

use lca::*;
pub mod lca {
  use std::mem::swap;
  pub struct Lca {
    parent: Vec<Vec<usize>>,
    dep: Vec<usize>,
  }

  impl Lca {
    // one indexed
    pub fn new(root: usize, adj: &Vec<Vec<usize>>) -> Self {
      let n = adj.len() - 1;
      let lg: usize = (usize::BITS - n.leading_zeros()) as usize;
      let (dep, parent) = {
        fn dfs(
          dep: &mut Vec<usize>,
          parent: &mut Vec<Vec<usize>>,
          u: usize,
          p: usize,
          adj: &Vec<Vec<usize>>,
        ) {
          parent[u][0] = p;
          for ll in 1..parent[u].len() {
            parent[u][ll] = parent[parent[u][ll - 1]][ll - 1];
          }
          for &v in &adj[u] {
            if v == p {
              continue;
            }
            dep[v] = dep[u] + 1;
            dfs(dep, parent, v, u, adj);
          }
        }
        let mut parent = vec![vec![0; lg]; n + 1];
        let mut dep = vec![0; n + 1];
        dfs(&mut dep, &mut parent, root, root, adj);
        (dep, parent)
      };
      Self { parent, dep }
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
      let mut u = u;
      let mut v = v;
      if self.dep[u] < self.dep[v] {
        swap(&mut u, &mut v);
      }
      let k = self.dep[u] - self.dep[v];
      let lg = self.parent[0].len();
      for j in 0..lg {
        if (k >> j) & 1 == 1 {
          u = self.parent[u][j];
        }
      }
      if u == v {
        return u;
      }
      for j in (0..lg).rev() {
        if self.parent[u][j] != self.parent[v][j] {
          u = self.parent[u][j];
          v = self.parent[v][j];
        }
      }
      self.parent[u][0]
    }

    pub fn distance(&self, u: usize, v: usize) -> usize {
      self.dep[u] + self.dep[v] - 2 * self.dep[self.lca(u, v)]
    }

    pub fn is_path_on(&self, u: usize, v: usize, a: usize) -> bool {
      self.distance(u, v) == self.distance(u, a) + self.distance(a, v)
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
