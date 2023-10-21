pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    if items.is_empty() || max_weight == 0 {
        return 0;
    }
    let mut m = vec![vec![0; (max_weight + 1) as usize]; items.len() + 1];
    for i in (0usize..items.len()).rev() {
        for w in (0usize..=max_weight as usize).rev() {
            m[i][w] = m[i + 1][w];
            if items[i].weight <= (w as u32) {
                m[i][w] = std::cmp::max(
                    m[i][w],
                    items[i].value + m[i + 1][w - items[i].weight as usize],
                );
            }
        }
    }
    m[0][max_weight as usize]
}
