use std::collections::BTreeMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
#[derive(Default)]
pub struct School {
    db: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            db: BTreeMap::default(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.db.entry(grade).or_default().push(student.to_owned())
    }

    pub fn grades(&self) -> Vec<u32> {
        self.db.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut result = self.db.get(&grade).cloned().unwrap_or_default();
        result.sort_unstable();
        result
    }
}
