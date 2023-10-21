pub fn reply(message: &str) -> &str {
    let text = message.trim();
    if text.is_empty() {
        return "Fine. Be that way!";
    }
    let (capital, question) = (
        {
            let string: String = text.chars()
                .filter(|c| c.is_alphabetic()).collect();
            match string.is_empty() {
                false => {string.chars().all(char::is_uppercase)}
                true => {false}
            }
        },
        text.ends_with('?')
    );

    let res = match (capital, question) {
        (false, true) => {"Sure."},
        (true, false) => {"Whoa, chill out!"},
        (true, true) => {"Calm down, I know what I'm doing!"},
        (_, _) => {"Whatever."}
    };
    res
}
