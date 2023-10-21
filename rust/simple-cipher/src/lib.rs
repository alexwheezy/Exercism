use rand::Rng;

fn encrypt(key: &str, s: &str, reverse: bool) -> Option<String> {
    if key.is_empty() || key.chars().any(|c| c.is_numeric() || c.is_uppercase()) {
        return None;
    }

    Some(
        s.bytes()
            .zip(key.bytes().cycle())
            .map(|(v, k)| {
                let key_b = k as i16 - 97;
                let mut offset = match reverse {
                    true => (v as i16 - key_b) as u8,
                    false => (v as i16 + key_b) as u8,
                };
                if offset >= 123 {
                    offset = (offset % 123) + 97;
                } else if offset < 97 {
                    offset = 123 - (97 % offset);
                }
                offset as char
            })
            .collect(),
    )
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    encrypt(key, s, false)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    encrypt(key, s, true)
}

pub fn encode_random(s: &str) -> (String, String) {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const KEY_LEN: usize = 127;
    let mut rng = rand::thread_rng();

    let key: String = (0..KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    (key.clone(), encode(&key, s).unwrap())
}
