#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    EmptyValue,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    assert!(!values.is_empty(), "values must be not empty");

    let mut res: Vec<Vec<u8>> = vec![vec![]; values.len()];
    for (idx, &item) in values.iter().enumerate() {
        let mut value = item;
        let mut shift = 0;
        if value == 0 {
            res[idx].push(value as u8);
        } else {
            while value != 0 {
                res[idx].push(((value % 128) | shift << 7) as u8);
                value /= 128;
                shift = 1;
            }
            res[idx].reverse();
        }
    }
    res.into_iter().flatten().collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes.is_empty() {
        return Err(Error::EmptyValue);
    }

    if bytes.len() == 1 && bytes[0] & 1 << 7 != 0 {
        return Err(Error::IncompleteNumber);
    }

    let res = bytes
        .split_inclusive(|&byte| byte & 1 << 7 == 0)
        .into_iter()
        .map(|item| {
            item.iter().rev().enumerate().fold(0, |acc, (idx, value)| {
                acc + (*value as u32 & 0x7f) * 128_u32.pow(idx as u32)
            })
        })
        .collect::<Vec<u32>>();

    Ok(res)
}
