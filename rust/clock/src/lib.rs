use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Self { hours, minutes };
        clock.roll_over();
        return clock;
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut clock = Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        };
        clock.roll_over();
        return clock;
    }

    fn roll_over(&mut self) {
        let m = self.minutes;
        let h_over = match m {
            m if m % 60 == 0 => m / 60,
            0.. => m / 60,
            _ => m / 60 - 1,
        };
        self.minutes = match m {
            0.. => m % 60,
            _ => ((m % -60) + 60) % 60,
        };
        let h = (self.hours + h_over) % 24;
        dbg!(h_over);
        self.hours = match h {
            0.. => h,
            _ => h + 24,
        };
    }
}
