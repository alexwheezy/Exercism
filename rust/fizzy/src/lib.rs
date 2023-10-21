use std::fmt::Display;
use std::ops::Rem;

pub struct Matcher<T: Display> {
    closure: Box<dyn Fn(T) -> bool>,
    string: String,
}

impl<T: Display> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: Display,
    {
        Self {
            closure: Box::new(matcher),
            string: subs.to_string(),
        }
    }
}

pub struct Fizzy<T>
where
    T: Display + Copy + Clone,
    T: Rem<Output = T> + From<u8>,
    T: PartialEq,
{
    matchers: Vec<Matcher<T>>,
}

impl<T> Default for Fizzy<T>
where
    T: Display + Copy + Clone,
    T: Rem<Output = T> + From<u8>,
    T: PartialEq,
{
    fn default() -> Self {
        let matchers = vec![
            Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"),
            Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"),
        ];
        Self { matchers }
    }
}

impl<T> Fizzy<T>
where
    T: Display + Copy + Clone,
    T: Rem<Output = T> + From<u8>,
    T: PartialEq,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    pub fn apply<I: Iterator<Item = T>>(self, iter: I) -> impl Iterator<Item = String> {
        let matchers = match self.matchers.is_empty() {
            true => Fizzy::default().matchers,
            false => self.matchers,
        };
        iter.map(move |item| {
            let mut word = String::new();
            for matcher in matchers.iter() {
                if (*matcher.closure)(item) {
                    word.push_str(&matcher.string.to_string());
                }
            }
            match word.is_empty() {
                true => item.to_string(),
                false => word,
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Display + Copy + Clone,
    T: Rem<Output = T> + From<u8>,
    T: PartialEq,
{
    Fizzy::new()
}
