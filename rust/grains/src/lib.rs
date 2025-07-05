pub fn square(s: u32) -> u64 {
    if let 0 | 65..=u32::MAX = s {
        panic!("Square must be between 1 and 64");
    }
    (1..=s).into_iter().skip(1).fold(1, |acc, _| (acc * 2))
}

pub fn total() -> u64 {
    (1..=64).into_iter().fold(1, |acc, x| acc.saturating_add(square(x)))
}
