//! Weekday

use crate::{CivilSecond, YearType, CivilDay};

/// The day of week.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Weekday {
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday,
    Sat,
    /// Sunday,
    Sun,
}

impl Weekday {
    const fn from_civil_second(cs: CivilSecond) -> Self {
        const WEEKDAY_BY_MON_OFF: [Weekday; 13] = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
        ];
        const WEEKDAY_OFFSETS: [i32; 13] = [-1, 0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

        let mut wd = 2400 + (cs.year() % 400) - if cs.month() < 3 { 1 } else { 0 };
        wd += wd / 4 - wd / 100 + wd / 400;
        wd += (WEEKDAY_OFFSETS[cs.month() as usize] + cs.day()) as YearType;
        let index = (wd % 7 + 6) as usize;

        WEEKDAY_BY_MON_OFF[index]
    }
}
