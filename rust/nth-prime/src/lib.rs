fn is_prime(n: u32) -> bool {
    (2..=(n as f64).sqrt() as u32).all(|number| n % number != 0)
}
pub fn nth(n: u32) -> u32 {
    (2..).filter(|n| is_prime(*n)).nth(n as usize).unwrap()
}