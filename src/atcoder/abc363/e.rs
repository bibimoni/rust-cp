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
  
  let h: usize = scan.next();
  let w: usize = scan.next();
  let y: usize = scan.next();
  let a: Vec<Vec<usize>> = 
    (0..h).map(|_| 
        (0..w).map(|_| scan.next()).collect()
    ).collect();
  
  let mut st: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
  
  for (i, item) in a.iter().enumerate() {
    st.insert((item[0], i, 0));
    st.insert((item[w - 1], i, w - 1));
  }
  for idx in [0, h - 1] {
    for (i, item) in a[idx].iter().skip(1).take(w - 2).enumerate() {
      st.insert((*item, idx, i + 1));
    }
  }

  let valid = |x: i32, y: i32| -> bool {
    x >= 0 && y >= 0 && x < h as i32 && y < w as i32
  };

  let dir: Vec<i32> = Vec::from([-1, 0, 1, 0, -1]);
  let mut ans: Vec<usize> = vec![0; y + 1];
  let mut cnt_sea: usize = 0;
  let mut was: Vec<Vec<bool>> = vec![vec![false; w]; h];
  
  for (sea_level, item) in ans.iter_mut().enumerate().take(y + 1).skip(1) {
    while !st.is_empty() {
      let (cell, ux, uy) = *st.first().unwrap();
      if cell > sea_level {
        break;
      }
      st.pop_first();
      if was[ux][uy] {
        continue;
      }
      was[ux][uy] = true;
      cnt_sea += 1;
      for i in 0..4 {
        let vx = ux as i32 + dir[i];
        let vy = uy as i32 + dir[i + 1];
        if !valid(vx, vy) {
          continue;
        }
        st.insert((a[vx as usize][vy as usize], vx as usize, vy as usize));
      }
    }
    *item = h * w - cnt_sea;
  }

  for land in ans.iter().skip(1) {
    writeln!(writer, "{land}").ok();
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