/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const ALPHABET_LENGTH: i32 = 26;
const LOWER_A: i32 = 97;

fn gcd(a: i32, b: i32) -> i32 {
    match (a, b) {
        (0, _) => b,
        (_, 0) => a,
        (a, b) if a == b => a,
        (a, b) if a > b => gcd(a - b, b),
        _ => gcd(a, b - a),
    }
}

fn mod_mul_inv(a: i32) -> i32 {
    let mut x = 1;
    loop {
        if (a * x) % ALPHABET_LENGTH == 1 {
            break;
        }
        x += 1;
    }
    x
}

fn is_not_coprime(a: i32) -> bool {
    gcd(a, ALPHABET_LENGTH) != 1
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if is_not_coprime(a) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    const INTERVAL: i32 = 5;
    let mut index = 0;
    let mut ciphertext = String::with_capacity(plaintext.len());

    for &i in plaintext
        .to_lowercase()
        .as_bytes()
        .iter()
        .filter(|i| i.is_ascii_alphanumeric())
    {
        if index == INTERVAL {
            ciphertext.push(' ');
            index = 0;
        }
        index += 1;
        if i.is_ascii_alphabetic() {
            let i = (i - LOWER_A as u8) as i32;
            let c = ((a * i + b).rem_euclid(ALPHABET_LENGTH) + LOWER_A) as u8 as char;
            ciphertext.push(c);
        } else {
            ciphertext.push(i as char);
        }
    }
    Ok(ciphertext)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if is_not_coprime(a) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let ax = mod_mul_inv(a);
    let mut plaintext = String::with_capacity(ciphertext.len());
    for &i in ciphertext
        .as_bytes()
        .iter()
        .filter(|i| i.is_ascii_alphanumeric())
    {
        if i.is_ascii_alphabetic() {
            let y = (i - LOWER_A as u8) as i32;
            let c = ((ax * (y - b)).rem_euclid(ALPHABET_LENGTH) + LOWER_A) as u8 as char;
            plaintext.push(c);
        } else {
            plaintext.push(i as char);
        }
    }
    Ok(plaintext)
}
