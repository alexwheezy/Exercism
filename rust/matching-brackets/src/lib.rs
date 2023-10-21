const BRACKETS: [char; 6] = ['(', ')', '[', ']', '{', '}'];

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: String = string.into();
    brackets.retain(|c| BRACKETS.contains(&c));

    loop {
        match () {
            _ if brackets.contains("()") => brackets = brackets.split("()").collect(),
            _ if brackets.contains("[]") => brackets = brackets.split("[]").collect(),
            _ if brackets.contains("{}") => brackets = brackets.split("{}").collect(),
            _ => break,
        }
    }
    brackets.is_empty()

}
