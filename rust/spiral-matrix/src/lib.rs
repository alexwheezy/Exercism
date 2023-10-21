const VECTORS: ([isize; 4], [isize; 4]) = ([0, 1, 0, -1], [1, 0, -1, 0]);

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    if size == 0 {
        return vec![];
    }
    let mut matrix = vec![vec![0; size]; size];
    let (dx, dy) = VECTORS;
    let (mut x, mut y, mut c) = (0, -1, 1);
    (0..size * 2).for_each(|i| {
        (0..(size * 2 - i) / 2).for_each(|_| {
            x += dx[i % 4];
            y += dy[i % 4];
            matrix[x as usize][y as usize] = c;
            c += 1;
        });
    });
    matrix
}
