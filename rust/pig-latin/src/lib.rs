fn convert(input: &str) -> String {
    if input.is_empty() {
        return String::default();
    }
    const VOWELS: &str = "aeiou";
    let trim_and_ay = |offset: usize| format!("{}{}ay", &input[offset..], &input[0..offset]);
    let mut letter = input.chars().peekable();
    match letter.peek().unwrap() {
        chr if VOWELS.contains(*chr) => trim_and_ay(0),
        'y' | 'x' => {
            letter.next();
            match letter.peek().unwrap() {
                't' | 'r' => trim_and_ay(0),
                _ => trim_and_ay(1),
            }
        }
        _ => {
            if let Some(chr) = input.find(|chr| chr == 'q') {
                if letter.nth(chr + 1).unwrap() == 'u' {
                    return trim_and_ay(chr + 2);
                }
            }
            let cluster = input
                .find(|chr| VOWELS.contains(chr) || chr == 'y')
                .unwrap_or_default();
            trim_and_ay(cluster)
        }
    }
}
pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(convert)
        .collect::<Vec<String>>()
        .join(" ")
}
