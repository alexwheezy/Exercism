pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut result = n;
    let mut step = 0u64;

    while result != 1 {
        result = match result % 2 {
            0 => result / 2,
            _ => {
                let check = result
                    .checked_mul(3)
                    .and_then(|v| v.checked_add(1));
                match check {
                    None => {return None}
                    Some(value) => {value}
                }
            }
        };
        step += 1;
    }
    Some(step)
}