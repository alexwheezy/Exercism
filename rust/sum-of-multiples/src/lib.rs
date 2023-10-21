pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit).into_iter()
        .filter(|x| {
            factors.iter().any(|f| match x.checked_rem(*f) {
                Some(value) => value == 0,
                None => false,
            })
        }).sum()
}