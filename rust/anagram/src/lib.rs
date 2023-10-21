use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let sort_chars = |input: &str| {
        let mut chars = input.to_lowercase().chars().collect::<Vec<char>>();
        chars.sort_unstable();
        chars.iter().collect::<String>()
    };

    let mut result: HashSet<&str> = HashSet::new();
    for &anagram in possible_anagrams {
        if anagram.to_lowercase() != word.to_lowercase() {
            if sort_chars(word) == sort_chars(anagram) {
                result.insert(anagram);
            }
        }
    }
    result
}
