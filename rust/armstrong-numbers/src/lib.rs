pub fn is_armstrong_number(num: u32) -> bool {
    let find_num = num.to_string().chars().fold(0, |acc, x| {
        acc + (x.to_digit(10).unwrap().pow(num.to_string().len() as u32)) as u64
    });
    find_num == num as u64
}
