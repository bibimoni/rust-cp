// time-limit: 4000
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
  
  const LG: usize = 18;
  const K: usize = 100_000;
  let mut dp = vec![[Mint::zero(); LG]; K + 1];
  
  let mut non_one_divs: Vec<Vec<usize>> = vec![vec![]; K + 1];
  for div in 2..=K {
    for mul in (div * 2..=K).step_by(div) {
      non_one_divs[mul].push(div);
    }
  }
  
  for len in 1..LG {
    dp[1][len] = Mint::one();
  }
  
  for x in 2..=K {
    dp[x][1] = Mint::one();
    for len in 2..(LG - 1) {
      for div in non_one_divs[x].iter() {
        let prev = dp[x / *div][len - 1];
        dp[x][len] += prev;
      }
    }
  }
  
  let tt = scan.next::<usize>();
  for _ in 0..tt {
    let k = scan.next::<usize>();
    let n = scan.next::<usize>();

    fn binom_brute(n: usize, k: usize) -> Mint {
      if n < k {
        return Mint::zero();
      }
      if k > n - k {
        return binom_brute(n, n - k);
      }
      let mut result = Mint::one();
      for i in 0..k {
        result *= <modint::ModNum<998244353> as modint::NumTrait>::from((n - i) as i64) 
          / <modint::ModNum<998244353> as modint::NumTrait>::from((i + 1) as i64);
      }
      result
    }
    print!("{} ", Mint::new(n as i64));
    for x in 2..=k {
      let mut ans = Mint::new(0);
      for len in 1..(LG - 1) {
        ans += dp[x][len] * binom_brute(n + 1, len + 1);
      }
      print!("{} ", ans);
    }
    println!();
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
      ModNum { rep: x.rem_euclid(MOD) }
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
        if p % 2 == 1 { out *= x; }
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