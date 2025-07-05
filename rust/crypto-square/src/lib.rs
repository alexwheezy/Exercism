fn compute_size_of_rectangle(length_of_message: usize) -> (usize, usize) {
    let mut r = 1;
    let mut c = 1;
    for _ in 0..length_of_message {
        if c >= r && c - r <= 1 {
            r += 1;
        } else {
            c += 1;
        }
        if c * r >= length_of_message {
            break;
        }
    }
    (r, c)
}

pub fn encrypt(input: &str) -> String {
    let normalized = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>();

    if normalized.len() <= 1 {
        return normalized;
    }

    let capacity = (normalized.len() as f32 * 1.5) as usize;
    let mut chipher = String::with_capacity(capacity);
    let (row, col) = compute_size_of_rectangle(normalized.len());
    let chunks: Vec<&[u8]> = normalized.as_bytes().chunks(row).collect();
    for r in 0..row {
        for c in 0..col {
            if chunks.get(c).is_some_and(|arr| arr.get(r).is_some()) {
                chipher.push(chunks[c][r] as char);
            } else {
                chipher.push(' ');
            }
        }
        if r < row - 1 {
            chipher.push(' ');
        }
    }
    chipher
}
