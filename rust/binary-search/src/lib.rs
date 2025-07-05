use std::cmp::Ordering;

pub fn find<T, U>(array: T, key: U) -> Option<usize>
where
    T: AsRef<[U]>,
    U: PartialEq + Ord,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = (array.len() - 1) as i32;
    while left <= right {
        let mid = left + (right - left) / 2;
        match array[mid as usize].cmp(&key) {
            Ordering::Equal => return Some(mid as usize),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1,
        }
    }
    None
}
