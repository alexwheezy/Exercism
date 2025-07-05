pub fn egg_count(display_value: u32) -> usize {
    (0..u32::BITS).fold(0, |acc, n| acc + (display_value & 1 << n != 0) as u32) as usize
}
