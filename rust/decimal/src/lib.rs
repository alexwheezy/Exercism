use num_bigint::BigInt;
/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal {
    value: BigInt,
    decimal_places: u32,
}
impl Decimal {
    pub fn new(mut value: BigInt, mut decimal_places: u32) -> Self {
        // Normalize by stripping trailing zeros
        // This is potentially very ineffificient, but will do for now.
        while (value.clone() % 10) == BigInt::from(0) && decimal_places > 0 {
            value /= 10;
            decimal_places -= 1
        }
        Decimal {
            value,
            decimal_places,
        }
    }
    pub fn try_from(input: &str) -> Option<Decimal> {
        let (int_part, decimal_part) = input
            .trim_start_matches('-')
            .split_once('.')
            .unwrap_or((input, "0"));
        let decimal_places = decimal_part.len() as u32;
        let mut value = int_part.trim_start_matches('-').parse::<BigInt>().ok()?
            * BigInt::from(10).pow(decimal_places)
            + decimal_part.parse::<BigInt>().ok()?;
        if input.starts_with('-') {
            value = -value;
        }
        Some(Self::new(value, decimal_places))
    }
}
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = self.value.clone() * BigInt::from(10).pow(other.decimal_places);
        let b = other.value.clone() * BigInt::from(10).pow(self.decimal_places);
        a.partial_cmp(&b)
    }
}
impl std::ops::Add for Decimal {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let a = self.value * BigInt::from(10).pow(other.decimal_places);
        let b = other.value * BigInt::from(10).pow(self.decimal_places);
        let total_decimal_places = self.decimal_places + other.decimal_places;
        Decimal::new(a + b, total_decimal_places)
    }
}
impl std::ops::Sub for Decimal {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}
impl std::ops::Mul for Decimal {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Decimal::new(
            self.value * other.value,
            self.decimal_places + other.decimal_places,
        )
    }
}
impl std::ops::Neg for Decimal {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Decimal {
            value: -self.value,
            decimal_places: self.decimal_places,
        }
    }
}
