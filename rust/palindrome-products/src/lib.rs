/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut tmp = value;
        let mut cmp = 0;
        while tmp > cmp {
            let rem = tmp % 10;
            tmp /= 10;
            if rem == 0 && cmp == 0 {
                return None;
            }
            if tmp == cmp {
                return Some(Palindrome(value));
            }
            cmp = cmp * 10 + rem;
        }
        match tmp == cmp {
            true => Some(Palindrome(value)),
            false => None,
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    (min..=max)
        .flat_map(|lhs| (lhs..=max).map(move |rhs| lhs * rhs))
        .flat_map(Palindrome::new)
        .fold(None, |acc, p| match acc {
            Some((min, max)) if p < min => Some((p, max)),
            Some((min, max)) if p > max => Some((min, p)),
            None => Some((p, p)),
            total => total,
        })
}
