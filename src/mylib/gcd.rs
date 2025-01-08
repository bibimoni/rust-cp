
#[allow(dead_code)]
fn gcd(a: u64, b: u64) -> u64 {
  if b == 0 {
    return a;
  } else {
    return gcd(a, a % b);
  }
}
