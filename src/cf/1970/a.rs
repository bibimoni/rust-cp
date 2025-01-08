#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
use std::io;
fn main() {
  // let tests = read_i32();
  let tests = 1;
  for _test in 0..tests {
    let mut s : Vec<char> = read_str().chars().collect();
    let n = s.len();
    s.insert(0, '#');
    let mut pre : Vec<(i32, usize, char)> = Vec::with_capacity(n + 1);
    pre.resize(n + 1, (0, 0, '#'));
    // println!("{:?} {}", s);
    let mut cnt = 0;
    for i in 1..n + 1 as usize {
      pre[i] = (cnt, i, s[i]);
      if s[i] == '(' {
        cnt += 1;
      }
      else {
        cnt -= 1;
      }
    }
    pre.remove(0);
    pre.sort_by(|a, b| {
      if a.0 != b.0 {
        a.0.cmp(&b.0)
      }
      else {
        b.1.cmp(&a.1)
      }
    });
    for i in 0..n as usize {
      print!("{}", pre[i].2);
    }
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