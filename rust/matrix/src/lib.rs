pub struct Matrix {
    data: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        assert!(!input.is_empty());
        let data = input
            .lines()
            .map(|s| {
                s.split(char::is_whitespace)
                    .map(|d| d.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        Self { data }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no > self.data.len() {
            return None;
        }
        Some(self.data[row_no - 1].clone())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no > self.data[0].len() {
            return None;
        }
        Some(
            self.data
                .iter()
                .map(|row| row[col_no - 1])
                .collect::<Vec<_>>(),
        )
    }
}
