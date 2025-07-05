// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::char;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Empty,
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    if input.is_empty() {
        return Err(Error::Empty);
    }

    const COL_SIZE: usize = 3;
    const ROW_SIZE: usize = 4;

    let row_count = input.lines().count();
    if row_count < ROW_SIZE {
        return Err(Error::InvalidRowCount(row_count));
    }

    let total_line = row_count / ROW_SIZE;
    let mut number = String::new();
    let mut chars = vec![];

    for (line_index, input) in input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(ROW_SIZE)
        .enumerate()
    {
        for &line in input {
            for (col_index, chunk) in line
                .chars()
                .collect::<Vec<char>>()
                .chunks(COL_SIZE)
                .enumerate()
            {
                if chars.len() <= col_index {
                    chars.resize(col_index + 1, vec![]);
                }
                chars[col_index].extend(chunk.iter().cloned());
            }
        }

        for bars in chars.iter() {
            if bars.len() < COL_SIZE * ROW_SIZE {
                return Err(Error::InvalidColumnCount(bars.len()));
            }
            match bars.as_slice() {
                [' ', '_', ' ', '|', ' ', '|', '|', '_', '|', ' ', ' ', ' '] => number.push('0'),
                [' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '] => number.push('1'),
                [' ', '_', ' ', ' ', '_', '|', '|', '_', ' ', ' ', ' ', ' '] => number.push('2'),
                [' ', '_', ' ', ' ', '_', '|', ' ', '_', '|', ' ', ' ', ' '] => number.push('3'),
                [' ', ' ', ' ', '|', '_', '|', ' ', ' ', '|', ' ', ' ', ' '] => number.push('4'),
                [' ', '_', ' ', '|', '_', ' ', ' ', '_', '|', ' ', ' ', ' '] => number.push('5'),
                [' ', '_', ' ', '|', '_', ' ', '|', '_', '|', ' ', ' ', ' '] => number.push('6'),
                [' ', '_', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '] => number.push('7'),
                [' ', '_', ' ', '|', '_', '|', '|', '_', '|', ' ', ' ', ' '] => number.push('8'),
                [' ', '_', ' ', '|', '_', '|', ' ', '_', '|', ' ', ' ', ' '] => number.push('9'),
                _ => number.push('?'),
            }
        }
        chars.clear();

        if line_index < total_line - 1 {
            number.push(',');
        }
    }

    Ok(number)
}
