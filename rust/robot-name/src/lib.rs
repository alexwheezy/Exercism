use std::sync::atomic::{AtomicU16, Ordering};
static COUNTER: AtomicU16 = AtomicU16::new(0);

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        let value = COUNTER.load(Ordering::SeqCst);
        let c1 = (((1 << ((value % 123) % 32)) ^ 0xffff_ffff) % 26) + 65;
        let c2 = (((1 << ((value % 234 + 679) % 32)) ^ 0xffff_ffff) % 26) + 65;
        let n1: u32 = ((value as u32 ^ c1 ^ c2) % 900) + 100;
        COUNTER.store(value + 1, Ordering::SeqCst);
        Self(format!(
            "{}{}{}",
            char::from_u32(c1).unwrap(),
            char::from_u32(c2).unwrap(),
            n1
        ))
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        *self = Self::new();
    }
}
