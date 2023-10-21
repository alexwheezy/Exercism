use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = vec![];
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut result = HashMap::new();

    for chunk in chunks {
        let string = chunk.join("");
        let handle = thread::spawn(move || {
            let mut map = HashMap::new();
            for c in string.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
            }
            map
        });
        handles.push(handle);
    }

    for handle in handles {
        let map = handle.join().unwrap();
        for (key, value) in map {
            *result.entry(key).or_insert(0) += value;
        }
    }
    result
}
