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

fn read_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

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
  let tests = read_i32();
  // let tests = 1;
  for _test in 0..tests {
    let p = read_ints();
      let (n, k) = (p[0], p[1]);
      let s = read_str();
      let mut f = |lcp| s[lcp..].matches(&s[0..lcp]).count();
      let ans = bin_search(0, s.len() + 1, |lcp| s[lcp..].matches(&s[0..lcp]).count() + 1 < k).0;
      println!("{}", ans);
  }
}
