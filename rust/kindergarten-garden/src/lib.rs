use std::collections::HashMap;

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    const MAX_STUDENTS: u8 = 11;
    let mut result = Vec::with_capacity(4);
    let idx = {
        let sub = 'L' as u8 - _student.chars().nth(0).unwrap() as u8;
        (MAX_STUDENTS - sub) as usize
    };
    for chipher in _diagram.lines() {
        for c in chipher.chars().skip(idx * 2).take(2) {
            let plant = match c {
                'C' => "clover",
                'G' => "grass",
                'R' => "radishes",
                'V' => "violets",
                _ => "",
            };
            result.push(plant);
        }
    }
    result
}
