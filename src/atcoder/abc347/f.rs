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
  let mut a: Vec<Vec<i64>> = vec![vec![0i64; n + 1]; n + 1];
  
  for i in 1..=n {
    for j in 1..=n {
      a[i][j] = scan.next();
    }
  }
  
  let build_prefix_sum = |arr: &Vec<Vec<i64>>| -> Vec<Vec<i64>> {
    let mut ret: Vec<Vec<i64>> = vec![vec![0 as i64; n + 1]; n + 1];
    for i in 1..=n {
      for j in 1..=n {
        ret[i][j] = ret[i - 1][j] + ret[i][j - 1] - ret[i - 1][j - 1] + arr[i][j];
      }
    }
    ret
  };
  
  let make_rect_mm = |arr: &Vec<Vec<i64>>| -> Vec<Vec<i64>> {
    let mut ret: Vec<Vec<i64>> = vec![vec![-1; n + 1]; n + 1];
    for i in m..=n {
      for j in m..=n {
        ret[i][j] = arr[i][j] - arr[i - m][j] - arr[i][j - m] + arr[i - m][j - m];
      }
    }
    ret
  };
  
  
  const LG: usize = 18;
  let build_sparse = |arr: &Vec<Vec<i64>>| -> Vec<Vec<Vec<Vec<i64>>>> {
    let mut mat: Vec<Vec<Vec<Vec<i64>>>> = vec![vec![vec![vec![-1; n + 1]; LG]; n + 1]; LG];
    for lg_1 in 0..LG {
      for i in 1.. {
        if i + (1 << lg_1) - 1 > n {
          break;
        }
        for lg_2 in 0..LG {
          for j in 1.. {
            if j + (1 << lg_2) - 1 > n {
              break;
            }
            mat[lg_1][i][lg_2][j] = match lg_1 {
              0 => match lg_2 {
                0 => arr[i][j],
                _ => max(mat[lg_1][i][lg_2 - 1][j], mat[lg_1][i][lg_2 - 1][j + (1 << (lg_2 - 1))])
              },
              _ => max(mat[lg_1 - 1][i][lg_2][j], mat[lg_1 - 1][i + (1 << (lg_1 - 1))][lg_2][j])
            }
          }
        }
      }
    }
    mat
  };
  
  let query_sparse = |mat: &Vec<Vec<Vec<Vec<i64>>>>, x: usize, y: usize, u: usize, v: usize| -> i64 {
    let xu = (u - x + 1).ilog2() as usize;
    let yv = (v - y + 1).ilog2() as usize;
    mat[xu][x][yv][y]
      .max(mat[xu][x][yv][v - (1 << yv) + 1])
      .max(mat[xu][u - (1 << xu) + 1][yv][y])
      .max(mat[xu][u - (1 << xu) + 1][yv][v - (1 << yv) + 1])
  };
  
  let rotate_right = |arr: &Vec<Vec<i64>>| -> Vec<Vec<i64>> {
    let mut ret: Vec<Vec<i64>> = vec![vec![-1; n + 1]; n + 1];
    for i in 1..=n {
      for j in 1..=n {
       ret[j][n - i + 1] = arr[i][j];
      }
    }
    ret
  };
  
  let find_ans = |arr: &Vec<Vec<i64>>| -> i64 {
    let mut ans = -1;
    let st = build_sparse(&make_rect_mm(&build_prefix_sum(&arr)));
    
    for vert in m..=(n - m) {
      for hor in m..=(n - m) {
        let pos_1_f = (m, m);
        let pos_1_s = (n, vert);
        
        assert!(pos_1_f.0 <= pos_1_s.0);
        assert!(pos_1_f.1 <= pos_1_s.1);
        
        let pos_2_f = (m, vert + m);
        let pos_2_s = (hor, n);
        
        assert!(pos_2_f.1 <= pos_2_s.1);
        assert!(pos_2_f.0 <= pos_2_s.0);
        
        let pos_3_f = (hor + m, vert + m);
        let pos_3_s = (n, n);
        
        assert!(pos_3_f.0 <= pos_3_s.0);
        assert!(pos_3_f.1 <= pos_3_s.1);
        
        ans = ans
          .max(query_sparse(&st, pos_1_f.0, pos_1_f.1, pos_1_s.0, pos_1_s.1)
           + query_sparse(&st, pos_2_f.0, pos_2_f.1, pos_2_s.0, pos_2_s.1)
           + query_sparse(&st, pos_3_f.0, pos_3_f.1, pos_3_s.0, pos_3_s.1));
      }
    }
    
    for vert_1 in m..=n {
      for vert_2 in vert_1 + m..=(n - m) {
        let pos_1_f = (m, m);
        let pos_1_s = (n, vert_1);
        
        assert!(pos_1_f.0 <= pos_1_s.0);
        assert!(pos_1_f.1 <= pos_1_s.1);
        
        let pos_2_f = (m, vert_1 + m);
        let pos_2_s = (n, vert_2);
        
        assert!(pos_2_f.1 <= pos_2_s.1);
        assert!(pos_2_f.0 <= pos_2_s.0);
        
        let pos_3_f = (m, vert_2 + m);
        let pos_3_s = (n, n);
        
        assert!(pos_3_f.0 <= pos_3_s.0);
        assert!(pos_3_f.1 <= pos_3_s.1);

        ans = ans
          .max(query_sparse(&st, pos_1_f.0, pos_1_f.1, pos_1_s.0, pos_1_s.1)
           + query_sparse(&st, pos_2_f.0, pos_2_f.1, pos_2_s.0, pos_2_s.1)
           + query_sparse(&st, pos_3_f.0, pos_3_f.1, pos_3_s.0, pos_3_s.1));
      }
    }
    ans
  };
  let mut ans = 0;
  for _ in 0..4 {
    ans = ans.max(find_ans(&a));
    a = rotate_right(&a);
  }
  writeln!(writer, "{}", ans).ok();
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