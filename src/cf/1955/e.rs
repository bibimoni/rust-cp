use std::io;
fn main() {
  let tests = read_i32();
  // let tests = 1;
  for _test in 0..tests {
    let n = read_i32() as usize;
    let a : Vec<i32> = read_str().chars().map(|c| {if c == '1' {1} else {0} }).collect();
    let mut ans = 1;
    for len in (1..n + 1).rev() {
      let mut b = vec![0; n + 1];
      let mut ok = true;
      let mut v = 0;
      for i in 0..a.len() {
        v ^= b[i];
        if v == a[i] {
          if i + len > n {
            ok = false;
            break;
          }
          v ^= 1;
          b[i + len] ^= 1;
        }
      }
      if ok {
        ans = len;
        break;
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