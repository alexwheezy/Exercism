const COST: u32 = 800;

fn discount_calculation(len: u32) -> u32 {
    match len {
        2 => len * (COST - 40),  // 5% discount
        3 => len * (COST - 80),  // 10% discount
        4 => len * (COST - 160), // 20% discount
        5 => len * (COST - 200), // 25% discount
        _ => COST,
    }
}

fn total_cost(data: Vec<Vec<u32>>) -> u32 {
    data.iter()
        .map(|items| discount_calculation(items.len() as u32))
        .sum()
}

fn uneven_distribution(data: &[u32]) -> u32 {
    let mut output: Vec<Vec<u32>> = vec![];
    let mut data = data.to_vec();
    data.sort_unstable();

    data.iter().for_each(|&item| {
        if let Some(chunk) = output.iter_mut().find(|chunk| !chunk.contains(&item)) {
            chunk.push(item);
        } else {
            output.push(vec![item]);
        }
    });
    total_cost(output)
}

fn uniform_distribution(data: &[u32]) -> u32 {
    let mut output: Vec<Vec<u32>> = vec![];
    let mut data = data.to_vec();
    data.sort_unstable();

    let mut last = 0;
    data.into_iter().enumerate().for_each(|(idx, item)| {
        if output.is_empty() || item == last {
            output.push(vec![item]);
            last = item;
        } else {
            let len = output.len();
            output[idx % len].push(item);
            last = 0;
        }
    });
    total_cost(output)
}

pub fn lowest_price(books: &[u32]) -> u32 {
    uneven_distribution(&books).min(uniform_distribution(&books))
}
