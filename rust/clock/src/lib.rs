use std::fmt;

const DAY: i32 = 24; // hour in day 24 time system
const HOUR: i32 = 60; // minutes in hour

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_mins = hours * HOUR + minutes;
        if total_mins < 0 {
            total_mins += (1 - total_mins / (DAY * HOUR)) * DAY * HOUR
        }
        Clock {
            hours: (total_mins / HOUR) % DAY,
            minutes: total_mins % HOUR,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
