pub mod test_bino_modint {
  
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
  
  use binomial_coefficient::*;
  pub mod binomial_coefficient {
    use super::modint::NumTrait;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct BinomialCoefficient<T: NumTrait + Clone + std::ops::Mul + std::ops::Div> {
      fact: Vec<T>,
      inv_fact: Vec<T>,
    }
  
    impl<T: NumTrait + Clone + std::ops::Mul<Output = T> + std::ops::Div<Output = T>> Default for BinomialCoefficient<T> {
      fn default() -> Self {
          Self::new()
      }
    }
  
    impl<T: NumTrait + Clone + std::ops::Mul<Output = T> + std::ops::Div<Output = T>> BinomialCoefficient<T>{
      pub fn new() -> BinomialCoefficient<T> {
        BinomialCoefficient {
          fact: vec![T::one(); 1],
          inv_fact: vec![T::one(); 1],
        }
      }
      
      pub fn C(&mut self, n: i64, mut k: i64) -> T {
        if k < 0 || k > n {
          return T::zero();
        }
        k = k.min(n - k);
        while self.fact.len() < (n + 1) as usize {
          self.fact.push((self.fact.last().unwrap().clone()) * T::from(self.fact.len() as i64));
          self.inv_fact.push(T::one() / self.fact.last().unwrap().clone());
        }
        
        self.fact[n as usize].clone() * self.inv_fact[k as usize].clone() * self.inv_fact[(n - k) as usize].clone()
      }
    }
  }
  
  #[cfg(test)]
  mod tests {
    use super::*;
      
    #[test]
    fn add_fn() {
      let a = Mint::new(1012312);
      let b = Mint::new(182379811231);
      assert_eq!(a + b, Mint::new(700351297));
      assert_eq!(a * b, Mint::new(340901838));
    }
  
    #[test]
    fn bino() {
      let mut coeff = BinomialCoefficient::<Mint>::new();
      assert_eq!(coeff.C(40, 32), Mint::new(76904685));
      assert_eq!(coeff.C(31, 32), Mint::new(0));
      assert_eq!(coeff.C(3, 2), Mint::new(3));
    }
  }
}
