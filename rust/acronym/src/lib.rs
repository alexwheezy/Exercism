fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    for word in phrase.trim().split(&[' ', ':', '_', '-']) {
        let letters = uppercase_first_letter(word);
        if !letters.chars().all(char::is_uppercase) {
            result.push_str(&letters.matches(char::is_uppercase).collect::<String>());
        } else {
            if !word.is_empty() {
                result.push(word.chars().nth(0).unwrap());
            }
        }
    }
    result
}
