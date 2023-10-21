// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

macro_rules! planets {
    ($($t:ident),*) => {
        $(pub struct $t;)*
    };
}
macro_rules! impl_planets {
    ($(($t:ty,$n:expr)),+) => {
        $(impl Planet for $t {
            const PERIOD: f64 = $n;
        })*
    }
}

#[derive(Debug)]
pub struct Duration {
    value: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { value: s }
    }
}

pub trait Planet {
    const PERIOD: f64;
    const SECONDSPEREARTHYEAR: f64 = 60.0 * 60.0 * 24.0 * 365.25;
    fn years_during(d: &Duration) -> f64 {
        d.value as f64 / (Self::SECONDSPEREARTHYEAR * Self::PERIOD)
    }
}

planets!(Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune);
impl_planets!(
    (Mercury, 0.2408467),
    (Venus, 0.61519726),
    (Mars, 1.8808158),
    (Earth, 1.0),
    (Jupiter, 11.862615),
    (Saturn, 29.447498),
    (Uranus, 84.016846),
    (Neptune, 164.79132)
);
