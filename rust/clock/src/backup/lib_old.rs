use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::set_time(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::set_time(self.hours, self.minutes + minutes)
    }

    fn set_time(hours: i32, minutes: i32) -> Self {
        let (set_hour, set_minutes) = {
            match (hours.is_negative(), minutes.is_negative()) {
                (true, false) => {
                    let time = (hours.abs() * 60 + minutes) as f32;
                    (24 - (time / 60.0 % 24.0) as i32,
                     minutes % 60)
                },
                (false, true) => {
                    if minutes.abs() > hours * 60{
                        let time = (minutes.abs() - hours * 60) as f32;
                        ((24.0 - (time / 60.0 % 24.0)) as i32,
                         (60 - (time as i32 % 60)) % 60)
                    }else{
                        let time = (hours * 60 - minutes.abs()) as f32;
                        ((time / 60.0 % 24.0) as i32,
                         time as i32 % 60)
                    }
                },
                (true, true) => {
                    let time = (hours.abs() * 60 + minutes.abs()) as f32;
                    ((24.0 - (time / 60.0 % 24.0)) as i32,
                     60 - (time as i32 % 60))
                },
                (_, _) => {
                    let time = (hours * 60 + minutes) as f32;
                    ((time / 60.0 % 24.0) as i32,
                     minutes % 60)
                },
            }
        };
        Self{
            hours: set_hour,
            minutes: set_minutes,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}