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

  let flip = |c: char| -> char {
    if c.is_ascii_lowercase() {
      c.to_ascii_uppercase()
    } else {
      c.to_ascii_lowercase()
    }
  };

  let mut s: Vec<char> = scan.next::<String>().chars().collect();
  let n = s.len();
  let mut st: Vec<i32> = vec![];
  let mut nxt: Vec<i32> = vec![n as i32; n];
  let mut prev: Vec<i32> = vec![n as i32; n];
  for (i, c) in s.iter_mut().enumerate() {
    if *c == '(' {
      st.push(i as i32);
    } else if *c == ')' {
      let open_idx = st.pop().unwrap();
      nxt[open_idx as usize] = i as i32;
      prev[i] = open_idx;
    } else if st.len() % 2 == 1 {
      *c = flip(*c);
    }
  }

  struct Env {
    ans: Vec<char>,
    s: Vec<char>,
    nxt: Vec<i32>,
    prev: Vec<i32>,
  }
  let mut env = Env {
    ans: vec![],
    s,
    nxt,
    prev,
  };
  fn recursive(env: &mut Env, l: i32, r: i32, flag: bool) {
    if l > r {
      return;
    }
    if !flag {
      let mut i = l;
      while i <= r {
        if env.s[i as usize] == '(' {
          recursive(env, i + 1, env.nxt[i as usize] - 1, !flag);
          i = env.nxt[i as usize];
        } else {
          env.ans.push(env.s[i as usize]);
        }
        i += 1;
      }
    } else {
      let mut i = r;
      while i >= l {
        if env.s[i as usize] == ')' {
          recursive(env, env.prev[i as usize] + 1, i - 1, !flag);
          i = env.prev[i as usize];
        } else {
          env.ans.push(env.s[i as usize]);
        }
        i -= 1;
      }
    }
  }
  recursive(&mut env, 0, n as i32 - 1i32, false);
  writeln!(writer, "{}", env.ans.into_iter().collect::<String>()).ok();
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
