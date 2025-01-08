#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]
use std::io;
use std::collections::VecDeque;
fn main() {
  let tests = read_i32();
  // let tests = 1;
  for _test in 0..tests {
    let mut s = read_str();
    let n = s.len();
    let mut a : VecDeque<i32> = s.chars().map(|c| if c == '1' {1} else if c == '0' {0} else {2} ).collect();
    a.push_front(0);
    let mut cnt1 = vec![0; n + 1];
    let mut cnt0 = vec![0; n + 1];
    for i in 1..=n {
      cnt1[i] += cnt1[i - 1] + (a[i] == 1) as i32;
      cnt0[i] += cnt0[i - 1] + (a[i] == 0) as i32;
    }
    let mut ans = 0;
    for i in 1..=n {
      if a[i] == 0 && cnt0[i - 1] > 0 {
        continue;
      }
      if a[i] == 1 && cnt1[n] - cnt1[i] > 0 {
        continue;
      }
      if a[i] == 2 && (cnt0[i - 1] > 0 || cnt1[n] - cnt1[i] > 0){
        continue;
      }
      ans += 1;
    }
    println!("{}", ans);
  }
}

fn read_i32() -> i32 {
  let mut number: String = String::new();
  io::stdin()
    .read_line(&mut number)
    .expect("Failed to read int32 in a line");
  number.trim().parse::<i32>().expect("Not i32")
}

fn read_str() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s.trim().to_string()
}