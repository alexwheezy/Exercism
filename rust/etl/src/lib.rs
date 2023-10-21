use std::collections::BTreeMap;

pub fn transform(dict: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let from_map = Vec::from_iter(dict.iter());
    let pairs = from_map.iter().map(|&pair| {
        pair.1
            .iter()
            .map(|c| c.to_ascii_lowercase())
            .zip(std::iter::repeat(*pair.0))
    });
    BTreeMap::from_iter(pairs.flatten())
}
