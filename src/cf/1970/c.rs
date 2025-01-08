#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
use std::io;
fn main() {
  // let tests = read_i32();
  let tests = 1;
  for _test in 0..tests {
    let mut p = read_ints();
    let n = p[0];
    let mut a = Vec::with_capacity(n + 1 as usize);
    a.resize(n + 1 as usize);
    for i in 0..n - 1 as usize {
      p = read_ints();
      let (mut u, mut v) = (p[0], p[1]);
      a[u] = v;
      a[v] = u;
    }
    let mut win = false;
    let mut was = vec![0; n + 1];
    let init = read_i32();
    let mut u = init;
    let mut cnt = 0;
    while(!was[a[u]]) {
      cnt += 1;
      u = a[u];
      was[u] = 1;
    }
    win |= (cnt & 1) as usize;
  }
}

fn read_i32() -> i32 {
  let mut number: String = String::new();
  io::stdin()
    .read_line(&mut number)
    .expect("Failed to read int32 in a line");
  number.trim().parse::<i32>().expect("Not i32")
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