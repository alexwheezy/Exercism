pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len {
        0 => return vec!["".to_string(); digits.len() + 1],
        _ => digits
            .as_bytes()
            .windows(len)
            .map(|slice| std::str::from_utf8(slice).unwrap().to_string())
            .collect::<Vec<String>>(),
    }
}
