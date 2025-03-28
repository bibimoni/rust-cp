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
  let a: Vec<usize> = (0..n + 1)
    .map(|x| if x == 0 { 0 } else { scan.next() })
    .collect();
  let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
  for _ in 0..n - 1 {
    let u: usize = scan.next();
    let v: usize = scan.next();
    adj[u].push(v);
    adj[v].push(u);
  }

  struct Env {
    adj: Vec<Vec<usize>>,
    dp: Vec<Vec<usize>>,
    a: Vec<usize>,
    ans: Vec<usize>,
  }

  let mut dp: Vec<Vec<usize>> = vec![vec![usize::MAX]; n + 1];
  dp[0][0] = 0;
  let mut env = Env {
    adj,
    dp,
    a,
    ans: vec![0; n + 1],
  };

  fn dfs(env: &mut Env, u: usize, p: usize) {
    let ret = env
      .dp
      .binary_search_by(|x| match x.last().unwrap().cmp(&env.a[u]) {
        Ordering::Equal => Ordering::Greater,
        ord => ord,
      })
      .unwrap_err();

    let last_e = *env.dp[ret].last().unwrap();
    env.dp[ret].push(last_e.min(env.a[u]));
    env.ans[u] = env
      .dp
      .binary_search_by(|x| x.last().unwrap().cmp(&(usize::MAX - 1)))
      .unwrap_err()
      - 1; // max index with last element != usize::MAX

    let child = env.adj[u].to_vec();
    for v in child {
      if v == p {
        continue;
      }
      dfs(env, v, u);
    }
    env.dp[ret].pop();
  }
  dfs(&mut env, 1, 1);

  for v in env.ans.iter().skip(1) {
    writeln!(writer, "{}", v).ok();
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
