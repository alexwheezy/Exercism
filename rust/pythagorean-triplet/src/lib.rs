use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    assert!(n > 0, "must be greater than zero");

    let mut result = HashSet::new();
    for a in 2..n {
        let b = n / 2 - a * n / (2 * (n - a));
        if a >= b {
            break;
        }

        let c = n - (a + b);
        if a * a + b * b == c * c {
            result.insert([a, b, c]);
        }
    }
    result
}
