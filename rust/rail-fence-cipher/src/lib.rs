pub struct RailFence {
    rails: u32,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self { rails }
    }

    fn zigzag_index(index: usize, size: isize) -> usize {
        ((index as isize - size).rem_euclid(size * 2) - size).unsigned_abs()
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails: Vec<Vec<char>> = vec![vec![]; self.rails as usize];
        let size = self.rails as isize - 1;

        text.chars()
            .enumerate()
            .for_each(|(idx, chr)| rails[Self::zigzag_index(idx, size)].push(chr));

        rails.iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rails = vec![vec!['*'; cipher.len()]; self.rails as usize];
        let size = self.rails as isize - 1;

        (0..cipher.len()).for_each(|idx| rails[Self::zigzag_index(idx, size)][idx] = '?');

        let mut next_char = cipher.chars();
        rails.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|curr_char| {
                if *curr_char == '?' {
                    *curr_char = next_char.next().unwrap();
                }
            });
        });

        (0..cipher.len())
            .map(|index| rails[Self::zigzag_index(index, size)][index])
            .collect()
    }
}
