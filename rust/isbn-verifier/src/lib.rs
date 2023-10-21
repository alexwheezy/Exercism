/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    const ISBN_LEN: usize = 10;
    let isbn = isbn
        .chars()
        .filter(|&c| c.is_ascii_digit() || c == 'X')
        .collect::<String>();

    if isbn.len() != ISBN_LEN || isbn.contains('X') && !isbn.ends_with('X') {
        return false;
    }
    isbn.chars()
        .map(|c| c.to_digit(10).unwrap_or(10)) // SBN-10 may be 'X' (representing '10')
        .enumerate()
        .fold(0, |acc, (idx, x)| acc + ((ISBN_LEN - idx) as u32 * x))
        % 11
        == 0
}
