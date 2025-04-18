use std::fmt::{Display, Formatter, Result};

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        assert!(num > 0, "Number must be greater than zero");

        const M: [&str; 4] = ["", "M", "MM", "MMM"];
        const C: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        const X: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        const I: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];

        let numeral: usize = num as usize;
        let thousands = M[numeral / 1000];
        let hundreds = C[numeral % 1000 / 100];
        let tens = X[numeral % 100 / 10];
        let ones = I[numeral % 10];

        Self(format!("{thousands}{hundreds}{tens}{ones}"))
    }
}
