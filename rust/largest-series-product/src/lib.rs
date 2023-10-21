#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }

    let larger_sum = string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)).map(u64::from))
        .collect::<Result<Vec<_>, _>>()?
        .windows(span)
        .map(|chunk| chunk.iter().product::<u64>())
        .max();

    Ok(larger_sum.unwrap())
}
