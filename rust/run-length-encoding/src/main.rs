use std::iter::Peekable;

struct SequentialCount<I>
where
    I: Iterator,
{
    iter: Peekable<I>,
}

impl<I> SequentialCount<I>
where
    I: Iterator,
{
    fn new(iter: I) -> Self {
        SequentialCount {
            iter: iter.peekable(),
        }
    }
}

impl<I> Iterator for SequentialCount<I>
where
    I: Iterator,
    I::Item: Eq,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(letter) => {
                let mut count = 1;
                while self.iter.peek() == Some(&letter) {
                    self.iter.next();
                    count += 1;
                }
                Some((letter, count))
            }
            _ => None,
        }
    }
}

fn main() {
    let s = "2 hs2q q2w2 ";
    let digits = s.split(char::is_alphabetic).collect::<Vec<_>>();
    let alpha = s.matches(char::is_alphabetic).collect::<Vec<_>>();

    println!(
        "{:?}",
        s.matches(|c: char| c.is_whitespace() || c.is_alphabetic())
            .collect::<Vec<_>>()
    );

    println!(
        "{:?}",
        s.split(|c: char| c.is_whitespace() || c.is_alphabetic())
            .collect::<Vec<_>>()
    );

    let result = s
        .split(|c: char| c.is_whitespace() || c.is_alphabetic())
        .zip(s.matches(|c: char| c.is_whitespace() || c.is_alphabetic()))
        .map(|(count, letter)| match count.parse::<usize>() {
            Ok(value) => letter.repeat(value),
            _ => letter.to_owned(),
        })
        .collect::<String>();
    println!("{}", result);
}
