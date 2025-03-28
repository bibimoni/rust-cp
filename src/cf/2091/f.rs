#![allow(unexpected_cfgs, unused_macros, unused_imports)]
#![allow(clippy::needless_range_loop)]
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
  
  let tt: usize = scan.next();
  for _ in 0..tt {
    let n: usize = scan.next();
    let m: usize = scan.next();
    let d: i32 = scan.next();
    let mut a: Vec<Vec<char>> = (0..n).map(|_| scan.next::<String>().chars().collect()).collect();
    
    a.reverse(); 
    let mut holds: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, p) in a.into_iter().enumerate() {
      for (j, &c) in p.iter().enumerate() {
        if c == 'X' {
          holds[i].push(j)
        }
      }
    }

    let mut lo: i32 = 0;
    let mut hi: i32 = m as i32 - 1;
    let mut d_1: i32 = 0;
    while lo <= hi {
      let mid = (lo + hi) >> 1;
      if d * d >= mid * mid + 1 {
        lo = mid + 1;
        d_1 = mid;
      } else {
        hi = mid - 1;
      }
    }

    fn make_pref(arr: &[Mint]) -> Vec<Mint> {
      let mut pref = vec![Mint::new(0); arr.len()];
      for i in 0..arr.len() {
          pref[i] = arr[i];
        if i > 0 {
          let local = pref[i - 1];
          pref[i] += local; 
        }
      }
      pref
    } 
    let range = |pref: &[Mint], l: i32, r: i32| -> Mint {
      let u: usize = l.max(0) as usize;
      let v: usize = r.min(m as i32 - 1) as usize;
      if u == 0 { pref[v] } else { pref[v] - pref[u - 1] }
    };

    let mut dp: (Vec<Mint>, Vec<Mint>) = (vec![Mint::new(0); m], vec![Mint::new(0); m]); 
    for &j in holds[0].iter() {
      dp.0[j] = Mint::new(1);
    }
    let pref = make_pref(&dp.0);
    for &j in holds[0].iter() {
      dp.1[j] = range(&pref, j as i32 - d, j as i32 + d);
      dp.1[j] -= dp.0[j];
    }
    for i in 1..n {
      let mut ndp: (Vec<Mint>, Vec<Mint>) = (vec![Mint::new(0); m], vec![Mint::new(0); m]); 
      let pref_down_1 = make_pref(&dp.1);
      let pref_down_0 = make_pref(&dp.0);
      for &j in holds[i].iter() {
        ndp.0[j] += range(&pref_down_0, j as i32 - d_1, j as i32 + d_1);
        ndp.0[j] += range(&pref_down_1, j as i32 - d_1, j as i32 + d_1);
      }
      let pref = make_pref(&ndp.0);
      for &j in holds[i].iter() {
        ndp.1[j] = range(&pref, j as i32 - d, j as i32 + d);
        ndp.1[j] -= ndp.0[j];
      }
      dp = ndp;
    }
    let mut ans = Mint::new(0);
    for j in 0..m {
      ans += dp.0[j] + dp.1[j];
    }
    println!("{ans}");
  }
}

use modint::*;
#[allow(clippy::module_inception)]
pub mod modint {

  #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
  pub struct ModNum<const MOD: i64> {
    rep: i64,
  }
  impl<const MOD: i64> ModNum<MOD> {
    pub fn new(x: i64) -> ModNum<MOD> {
      ModNum {
        rep: x.rem_euclid(MOD),
      }
    }
    pub fn rep(self) -> i64 {
      self.rep
    }
    pub fn inv(self) -> Self {
      self.power((MOD - 2) as usize)
    }
    pub fn power(self, b: usize) -> ModNum<MOD> {
      let mut out = ModNum::new(1);
      let mut x = ModNum::new(self.rep);
      let mut p = b;
      while p > 0 {
        if p % 2 == 1 {
          out *= x;
        }
        x *= x;
        p /= 2;
      }
      out
    }
  }

  pub trait NumTrait {
    fn zero() -> Self;
    fn one() -> Self;
    fn from(value: i64) -> Self;
  }

  impl<const MOD: i64> NumTrait for ModNum<MOD> {
    fn zero() -> Self {
      ModNum::new(0)
    }
    fn one() -> Self {
      ModNum::new(1)
    }
    fn from(value: i64) -> Self {
      ModNum::new(value)
    }
  }

  impl<const MOD: i64> std::ops::Neg for ModNum<MOD> {
    type Output = Self;
    fn neg(self) -> Self::Output {
      ModNum::new(-self.rep)
    }
  }
  impl<const MOD: i64> std::ops::AddAssign for ModNum<MOD> {
    fn add_assign(&mut self, rhs: Self) {
      self.rep = (self.rep + rhs.rep).rem_euclid(MOD);
    }
  }
  impl<const MOD: i64> std::ops::SubAssign for ModNum<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
      self.rep = (self.rep - rhs.rep).rem_euclid(MOD);
    }
  }
  impl<const MOD: i64> std::ops::MulAssign for ModNum<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
      self.rep = (self.rep * rhs.rep).rem_euclid(MOD);
    }
  }
  #[allow(clippy::suspicious_op_assign_impl)]
  impl<const MOD: i64> std::ops::DivAssign for ModNum<MOD> {
    fn div_assign(&mut self, rhs: Self) {
      *self *= rhs.inv()
    }
  }
  macro_rules ! bi_ops_impl {
    ($std_ops: ident , $fn: ident , $ op: tt ) => {
      impl<const MOD: i64> std::ops::$std_ops for ModNum<MOD> {
        type Output = Self;
        fn $fn (self, rhs: Self ) -> Self::Output {
          let mut out = ModNum::new(self.rep);
          out $op rhs;
          out
        }
      }
    };
  }
  bi_ops_impl ! (Add, add, += );
  bi_ops_impl ! (Sub, sub, -= );
  bi_ops_impl ! (Mul, mul, *= );
  bi_ops_impl ! (Div, div, /= );

  impl<const MOD: i64> std::fmt::Display for ModNum<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
      write!(f, "{}", self.rep)
    }
  }
}

pub fn power<const MOD: i64>(a: ModNum<MOD>, b: usize) -> ModNum<MOD> {
  let out = a.power(b);
  out
}
const MOD: i64 = 998_244_353;
pub type Mint = ModNum<MOD>;
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
