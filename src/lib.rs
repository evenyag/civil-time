use crate::core::Fields;
use crate::granularity::{Day, Hour, Minute, Month, Second, Year};
use std::ops::{Add, Sub};

mod convert;
mod core;
mod granularity;
mod weekday;

pub use crate::core::{DiffType, YearType};
pub use crate::weekday::{next_weekday, prev_weekday, Weekday};

macro_rules! impl_civil_time_type {
    ($Type: ident, $Granularity: ident) => {
        impl $Type {
            const fn from_fields(fields: Fields) -> Self {
                $Type($Granularity::align(fields))
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

            pub const fn add_diff(self, n: DiffType) -> Self {
                let fields = $Granularity::step(self.0, n);

                Self::from_fields(fields)
            }

            pub const fn sub_diff(self, n: DiffType) -> Self {
                let fields = if n != DiffType::MIN {
                    $Granularity::step(self.0, -n)
                } else {
                    $Granularity::step($Granularity::step(self.0, -(n + 1)), 1)
                };

                Self::from_fields(fields)
            }

            pub const fn difference(self, other: Self) -> DiffType {
                $Granularity::difference(self.0, other.0)
            }

            pub const fn yearday(self) -> i32 {
                get_yearday(CivilSecond::from_fields(self.0))
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
    };
}

#[derive(Debug, Clone, Copy)]
pub struct CivilSecond(Fields);

// TODO(evenyag): Maybe we can construct different civil time via a generic builder, which
// decide which civil time struct to built via generic param in build() method.
impl CivilSecond {
    pub const fn new(
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
}

#[derive(Debug, Clone, Copy)]
pub struct CivilMinute(Fields);

impl CivilMinute {
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType, mm: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, d, hh, mm, 0);

        Self::from_fields(fields)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilHour(Fields);

impl CivilHour {
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, d, hh, 0, 0);

        Self::from_fields(fields)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilDay(Fields);

impl CivilDay {
    pub const fn new(y: YearType, m: DiffType, d: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, d, 0, 0, 0);

        Self::from_fields(fields)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilMonth(Fields);

impl CivilMonth {
    pub const fn new(y: YearType, m: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, 1, 0, 0, 0);

        Self::from_fields(fields)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilYear(Fields);

impl CivilYear {
    pub const fn new(y: YearType) -> Self {
        let fields = Fields::n_sec(y, 1, 1, 0, 0, 0);

        Self::from_fields(fields)
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
