const UPPER_A: u8 = 65;

fn write_chars(offset: u8, height: u8) -> String {
    let next_char = (offset + UPPER_A) as char;
    let side_space = (height - offset) as usize;
    let center_space = ((offset - 1) * 2 + 1) as usize;
    format!(
        "{:^side_space$}{next_char}{:^center_space$}{next_char}{:^side_space$}",
        "", "", "",
    )
}

pub fn get_diamond(c: char) -> Vec<String> {
    let mut distance = c as u8 - UPPER_A;
    let half = distance;
    let height = (distance * 2) + 1;
    let mut lines = Vec::with_capacity(height as usize);

    for index in 0..height {
        let line = if index == 0 || index == height - 1 {
            format!("{:^space$}", UPPER_A as char, space = height as usize)
        } else if index <= distance {
            write_chars(index, half)
        } else {
            distance -= 1;
            write_chars(distance, half)
        };
        lines.push(line);
    }
    lines
}
