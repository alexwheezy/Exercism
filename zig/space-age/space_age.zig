const std = @import("std");

pub const Planet = enum {
    earth,
    jupiter,
    mars,
    mercury,
    neptune,
    saturn,
    uranus,
    venus,
    pub fn age(self: Planet, seconds: usize) f64 {
        // Earth's orbital period is 365.25 Earth days, or 31,557,600 seconds
        const period = 31_557_600;
        const secs: f64 = @floatFromInt(seconds);
        // Orbitals in the order of Earth, Jupiter, Mars, Mercury, Neptune, Saturn, Uranus, Venus
        return switch (self) {
            .earth => secs / (period * 1.0),
            .jupiter => secs / (period * 11.862615),
            .mars => secs / (period * 1.8808158),
            .mercury => secs / (period * 0.2408467),
            .neptune => secs / (period * 164.79132),
            .saturn => secs / (period * 29.447498),
            .uranus => secs / (period * 84.016846),
            .venus => secs / (period * 0.61519726),
        };
    }
};
