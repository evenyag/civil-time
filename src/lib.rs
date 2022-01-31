use crate::core::Fields;
use crate::granularity::{Day, Hour, Minute, Month, Second, Year};
use std::ops::{Add, Sub};

mod convert;
mod core;
mod granularity;
mod weekday;

pub use crate::core::{DiffType, YearType};
pub use crate::weekday::Weekday;

pub trait BuildCivilTime {
    fn build_from_ymd_hms(
        y: YearType,
        m: DiffType,
        d: DiffType,
        hh: DiffType,
        mm: DiffType,
        ss: DiffType,
    ) -> Self;
}

macro_rules! impl_civil_time_type {
    ($Type: ident, $Granularity: ident) => {
        impl $Type {
            pub const MAX: $Type = $Type::from_ymd_hms(DiffType::MAX, 12, 31, 23, 59, 59);
            pub const MIN: $Type = $Type::from_ymd_hms(DiffType::MIN, 1, 1, 0, 0, 0);

            const fn from_fields(fields: Fields) -> Self {
                $Type($Granularity::align(fields))
            }

            const fn from_ymd_hms(
                y: YearType,
                m: DiffType,
                d: DiffType,
                hh: DiffType,
                mm: DiffType,
                ss: DiffType,
            ) -> Self {
                let fields = Fields::n_sec(y, m, d, hh, mm, ss);

                Self::from_fields(fields)
            }

            pub const fn year(&self) -> YearType {
                self.0.y
            }

            pub const fn month(&self) -> i32 {
                self.0.m as i32
            }

            pub const fn day(&self) -> i32 {
                self.0.d as i32
            }

            pub const fn hour(&self) -> i32 {
                self.0.hh as i32
            }

            pub const fn minute(&self) -> i32 {
                self.0.mm as i32
            }

            pub const fn second(&self) -> i32 {
                self.0.ss as i32
            }

            pub const fn weekday(&self) -> Weekday {
                Weekday::from_second(CivilSecond::from_fields(self.0))
            }

            pub const fn yearday(&self) -> i32 {
                get_yearday(CivilSecond::from_fields(self.0))
            }

            const fn add_diff(self, n: DiffType) -> Self {
                let fields = $Granularity::step(self.0, n);

                Self::from_fields(fields)
            }

            const fn sub_diff(self, n: DiffType) -> Self {
                let fields = if n != DiffType::MIN {
                    $Granularity::step(self.0, -n)
                } else {
                    $Granularity::step($Granularity::step(self.0, -(n + 1)), 1)
                };

                Self::from_fields(fields)
            }

            const fn difference(self, other: Self) -> DiffType {
                $Granularity::difference(self.0, other.0)
            }
        }

        impl Add<DiffType> for $Type {
            type Output = Self;

            fn add(self, n: DiffType) -> Self::Output {
                self.add_diff(n)
            }
        }

        impl Sub<DiffType> for $Type {
            type Output = Self;

            fn sub(self, n: DiffType) -> Self::Output {
                self.sub_diff(n)
            }
        }

        impl Sub for $Type {
            type Output = DiffType;

            fn sub(self, rhs: Self) -> Self::Output {
                self.difference(rhs)
            }
        }

        impl BuildCivilTime for $Type {
            fn build_from_ymd_hms(
                y: YearType,
                m: DiffType,
                d: DiffType,
                hh: DiffType,
                mm: DiffType,
                ss: DiffType,
            ) -> Self {
                Self::from_ymd_hms(y, m, d, hh, mm, ss)
            }
        }

        impl Default for $Type {
            fn default() -> Self {
                Builder::default().build()
            }
        }
    };
}

#[derive(Debug, Clone, Copy)]
pub struct CivilSecond(Fields);

