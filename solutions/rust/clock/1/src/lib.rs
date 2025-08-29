use std::fmt::{self, Display};

/// A clock that handles times without dates
#[derive(Debug, PartialEq)]
pub struct Clock {
    /// Hours of the clock
    hours: i32,
    /// Minutes of the clock
    minutes: i32
}

impl Clock {
    /// Create new instance of Clocks
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = count_minutes_and_hours(hours, minutes);

        Clock {
            hours,
            minutes,
        }
    }

    /// Add minutes to the clock
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = count_minutes_and_hours(self.hours, self.minutes + minutes);

        Clock {
            hours,
            minutes,
        }
    }
}

// Implement the Display trait for Clock
impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

/// Properly count hours and minutes overflow
fn count_minutes_and_hours(hours: i32, minutes: i32) -> (i32, i32) {
    let mut hours = hours;
    let mut minutes = minutes;

    while minutes <= 0 {
        hours -= 1;
        minutes += 60;
    }

    while hours <= 0 {
        hours += 24;
    }

    while minutes >= 60 {
        minutes -= 60;
        hours += 1;
    }

    while hours >= 24 {
        hours -= 24;
    }

    (hours, minutes)
}
