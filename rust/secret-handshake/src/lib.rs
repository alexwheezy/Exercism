pub fn actions(n: u8) -> Vec<&'static str> {
    const SIGNS: [&'static str; 4] = ["wink", "double blink", "close your eyes", "jump"];

    let mut result = Vec::with_capacity(4);
    for sh in 0..=4u8 {
        match (1 << sh) & n {
            0x1..=0x8 => result.push(SIGNS[sh as usize]),
            0x10 => {
                result.reverse();
            }
            _ => {}
        }
    }
    result
}
