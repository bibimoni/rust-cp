use std::io;

fn main() {
  let tests = read_i32();
  for _test in 0..tests {
    let MAX_LEN = 25;
    let p = read_vec_i32();
    let (n, k) = (p[0], p[1]);
    let (mut curr, mut sum) = (1, 1);
    let mut ans = Vec::new();
    while(sum < k) {
      ans.push(curr);
      curr *= 2;
      if(sum + curr >= k) {
        ans.push(k - 1 - sum);
        break;
      }
      sum += curr;
    }
    while(curr <= k) {
      curr *= 2;
    }
    ans.push(k + 1);
    ans.push(k + curr);
    while(ans.len() < MAX_LEN - 2 && curr <= n) {
      ans.push(curr);
      curr *= 2;
    }
    println!("{}", ans.len());
    for i in 0..ans.len() {
      print!("{} ", ans[i]);
    }
    println!();
  }
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