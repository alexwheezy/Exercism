use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

pub fn count(input: &[&str]) -> usize {
    let mut corners = BTreeSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '+' {
                corners.insert(Point { x, y });
            }
        }
    }

    let mut count = 0;
    let data: Vec<_> = corners.iter().copied().collect();

    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let a = data[i];
            let c = data[j];

            if a.x >= c.x || a.y >= c.y {
                continue;
            }

            let b = Point { x: c.x, y: a.y };
            let d = Point { x: a.x, y: c.y };

            if !corners.contains(&b) || !corners.contains(&d) {
                continue;
            }

            if is_horizontal_line_valid(input, a, b)
                && is_horizontal_line_valid(input, d, c)
                && is_vertical_line_valid(input, a, d)
                && is_vertical_line_valid(input, b, c)
            {
                count += 1;
            }
        }
    }

    count
}

fn is_horizontal_line_valid(grid: &[&str], start: Point, end: Point) -> bool {
    if start.y != end.y {
        return false;
    }
    for x in start.x + 1..end.x {
        let c = grid[start.y].chars().nth(x).unwrap();
        if c != '-' && c != '+' {
            return false;
        }
    }
    true
}

fn is_vertical_line_valid(grid: &[&str], start: Point, end: Point) -> bool {
    if start.x != end.x {
        return false;
    }

    for y in grid.iter().take(end.y).skip(start.y + 1) {
        let c = y.chars().nth(start.x).unwrap();
        if c != '|' && c != '+' {
            return false;
        }
    }
    true
}
