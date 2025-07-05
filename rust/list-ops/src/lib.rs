/// Yields each item of a and then each item of b
pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    let mut a = a;
    let mut b = b;
    std::iter::from_fn(move || a.next().or_else(|| b.next()))
}

pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut outer_iter = nested_iter;
    let mut current_iter: Option<<I as Iterator>::Item> = None;
    std::iter::from_fn(move || {
        loop {
            if let Some(inner_iter) = &mut current_iter {
                if let Some(item) = inner_iter.next() {
                    return Some(item);
                } else {
                    current_iter = None;
                }
            }
            current_iter = Some(outer_iter.next()?);
        }
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    let mut outer_iter = iter;
    std::iter::from_fn(move || {
        if let Some(item) = outer_iter.next() {
            if predicate(&item) {
                Some(item)
            } else {
                outer_iter.next()
            }
        } else {
            None
        }
    })
}

pub fn length<I: Iterator>(iter: I) -> usize {
    let mut count = 0;
    for _ in iter {
        count += 1;
    }
    count
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    let mut outer_iter = iter;
    std::iter::from_fn(move || match outer_iter.next() {
        Some(item) => Some(function(item)),
        _ => None,
    })
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    let mut initial = initial;
    for item in iter {
        initial = function(initial, item);
    }
    initial
}

pub fn foldr<I, F, U>(mut iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    let mut initial = initial;
    while let Some(item) = iter.next_back() {
        initial = function(initial, item);
    }
    initial
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    let mut outer_iter = iter;
    std::iter::from_fn(move || outer_iter.next_back())
}
