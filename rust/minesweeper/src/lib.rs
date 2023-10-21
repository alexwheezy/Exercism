#[rustfmt::skip]
static SDF: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    let mut result = Vec::with_capacity(minefield.len());
    let mut line = String::new();

    let height = minefield.len() as i32;
    for y in 0..height {
        let width = minefield[y as usize].len() as i32;
        for x in 0..width {
            let check = if minefield[y as usize].as_bytes()[x as usize] == b'*' {
                '*'
            } else {
                let search_mine = SDF
                    .iter()
                    .map(|&(posx, posy)| (x + posx, y + posy))
                    .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
                    .filter(|&(x, y)| minefield[y as usize].as_bytes()[x as usize] == b'*')
                    .count();
                match search_mine {
                    0 => ' ',
                    value => (value as u8 + '0' as u8) as char,
                }
            };
            line.push(check);
        }
        result.push(line.clone());
        line.clear();
    }
    result
}
