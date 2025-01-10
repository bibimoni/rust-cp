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
  let mut a: Vec<usize> = vec![0; n + 1];
  for val in a.iter_mut().take(n + 1).skip(1) {
   *val = scan.next();
  }  

  let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
  let mut e: Vec<(usize, usize)> = vec![];
  for i in 1..=n {
    adj[i].push(a[i]);
    e.push((i, a[i]));
  }

  struct _Env {
    time_dfs: usize,
    scc_count: usize,
    st: Vec<usize>,
    num: Vec<Option<usize>>,
    low: Vec<usize>,
    was: Vec<bool>,
    group_id: Vec<usize>,
    g: Vec<Vec<usize>>,
  }
  let mut env = _Env {
    time_dfs: 0,
    scc_count: 0,
    st: vec![],
    num: vec![None; n + 1],
    low: vec![0; n + 1],
    was: vec![false; n + 1],
    group_id: vec![0; n + 1],
    g: adj
  };

  fn dfs (u: usize, env: &mut _Env) {
    env.time_dfs += 1;
    env.num[u] = Some(env.time_dfs);
    env.low[u] = env.time_dfs;
    env.st.push(u);

    for i in 0..env.g[u].len() {
      let v = env.g[u][i];
      if env.was[v] {
        continue;
      }
      if let Some(x) = env.num[v] {
        env.low[u] = min(env.low[u], x);
      } else {
        dfs(v, env);
        env.low[u] = min(env.low[u], env.low[v]);
      }
    }
    
    if env.low[u] == env.num[u].unwrap() {
      env.scc_count += 1;
      loop {
        let v = *env.st.last().unwrap();
        env.st.pop();
        env.was[v] = true;
        env.group_id[v] = env.scc_count;
        if u == v {
          break;
        }
      }
    }
  }

  for i in 1..=n {
    if env.num[i].is_none() {
      dfs(i, &mut env);   
    }
  }

  let mut scc_g : Vec<Vec<usize>> = vec![vec![]; n + 1];
  for (u, v) in e.into_iter() {
    if env.group_id[u] != env.group_id[v] {
      scc_g[env.group_id[u]].push(env.group_id[v]);
    }
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
        return self.next_opt().unwrap();
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
            return self.next_opt();
        } else {
            return None;
        }
    }
 
    fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();
        return stdin().read_line(&mut line).map(|_| line).ok();
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