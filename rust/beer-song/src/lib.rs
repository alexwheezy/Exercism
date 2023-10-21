use std::fmt::format;

pub fn verse(n: u32) -> String {
    match n {
        n @ 3..=99 => {
            format!(
                "{n} bottles of beer on the wall, {n} bottles of beer.
Take one down and pass it around, {} bottles of beer on the wall.\n",
                n - 1
            )
        }
        2 => format!(
            "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.\n"
        ),
        1 => format!(
            "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.\n"
        ),
        _ => format!(
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n"
        ),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    let mut range: Vec<u32> = (start..end + 1).collect();
    if start > end {
        range = (end..start + 1).collect();
        range.reverse();
    }

    let mut counter: usize = 0;
    for value in &range{
        result.push_str(verse(*value).as_str());
        if range.len() > 1 && counter < range.len() - 1{
            result.push_str("\n");
        }
        counter += 1;
    }

    result
}