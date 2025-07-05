use std::{error::Error, fmt::Display};

pub struct Luhn(String);

impl Luhn {
    /// Check a Luhn checksum.
    pub fn is_valid(&self) -> bool {
        matches!(Self::parse(&self.0), Ok(numbers) if {
            let sum = numbers
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
        })
    }

    fn is_valid_input(input: &str) -> bool {
        input.len() <= 1 || input.chars().any(|c| c.is_alphabetic())
    }

    fn parse(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
        if Self::is_valid_input(input) {
            return Err("Input contains letters".into());
        }
        Ok(input.chars().filter_map(|c| c.to_digit(10)).collect())
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self(input.to_string())
    }
}
