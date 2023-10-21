pub fn factors(n: u64) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    
    let mut d = n;
    let mut i = 2;

    while d != 1 {
        if d % i == 0 {
            result.push(i);
            d /= i;
        } else {
            i += 1;
        }
    }
    result
}
