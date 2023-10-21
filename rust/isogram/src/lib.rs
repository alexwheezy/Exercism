pub fn check(candidate: &str) -> bool {
    let letters: Vec<char> = candidate
        .to_lowercase()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .collect();
    let mut tmp_letters = letters.clone();
    tmp_letters.sort();
    tmp_letters.dedup();
    letters.len() == tmp_letters.len()
}