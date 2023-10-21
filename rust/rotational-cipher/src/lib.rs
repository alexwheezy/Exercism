pub fn rotate(input: &str, key: i8) -> String {
    const UPPER: i8 = 65; // Upper case alphabet
    const LOWER: i8 = 97; // Lower case alphabet
    const ASCIIL: i8 = 26; // The length of the alphabet is 26 characters

    input
        .chars()
        .map(|c| match c.is_alphabetic() {
            true => {
                let offset = |n: i8| (c as i8 - n + key).rem_euclid(ASCIIL) + n;
                match c.is_uppercase() {
                    true => offset(UPPER) as u8 as char,
                    false => offset(LOWER) as u8 as char,
                }
            }
            false => c,
        })
        .collect()
}
