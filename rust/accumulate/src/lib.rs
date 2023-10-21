/// What should the type of _function be?
pub fn map<T, O, F>(input: Vec<T>, mut _function: F) -> Vec<O>
where
    F: FnMut(T) -> O,
{
    let mut result = vec![];
    for item in input {
        result.push(_function(item));
    }
    result
}
