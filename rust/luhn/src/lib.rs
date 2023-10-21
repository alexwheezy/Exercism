fn string_to_ints(code: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut numbers = vec![];
    for c in code.chars() {
        if c.is_whitespace() {
            continue;
        }
        let value = c.to_string().parse::<u32>()?;
        numbers.push(value);
    }
    Ok(numbers)
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let result;

    match string_to_ints(code) {
        Ok(numbers) => {
            if numbers.len() <= 1 {
                return false;
            }
            result = numbers
                .iter().rev().enumerate()
                .map(|(idx, &value)| {
                    let mut number = value;
                    if idx % 2 != 0 {
                        number *= 2;
                        if number > 9 {
                            number -= 9;
                        }
                    }
                    number
                }).sum::<u32>();
        }
        Err(_) => {
            return false;
        }
    }
    result % 10 == 0
}
