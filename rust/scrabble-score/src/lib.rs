use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn value_from_letter(letter: char) -> u8 {
    match letter.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => 0,
    }
}

pub fn score(word: &str) -> u64 {
    let mut letters: HashMap<char, u8> = HashMap::new();
    word.chars().for_each(|c| {
        *letters.entry(c).or_insert(0) += 1;
    });

    letters.iter().fold(0, |sum, (key, value)| {
        sum + (value_from_letter(*key) * value) as u64
    })
}