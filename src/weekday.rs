//! Weekday and related utilities.

use crate::{CivilDay, CivilSecond, DiffType, YearType};

/// The day of week.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weekday {
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
    // TODO(evenyag): Impl for all civil time.
    pub const fn from_civil_second(cs: CivilSecond) -> Self {
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

    const fn const_eq(&self, other: Weekday) -> bool {
        // Can't call PartialEq/Eq in const function.
        *self as usize == other as usize
    }
}

// TODO(evenyag): Impl for all civil time which can convert into CivilDay.
pub const fn next_weekday(cd: CivilDay, wd: Weekday) -> CivilDay {
    const WEEKDAYS_FORW: [Weekday; 14] = [
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
        Weekday::Sun,
    ];
    let base = Weekday::from_civil_second(CivilSecond::from_civil_day(cd));
    let mut i = 0;
    loop {
        if base.const_eq(WEEKDAYS_FORW[i]) {
            let mut j = i + 1;
            loop {
                if wd.const_eq(WEEKDAYS_FORW[j]) {
                    return cd.add_diff((j - i) as DiffType);
                }
                j += 1;
            }
        }
        i += 1;
    }
}

// TODO(evenyag): Impl for all civil time which can convert into CivilDay.
pub const fn prev_weekday(cd: CivilDay, wd: Weekday) -> CivilDay {
    const WEEKDAYS_BACK: [Weekday; 14] = [
        Weekday::Sun,
        Weekday::Sat,
        Weekday::Fri,
        Weekday::Thu,
        Weekday::Wed,
        Weekday::Tue,
        Weekday::Mon,
        Weekday::Sun,
        Weekday::Sat,
        Weekday::Fri,
        Weekday::Thu,
        Weekday::Wed,
        Weekday::Tue,
        Weekday::Mon,
    ];
    let base = Weekday::from_civil_second(CivilSecond::from_civil_day(cd));
    let mut i = 0;
    loop {
        if base.const_eq(WEEKDAYS_BACK[i]) {
            let mut j = i + 1;
            loop {
                if wd.const_eq(WEEKDAYS_BACK[j]) {
                    return cd.sub_diff((j - i) as DiffType);
                }
                j += 1;
            }
        }
        i += 1;
    }
}
