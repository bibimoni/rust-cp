use proconio::input;
use cargo_snippet :: snippet;

#[snippet]
use std::io;
fn read_i32() -> i32 {
  let mut number: String = String::new();
  io::stdin()
    .read_line(&mut number)
    .expect("Failed to read int32 in a line");
  number.trim().parse::<i32>().expect("Not i32")
}
fn read_vec_i32() -> Vec<i32> {
  let mut numbers: String = String::new();
  io::stdin()
    .read_line(&mut numbers)
    .expect("Failed to read line");
  numbers
    .trim()
    .split_whitespace()
    .map(|i: &str| i.parse().unwrap())
    .collect()
}
#[snippet]
fn read_ints() -> Vec<usize> {
  let mut numbers: String = String::new();
  io::stdin()
    .read_line(&mut numbers)
    .expect("Failed to read line");
  numbers
    .trim()
    .split_whitespace()
    .map(|i: &str| i.parse().unwrap())
    .collect()
}
#[snippet]
fn read_str() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s.trim().to_string()
}
#[snippet]
fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 {
    return a;
  } else {
    return gcd(a, a % b);
  }
}
#[snippet]
fn bin_search(mut l: usize, mut r: usize, mut f: impl FnMut(usize) -> bool) -> (usize, usize) {
  while r > l + 1 {
    let mid = (r + l) / 2;
    if f(mid) {
      r = mid;
    } else {
      l = mid;
    }
  }
  (l, r)
}
fn main() {
  input! {
      n: u8,
      m: u32,
      l: i32,
  }
  
  // now you can use n, m and l as variable.
  println!("{} {} {}", n, m, l);
}