use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    const PUNCTUATION: [char; 11] = [':', '.', ',', '!', '^', '&', '@', '$', '%', ' ', '\n'];
    let mut result = HashMap::new();
    for word in words.split(&PUNCTUATION[..]) {
        if !word.is_empty() {
            *result
                .entry(word.trim_matches('\'').to_lowercase())
                .or_insert(0) += 1;
        }
    }
    result
}
