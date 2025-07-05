use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> Luhn for T {
    /// Check a Luhn checksum.
    fn valid_luhn(&self) -> bool {
        let input = self.to_string();
        if input.len() <= 1 || input.chars().any(|c| c.is_alphabetic()) {
            return false;
        }
        let parse: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
        let sum = parse
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, &value)| {
                let mut number = value;
                if idx % 2 != 0 {
                    number *= 2;
                    if number > 9 {
                        number -= 9;
                    }
                }
                number
            })
            .sum::<u32>();
        sum % 10 == 0
    }
}
