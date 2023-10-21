use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    use Classification as Class;
    if num == 0 {
        return None;
    }
    let result = (1..(num / 2) + 1)
        .filter(|i| num % i == 0)
        .sum::<u64>()
        .cmp(&num);
    match result {
        Ordering::Equal => Some(Class::Perfect),
        Ordering::Greater => Some(Class::Abundant),
        Ordering::Less => Some(Class::Deficient),
    }
}
