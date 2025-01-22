// required modnum (Mint) to use
use binomial_coefficient::*;
pub mod binomial_coefficient {
  use crate::modint::NumTrait;
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

