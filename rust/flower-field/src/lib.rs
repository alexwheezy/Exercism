#[rustfmt::skip]
static SDF: &[(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let mut result = Vec::with_capacity(garden.len());
    let mut line = String::new();

    let height = garden.len() as i32;
    for y in 0..height {
        let width = garden[y as usize].len() as i32;
        for x in 0..width {
            let check = if garden[y as usize].as_bytes()[x as usize] == b'*' {
                '*'
            } else {
                let search_mine = SDF
                    .iter()
                    .map(|&(posx, posy)| (x + posx, y + posy))
                    .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                    .filter(|&(x, y)| garden[y as usize].as_bytes()[x as usize] == b'*')
                    .count();
                match search_mine {
                    0 => ' ',
                    value => (value as u8 + b'0') as char,
                }
            };
            line.push(check);
        }
        result.push(line.clone());
        line.clear();
    }
    result
}
