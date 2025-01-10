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
  let a: Vec<usize> = (0..n).map(|_| scan.next()).collect();

  let mut s: Vec<usize> = vec![0; n];
  for (i, val) in a.iter().enumerate() {
    s[i] = if i == 0 { *val } else { s[i - 1] + *val };
    s[i] %= m;
  }

  let mut ft: Vec<usize> = vec![0; m + 2];
  let mut cnt: Vec<usize> = vec![0; m + 2];

  fn update(i: usize, val: usize, vec: &mut [usize]) {
    let mut idx = i + 1;
    while idx < vec.len() {
      vec[idx] += val;
      idx += idx & (usize::MAX - idx + 1);
    }
  }

  fn query(i: usize, vec: &[usize]) -> usize {
    let mut idx = i + 1;
    let mut ret = 0;
    while idx > 0 {
      ret += vec[idx];
      idx -= idx & (usize::MAX - idx + 1)
    }
    ret
  }

  fn range(l: usize, r: usize, vec: &[usize]) -> usize {
    query(r, vec) - query(l - 1, vec)
  }

  update(0, 1, &mut cnt);
  update(0, 0, &mut ft);

  let mut ans = 0;
  for sum in s.iter() {
    let cm = range(*sum + 1, m, &cnt);
    let sm = range(*sum + 1, m, &ft);
    ans += cm * (sum + m) - sm;
    
    let c0 = query(*sum, &cnt);
    let s0 = query(*sum, &ft);
    ans += c0 * sum - s0;

    update(*sum, 1, &mut cnt);
    update(*sum, *sum, &mut ft);
  }

  writeln!(writer, "{ans}").ok();
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