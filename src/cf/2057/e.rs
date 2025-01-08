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
  let ntest : usize = scan.next();
  for _ in 0..ntest {
    let n : usize = scan.next();
    let m : usize = scan.next();
    let q : usize = scan.next();
    let mut adj : Vec<Vec<(usize, i32)>> = vec![vec![]; n + 1];
    let mut ws : Vec<i32> = vec![];
    for _ in 0..m {
      let u : usize = scan.next();
      let v : usize = scan.next();
      let w : i32 = scan.next();
      adj[u].push((v, w));
      adj[v].push((u, w));
      ws.push(w);
    }
    
    ws.sort();
    ws.dedup();
    
    let INF : i32 = 1e9 as i32;
    let mut dist : Vec<Vec<Vec<i32>>> = vec![vec![vec![0 as i32; n + 1]; n + 1]; m];
    
    for (idx, bound) in ws.iter().enumerate()  {
      let mut d : Vec<Vec<i32>> = vec![vec![INF; n + 1]; n + 1];
      for i in 1..=n {
        let mut deq : VecDeque<usize> = VecDeque::new();
        deq.push_back(i);
        d[i][i] = 0 as i32;
        while !deq.is_empty() {
          let u : usize = *deq.front().unwrap();
          deq.pop_front();
          for (v, w) in adj[u].iter() {
            let n_w = if *w > *bound { 1 } else { 0 };
            if d[i][u] + n_w < d[i][*v] {
              d[i][*v] = d[i][u] + n_w;
              if n_w == 1 {
                deq.push_back(*v);
              } else {
                deq.push_front(*v);
              }
            }
          }
        }
      }
      dist[idx] = d;
    } 

    for _ in 0..q {
      let a : usize = scan.next();
      let b : usize = scan.next();
      let k : i32 = scan.next();
      let mut lo : i32 = 0;
      let mut hi : i32 = (ws.len() - 1) as i32;
      let mut ans= -1;
      while lo <= hi {
        let mid = ((lo + hi) >> 1) as usize;
        if dist[mid][a][b] < k {
          ans = ws[mid];
          hi = mid as i32 - 1;
        } else {
          lo = mid as i32  + 1;
        }
      }
      print!("{} ", ans);
    }
    println!();
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
// template source: darkkcyan