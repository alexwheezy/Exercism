#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let check = |subset_a: &[T], subset_b: &[T]| -> bool {
        subset_a.windows(subset_b.len()).any(|v| v == subset_b)
    };
    match (first_list, second_list) {
        _ if first_list == second_list => Comparison::Equal,
        _ if first_list.is_empty() || check(second_list, first_list) => Comparison::Sublist,
        _ if second_list.is_empty() || check(first_list, second_list) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
