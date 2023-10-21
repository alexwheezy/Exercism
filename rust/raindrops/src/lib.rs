pub fn raindrops(n: u32) -> String {
    const STRINGS: [&str; 3] = ["Pling", "Plang", "Plong"];
    let result: String = [3, 5, 7]
        .iter()
        .enumerate()
        .filter(|(_, i)| n % *i == 0)
        .fold("".to_string(), |acc, (e, _)|
            format!("{}{}", acc, &STRINGS[e]));
    if result.is_empty(){
        return n.to_string();
    }
    result
}