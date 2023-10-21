pub fn primes_up_to(n: u64) -> Vec<u64> {
    if n == 0 || n == 1 {
        return vec![];
    }

    let mut primes: Vec<bool> = vec![false; (n as usize) + 1];
    let mut result = Vec::with_capacity(n as usize / 2);

    let mut p = 2;
    while p <= n {
        if !primes[p as usize] {
            result.push(p);
            let mut i = p * p;
            while i <= n {
                primes[i as usize] = true;
                i += p;
            }
        }
        p += 1;
    }
    result
}
