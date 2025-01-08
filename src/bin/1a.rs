#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
use std::io;
use std::cmp;
fn main() {
  let tests = read_i32();
  // let tests = 1;
  for _test in 0..tests {
    let mut p = read_vec_i32();
    let (n, m) = (p[0], p[1]);
    let (mut minE, mut sum) = (101, 0);
    for i in 0..n {
      p = read_vec_i32();
      for j in 0..m as usize {
        let x = p[j].abs();
        minE = cmp::min(minE, x);
        sum += x;
      }
    }
    println!("{}", sum - (n * m % 2) * 2 * minE);
  }
}

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