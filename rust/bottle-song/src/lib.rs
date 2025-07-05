const NUMBER_WORDS: [(&str, &str); 11] = [
    ("No", "no"),
    ("One", "one"),
    ("Two", "two"),
    ("Three", "three"),
    ("Four", "four"),
    ("Five", "five"),
    ("Six", "six"),
    ("Seven", "seven"),
    ("Eight", "eight"),
    ("Nine", "nine"),
    ("Ten", "ten"),
];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if start_bottles < 1 || take_down < 1 {
        panic!("Need at least 1 bottle and 1 take down");
    }

    const MAX_BUFFER_SIZE: usize = 4000;
    let mut buffer = String::with_capacity(MAX_BUFFER_SIZE);
    let mut begin = start_bottles as usize;
    let plural = |conditon: bool| if conditon { "" } else { "s" };

    (0..take_down).rev().for_each(|_| {
        let (title, lower) = (NUMBER_WORDS[begin].0, NUMBER_WORDS[begin - 1].1);
        let suffix = plural(begin == 1);
        let plural = plural(begin - 1 == 1);
        let phrase = format!(
            "{title} green bottle{suffix} hanging on the wall,
{title} green bottle{suffix} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {lower} green bottle{plural} hanging on the wall.

",
        );
        buffer.push_str(&phrase);
        begin -= 1;
    });

    buffer.to_owned()
}
