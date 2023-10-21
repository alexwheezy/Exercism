pub fn number(user_number: &str) -> Option<String> {
    if user_number.is_empty() {
        return None;
    }

    let phone_number = user_number
        .chars()
        .filter(|c| c.is_numeric())
        .skip_while(|c| *c == '1')
        .collect::<String>();

    if phone_number.len() != 10 {
        return None;
    }

    let (area_code, exchange_code) = (
        phone_number.chars().next().unwrap(),
        phone_number.chars().nth(3).unwrap(),
    );

    match (area_code < '2', exchange_code < '2') {
        (false, false) => Some(phone_number),
        (_, _) => None,
    }
}
