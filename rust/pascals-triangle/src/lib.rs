pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = vec![vec![]; self.row_count as usize];
        for idx in 0..self.row_count {
            let it = idx as usize;
            match idx {
                0..=1 => result[it] = vec![1; it + 1],
                _ => {
                    let mut tmp_vec = Vec::from(result[it - 1].as_slice());
                    let vec_len = result[it - 1].len();
                    let new_vec = result[it - 1]
                        .windows(2)
                        .map(|v| v[0] + v[1])
                        .collect::<Vec<_>>();
                    tmp_vec.splice(1..vec_len - 1, new_vec);
                    result[it] = tmp_vec;
                }
            }
        }
        result
    }
}
