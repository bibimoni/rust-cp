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
    let (l, r): (Vec<usize>, Vec<usize>) = (0..(n + 1))
      .map(|i| {
        if i == 0 {
          (0, 0)
        } else {
          (scan.next(), scan.next())
        }
      })
      .unzip();
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    for _ in 0..(n - 1) {
      let u: usize = scan.next();
      let v: usize = scan.next();
      adj[u].push(v);
      adj[v].push(u);
    }

    struct Env {
      adj: Vec<Vec<usize>>,
      l: Vec<usize>,
      r: Vec<usize>,
      a: Vec<usize>,
    }
    let a: Vec<usize> = vec![0; n + 1];
    let mut env: Env = Env { adj, l, r, a };
    let mut ans = 0;
    fn dfs(env: &mut Env, u: usize, p: usize) {
      env.a[u] = env.l[u];
      for &v in env.adj[p].clone().iter() {
        if p != u {
          dfs(env, v, u);
          env.a[u] = env.a[u].max(env.a[v]);
        }
      }
      env.a[u] = env.a[u].min(env.r[u]);
      for &v in env.adj[p].clone().iter() {
        if p != u {}
      }
    }
    dfs(&mut env, 1, 1);
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
