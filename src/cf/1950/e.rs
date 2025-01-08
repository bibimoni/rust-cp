#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
use std::io;
use std::cmp;
fn main() {
  let tests = read_i32();
  for _test in 0..tests {
    let n = read_i32();
    let s = read_str();
    let mut lens = Vec::new();
    let mut i = 1;
    while i * i <= n {
      if n % i == 0 {
        lens.push(i);
        if n != i * i {
          lens.push(n / i);
        }
      }
      i += 1;
    }
    let mut ans = n;
    for cnt in lens {
      let sz = n / cnt;
      let mut ok = true;
      let mut diffs = 0;
      for i in 0..sz {
        let mut mark = vec![0; 26];
        for j in 0..cnt {
          mark[(s.as_bytes()[(i + j * sz) as usize] as usize) - ('a') as usize] += 1;
        }
        mark.sort_by(|a, b| a.cmp(&b));
        mark.reverse();
        if mark[2] != 0 || mark[1] > 1 {
          ok = false;
        }
        if mark[1] == 1 {
          diffs += 1;
        }
      }
      if ok && diffs <= 1 {
        ans = cmp::min(ans, sz);
      }
    }
    println!("{}", ans);
  }
}

fn read_str() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s.trim().to_string()
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