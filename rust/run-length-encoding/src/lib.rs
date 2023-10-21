pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    let mut curr_count = 0;

    while let Some(curr) = chars.next() {
        curr_count += 1;
        if chars.peek() != Some(&curr) {
            if curr_count > 1 {
                result.push_str(&curr_count.to_string())
            }
            result.push(curr);
            curr_count = 0;
        }
    }
    result
}

pub fn decode(source: &str) -> String {
    let pred = |c: char| c.is_whitespace() || c.is_alphabetic();
    source
        .split(pred)
        .zip(source.matches(pred))
        .map(|(count, letter)| match count.parse::<usize>() {
            Ok(value) => letter.repeat(value),
            _ => letter.to_owned(),
        })
        .collect::<String>()
}
