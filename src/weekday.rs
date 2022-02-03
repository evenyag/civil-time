//! Weekday and related utilities.

use crate::{CivilDay, CivilHour, CivilMinute, CivilSecond, DiffType, YearType};
use std::fmt;

/// An enum with members monday, tuesday, wednesday, thursday, friday,
/// saturday, and sunday.
#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub(crate) const fn from_second(cs: CivilSecond) -> Self {
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

    pub(crate) const fn equals(&self, other: Weekday) -> bool {
        // Can't call PartialEq/Eq in const function.
        *self as usize == other as usize
    }
}

impl fmt::Debug for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            Weekday::Mon => "Mon",
            Weekday::Tue => "Tue",
            Weekday::Wed => "Wed",
            Weekday::Thu => "Thu",
            Weekday::Fri => "Fri",
            Weekday::Sat => "Sat",
            Weekday::Sun => "Sun",
        })
    }
}

const fn next_weekday(cd: CivilDay, wd: Weekday) -> CivilDay {
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
    let base = cd.weekday();
    let mut i = 0;
    loop {
        if base.equals(WEEKDAYS_FORW[i]) {
            let mut j = i + 1;
            loop {
                if wd.equals(WEEKDAYS_FORW[j]) {
                    return cd.add_diff((j - i) as DiffType);
                }
                j += 1;
            }
        }
        i += 1;
    }
}

const fn prev_weekday(cd: CivilDay, wd: Weekday) -> CivilDay {
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
    let base = cd.weekday();
    let mut i = 0;
    loop {
        if base.equals(WEEKDAYS_BACK[i]) {
            let mut j = i + 1;
            loop {
                if wd.equals(WEEKDAYS_BACK[j]) {
                    return cd.sub_diff((j - i) as DiffType);
                }
                j += 1;
            }
        }
        i += 1;
    }
}

macro_rules! impl_weekday_ops {
    ($Type: ty) => {
        impl $Type {
            /// Returns the [CivilDay] that strictly follows the day of given
            /// civil time, and that falls on the given weekday `wd`.
            pub const fn next_weekday(self, wd: Weekday) -> CivilDay {
                let cd = CivilDay::from_fields(self.0);
                next_weekday(cd, wd)
            }

            /// Returns the [CivilDay] that strictly precedes the day of given
            /// civil time, and that falls on the given weekday `wd`.
            pub const fn prev_weekday(self, wd: Weekday) -> CivilDay {
                let cd = CivilDay::from_fields(self.0);
                prev_weekday(cd, wd)
            }
        }
    };
}

impl_weekday_ops!(CivilSecond);
impl_weekday_ops!(CivilMinute);
impl_weekday_ops!(CivilHour);
impl_weekday_ops!(CivilDay);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::expect_eq;

    #[test]
    fn test_debug_format() {
        expect_eq("Mon", Weekday::Mon);
        expect_eq("Tue", Weekday::Tue);
        expect_eq("Wed", Weekday::Wed);
        expect_eq("Thu", Weekday::Thu);
        expect_eq("Fri", Weekday::Fri);
        expect_eq("Sat", Weekday::Sat);
        expect_eq("Sun", Weekday::Sun);
    }

    #[test]
    fn test_next_prev_weekday() {
        // Jan 1, 1970 was a Thursday.
        let thursday = CivilDay::new(1970, 1, 1);
        assert_eq!(Weekday::Thu, thursday.weekday());

        // Thursday -> Thursday
        let d = thursday.next_weekday(Weekday::Thu);
        assert_eq!(7, d - thursday);
        assert_eq!(d - 14, thursday.prev_weekday(Weekday::Thu));

        // Thursday -> Friday
        let d = thursday.next_weekday(Weekday::Fri);
        assert_eq!(1, d - thursday);
        assert_eq!(d - 7, thursday.prev_weekday(Weekday::Fri));

        // Thursday -> Saturday
        let d = thursday.next_weekday(Weekday::Sat);
        assert_eq!(2, d - thursday);
        assert_eq!(d - 7, thursday.prev_weekday(Weekday::Sat));

        // Thursday -> Sunday
        let d = thursday.next_weekday(Weekday::Sun);
        assert_eq!(3, d - thursday);
        assert_eq!(d - 7, thursday.prev_weekday(Weekday::Sun));

        // Thursday -> Monday
        let d = thursday.next_weekday(Weekday::Mon);
        assert_eq!(4, d - thursday);
        assert_eq!(d - 7, thursday.prev_weekday(Weekday::Mon));

        // Thursday -> Tuesday
        let d = thursday.next_weekday(Weekday::Tue);
        assert_eq!(5, d - thursday);
        assert_eq!(d - 7, thursday.prev_weekday(Weekday::Tue));

        // Thursday -> Wednesday
        let d = thursday.next_weekday(Weekday::Wed);
        assert_eq!(6, d - thursday);
        assert_eq!(d - 7, thursday.prev_weekday(Weekday::Wed));
    }

    #[test]
    fn test_first_thursday_in_month() {
        let nov1 = CivilDay::new(2014, 11, 1);
        let thursday = (nov1 - 1).next_weekday(Weekday::Thu);
        expect_eq("2014-11-06", thursday);

        // Bonus: Date of Thanksgiving in the United States
        // Rule: Fourth Thursday of November
        let thanksgiving = thursday + 7 * 3;
        expect_eq("2014-11-27", thanksgiving);
    }
}
