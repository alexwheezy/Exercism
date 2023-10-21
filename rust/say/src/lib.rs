#[rustfmt::skip]
const ONE_NINETEEN: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
    "ten", "eleven", "tweelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", 
    "eighteen", "nineteen",
];

#[rustfmt::skip]
const TENS: [&str; 8] = [
    "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

#[rustfmt::skip]
const HUGE_NUM: [&str; 7] = [
    "hundred", "thousand", "million", "billion", "trillion", "quadrillion", "quintillion",
];

#[inline]
fn one_num(n: u64) -> String {
    ONE_NINETEEN[n as usize].to_owned()
}

fn two_num(n: u64) -> String {
    if n < ONE_NINETEEN.len() as u64 {
        return one_num(n);
    }
    let (first, last) = (n / 10, n % 10);
    let start_idx = 2;
    let mut result = TENS[(first - start_idx) as usize].to_owned();
    if n % 10 != 0 {
        result = format!("{}-{}", result, one_num(last));
    }
    result
}

fn three_num(n: u64) -> String {
    let (first, last) = (n / 100, n % 100);
    let mut result = format!("{} {}", one_num(first), HUGE_NUM[0]);
    if n % 100 != 0 {
        result = format!("{} {}", result, two_num(last));
    }
    result
}

fn over_num(n: u64) -> String {
    if n == 0 {
        return one_num(n);
    }
    let nums = split_num(n, 1000);
    let mut result = Vec::with_capacity(8);
    for (idx, &num) in nums.iter().enumerate() {
        let mut text = match num {
            0..=19 => one_num(num),
            20..=99 => two_num(num),
            100..=999 => three_num(num),
            _ => unreachable!(),
        };
        if num != 0 {
            if idx != nums.len() - 1 {
                let huge_num = HUGE_NUM[nums.len() - idx - 1];
                text = format!("{text} {huge_num}");
            }
            result.push(text);
        }
    }
    result.join(" ")
}

fn split_num(mut n: u64, rem: u64) -> Vec<u64> {
    let mut digits = Vec::with_capacity(8);
    while n != 0 {
        digits.push(n % rem);
        n /= rem;
    }
    digits.reverse();
    digits
}

pub fn encode(n: u64) -> String {
    over_num(n)
}