impl CivilSecond {
    pub const fn new(
        y: YearType,
        m: DiffType,
        d: DiffType,
        hh: DiffType,
        mm: DiffType,
        ss: DiffType,
    ) -> Self {
        Self::from_ymd_hms(y, m, d, hh, mm, ss)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilMinute(Fields);

impl CivilMinute {
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType, mm: DiffType) -> Self {
        Self::from_ymd_hms(y, m, d, hh, mm, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilHour(Fields);

impl CivilHour {
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType) -> Self {
        Self::from_ymd_hms(y, m, d, hh, 0, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilDay(Fields);

impl CivilDay {
    pub const fn new(y: YearType, m: DiffType, d: DiffType) -> Self {
        Self::from_ymd_hms(y, m, d, 0, 0, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilMonth(Fields);

impl CivilMonth {
    pub const fn new(y: YearType, m: DiffType) -> Self {
        Self::from_ymd_hms(y, m, 1, 0, 0, 0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilYear(Fields);

impl CivilYear {
    pub const fn new(y: YearType) -> Self {
        Self::from_ymd_hms(y, 1, 1, 0, 0, 0)
    }
}

impl_civil_time_type!(CivilSecond, Second);
impl_civil_time_type!(CivilMinute, Minute);
impl_civil_time_type!(CivilHour, Hour);
impl_civil_time_type!(CivilDay, Day);
impl_civil_time_type!(CivilMonth, Month);
impl_civil_time_type!(CivilYear, Year);

const fn get_yearday(cs: CivilSecond) -> i32 {
    const MONTH_OFFSETS: [i32; 13] = [-1, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let feb29 = if cs.month() > 2 && core::is_leap_year(cs.year()) {
        1
    } else {
        0
    };
    MONTH_OFFSETS[cs.month() as usize] + feb29 + cs.day()
}

#[derive(Clone, Copy)]
pub struct Builder {
    y: YearType,
    m: DiffType,
    d: DiffType,
    hh: DiffType,
    mm: DiffType,
    ss: DiffType,
}

impl Builder {
    pub const fn new() -> Self {
        Self {
            y: 1970,
            m: 1,
            d: 1,
            hh: 0,
            mm: 0,
            ss: 0,
        }
    }

    pub const fn year(mut self, y: YearType) -> Self {
        self.y = y;
        self
    }

    pub const fn month(mut self, m: DiffType) -> Self {
        self.m = m;
        self
    }

    pub const fn day(mut self, d: DiffType) -> Self {
        self.d = d;
        self
    }

    pub const fn hh(mut self, hh: DiffType) -> Self {
        self.hh = hh;
        self
    }

    pub const fn mm(mut self, mm: DiffType) -> Self {
        self.mm = mm;
        self
    }

    pub const fn ss(mut self, ss: DiffType) -> Self {
        self.ss = ss;
        self
    }

    pub fn build<T: BuildCivilTime>(self) -> T {
        T::build_from_ymd_hms(self.y, self.m, self.d, self.hh, self.mm, self.ss)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_build {
    ($func: ident, $Type: ty) => {
        impl Builder {
            pub const fn $func(self) -> $Type {
                <$Type>::from_ymd_hms(self.y, self.m, self.d, self.hh, self.mm, self.ss)
            }
        }
    };
}

// Implement build method for each civil time type.
impl_build!(build_second, CivilSecond);
impl_build!(build_minute, CivilMinute);
impl_build!(build_hour, CivilHour);
impl_build!(build_day, CivilDay);
impl_build!(build_month, CivilMonth);
impl_build!(build_year, CivilYear);

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions as sa;

    // const tests

    #[test]
    fn test_normal() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 28, 17, 14, 12);
        sa::const_assert_eq!(12, _CSS.second());
        const _CMM: CivilMinute = CivilMinute::new(2016, 1, 28, 17, 14);
        sa::const_assert_eq!(14, _CMM.minute());
        const _CHH: CivilHour = CivilHour::new(2016, 1, 28, 17);
        sa::const_assert_eq!(17, _CHH.hour());
        const _CD: CivilDay = CivilDay::new(2016, 1, 28);
        sa::const_assert_eq!(28, _CD.day());
        const _CM: CivilMonth = CivilMonth::new(2016, 1);
        sa::const_assert_eq!(1, _CM.month());
        const _CY: CivilYear = CivilYear::new(2016);
        sa::const_assert_eq!(2016, _CY.year());
    }

    #[test]
    fn test_normalized() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 28, 17, 14, 12);
        sa::const_assert_eq!(2016, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(28, _CSS.day());
        sa::const_assert_eq!(17, _CSS.hour());
        sa::const_assert_eq!(14, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_second_overflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 28, 17, 14, 121);
        sa::const_assert_eq!(2016, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(28, _CSS.day());
        sa::const_assert_eq!(17, _CSS.hour());
        sa::const_assert_eq!(16, _CSS.minute());
        sa::const_assert_eq!(1, _CSS.second());
    }

    #[test]
    fn test_second_underflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 28, 17, 14, -121);
        sa::const_assert_eq!(2016, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(28, _CSS.day());
        sa::const_assert_eq!(17, _CSS.hour());
        sa::const_assert_eq!(11, _CSS.minute());
        sa::const_assert_eq!(59, _CSS.second());
    }

    #[test]
    fn test_minute_overflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 28, 17, 121, 12);
        sa::const_assert_eq!(2016, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(28, _CSS.day());
        sa::const_assert_eq!(19, _CSS.hour());
        sa::const_assert_eq!(1, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_minute_underflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 28, 17, -121, 12);
        sa::const_assert_eq!(2016, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(28, _CSS.day());
        sa::const_assert_eq!(14, _CSS.hour());
        sa::const_assert_eq!(59, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_hour_overflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 28, 49, 14, 12);
        sa::const_assert_eq!(2016, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(30, _CSS.day());
        sa::const_assert_eq!(1, _CSS.hour());
        sa::const_assert_eq!(14, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_hour_underflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 28, -49, 14, 12);
        sa::const_assert_eq!(2016, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(25, _CSS.day());
        sa::const_assert_eq!(23, _CSS.hour());
        sa::const_assert_eq!(14, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_month_overflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 25, 28, 17, 14, 12);
        sa::const_assert_eq!(2018, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(28, _CSS.day());
        sa::const_assert_eq!(17, _CSS.hour());
        sa::const_assert_eq!(14, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_month_underflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, -25, 28, 17, 14, 12);
        sa::const_assert_eq!(2013, _CSS.year());
        sa::const_assert_eq!(11, _CSS.month());
        sa::const_assert_eq!(28, _CSS.day());
        sa::const_assert_eq!(17, _CSS.hour());
        sa::const_assert_eq!(14, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_c4_overflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, 292195, 17, 14, 12);
        sa::const_assert_eq!(2816, _CSS.year());
        sa::const_assert_eq!(1, _CSS.month());
        sa::const_assert_eq!(1, _CSS.day());
        sa::const_assert_eq!(17, _CSS.hour());
        sa::const_assert_eq!(14, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_c4_underflow() {
        const _CSS: CivilSecond = CivilSecond::new(2016, 1, -292195, 17, 14, 12);
        sa::const_assert_eq!(1215, _CSS.year());
        sa::const_assert_eq!(12, _CSS.month());
        sa::const_assert_eq!(30, _CSS.day());
        sa::const_assert_eq!(17, _CSS.hour());
        sa::const_assert_eq!(14, _CSS.minute());
        sa::const_assert_eq!(12, _CSS.second());
    }

    #[test]
    fn test_mixed_normalization() {
        const _CSS: CivilSecond = CivilSecond::new(2016, -42, 122, 99, -147, 4949);
        sa::const_assert_eq!(2012, _CSS.year());
        sa::const_assert_eq!(10, _CSS.month());
        sa::const_assert_eq!(4, _CSS.day());
        sa::const_assert_eq!(1, _CSS.hour());
        sa::const_assert_eq!(55, _CSS.minute());
        sa::const_assert_eq!(29, _CSS.second());
    }

    // Arithmetic const tests
    #[test]
    fn test_addition() {
        const _CS1: CivilSecond = CivilSecond::new(2016, 1, 28, 17, 14, 12);
        const _CS2: CivilSecond = _CS1.add_diff(50);
        sa::const_assert_eq!(2016, _CS2.year());
        sa::const_assert_eq!(1, _CS2.month());
        sa::const_assert_eq!(28, _CS2.day());
        sa::const_assert_eq!(17, _CS2.hour());
        sa::const_assert_eq!(15, _CS2.minute());
        sa::const_assert_eq!(2, _CS2.second());
    }

    #[test]
    fn test_subtraction() {
        const _CS1: CivilSecond = CivilSecond::new(2016, 1, 28, 17, 14, 12);
        const _CS2: CivilSecond = _CS1.sub_diff(50);
        sa::const_assert_eq!(2016, _CS2.year());
        sa::const_assert_eq!(1, _CS2.month());
        sa::const_assert_eq!(28, _CS2.day());
        sa::const_assert_eq!(17, _CS2.hour());
        sa::const_assert_eq!(13, _CS2.minute());
        sa::const_assert_eq!(22, _CS2.second());
    }

    #[test]
    fn test_difference() {
        const _CD1: CivilDay = CivilDay::new(2016, 1, 28);
        const _CD2: CivilDay = CivilDay::new(2015, 1, 28);
        const _DIFF: DiffType = _CD1.difference(_CD2);
        sa::const_assert_eq!(365, _DIFF);
    }

    #[test]
    fn test_new_with_huge_year() {
        const _H: CivilHour = CivilHour::new(-9223372036854775807, 1, 1, -1);
        sa::const_assert_eq!(-9223372036854775807 - 1, _H.year());
        sa::const_assert_eq!(12, _H.month());
        sa::const_assert_eq!(31, _H.day());
        sa::const_assert_eq!(23, _H.hour());
    }

    #[test]
    fn test_difference_with_huge_year() {
        {
            const _D1: CivilDay = CivilDay::new(9223372036854775807, 1, 1);
            const _D2: CivilDay = CivilDay::new(9223372036854775807, 12, 31);
            sa::const_assert_eq!(364, _D2.difference(_D1));
        }
        {
            const _D1: CivilDay = CivilDay::new(-9223372036854775807 - 1, 1, 1);
            const _D2: CivilDay = CivilDay::new(-9223372036854775807 - 1, 12, 31);
            sa::const_assert_eq!(365, _D2.difference(_D1));
        }
        {
            // Check the limits of the return value at the end of the year range.
            const _D1: CivilDay = CivilDay::new(9223372036854775807, 1, 1);
            const _D2: CivilDay = CivilDay::new(9198119301927009252, 6, 6);
            sa::const_assert_eq!(9223372036854775807, _D1.difference(_D2));
            sa::const_assert_eq!(-9223372036854775807 - 1, (_D2.sub_diff(1)).difference(_D1));
        }
        {
            // Check the limits of the return value at the start of the year range.
            const _D1: CivilDay = CivilDay::new(-9223372036854775807 - 1, 1, 1);
            const _D2: CivilDay = CivilDay::new(-9198119301927009254, 7, 28);
            sa::const_assert_eq!(9223372036854775807, _D2.difference(_D1));
            sa::const_assert_eq!(-9223372036854775807 - 1, _D1.difference(_D2.add_diff(1)));
        }
        {
            // Check the limits of the return value from either side of year 0.
            const _D1: CivilDay = CivilDay::new(-12626367463883278, 9, 3);
            const _D2: CivilDay = CivilDay::new(12626367463883277, 3, 28);
            sa::const_assert_eq!(9223372036854775807, _D2.difference(_D1));
            sa::const_assert_eq!(-9223372036854775807 - 1, _D1.difference(_D2.add_diff(1)));
        }
    }

    #[test]
    fn test_difference_no_intermediate_overflow() {
        {
            // The difference up to the minute field would be below the minimum
            // DiffType, but the 52 extra seconds brings us back to the minimum.
            const _S1: CivilSecond = CivilSecond::new(-292277022657, 1, 27, 8, 29 - 1, 52);
            const _S2: CivilSecond = CivilSecond::new(1970, 1, 1, 0, 0 - 1, 0);
            sa::const_assert_eq!(-9223372036854775807 - 1, _S1.difference(_S2));
        }
        {
            // The difference up to the minute field would be above the maximum
            // DiffType, but the -53 extra seconds brings us back to the maximum.
            const _S1: CivilSecond = CivilSecond::new(292277026596, 12, 4, 15, 30, 7 - 7);
            const _S2: CivilSecond = CivilSecond::new(1970, 1, 1, 0, 0, 0 - 7);
            sa::const_assert_eq!(9223372036854775807, _S1.difference(_S2));
        }
    }

    // Helper const test.
    #[test]
    fn test_weekday() {
        const _CD: CivilDay = CivilDay::new(2016, 1, 28);
        const _WD: Weekday = _CD.weekday();
        sa::const_assert!(_WD.equals(Weekday::Thu));
    }

    #[test]
    fn test_next_weekday() {
        const _CD: CivilDay = CivilDay::new(2016, 1, 28);
        const _NEXT: CivilDay = _CD.next_weekday(Weekday::Thu);
        sa::const_assert_eq!(2016, _NEXT.year());
        sa::const_assert_eq!(2, _NEXT.month());
        sa::const_assert_eq!(4, _NEXT.day());
    }

    #[test]
    fn test_prev_weekday() {
        const _CD: CivilDay = CivilDay::new(2016, 1, 28);
        const _PREV: CivilDay = _CD.prev_weekday(Weekday::Thu);
        sa::const_assert_eq!(2016, _PREV.year());
        sa::const_assert_eq!(1, _PREV.month());
        sa::const_assert_eq!(21, _PREV.day());
    }

    #[test]
    fn test_yearday() {
        const _CD: CivilDay = CivilDay::new(2016, 1, 28);
        const _YD: i32 = _CD.yearday();
        sa::const_assert_eq!(28, _YD);
    }

    // The remaining tests do not use constexpr.

    // TODO(evenyag): Add/Sub/Difference/Compare test without const.
}
