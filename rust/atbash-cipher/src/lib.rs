/// "Encipher" with the Atbash cipher.

const INV_ASCII: &str = "zyxwvutsrqponmlkjihgfedcba";

fn encrypt(plain: &str, ascii: &str, code: bool) -> String {
    let mut result = String::new();
    plain
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .enumerate()
        .for_each(|(idx, c)| {
            if code && idx % 5 == 0 && !result.is_empty() {
                result.push(' ');
            }
            match (c as usize).checked_sub(97) {
                Some(offset) => result.push(ascii.chars().nth(offset).unwrap()),
                None => result.push(c),
            }
        });
    result
}

pub fn encode(plain: &str) -> String {
    encrypt(plain, INV_ASCII, true)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    encrypt(cipher, INV_ASCII, false)
}
