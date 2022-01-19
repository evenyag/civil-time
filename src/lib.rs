use crate::fields::Fields;
use crate::granularity::{Day, Hour, Minute, Month, Second, Year};
use std::ops::{Add, Sub};

mod fields;
mod granularity;
mod weekday;

/// Support years that at least span the range of 64-bit time_t values.
type YearType = i64;
/// Type alias that indicates an argument is not normalized (e.g., the
/// constructor parameters and operands/results of addition/subtraction).
type DiffType = i64;

// Type aliases that indicate normalized argument values.
/// Normalized month [1:12].
type MonthType = i8;
/// Normalized day [1:31].
type DayType = i8;
/// Normalized hour [0:23].
type HourType = i8;
/// Normalized minute [0:59].
type MinuteType = i8;
/// Normalized second [0:59].
type SecondType = i8;

#[derive(Debug, Clone, Copy)]
pub struct CivilYear(Fields);

impl CivilYear {
    pub const fn new(y: YearType) -> Self {
        let fields = Fields::n_sec(y, 1, 1, 0, 0, 0);

        Self::from_fields(fields)
    }

    const fn from_fields(fields: Fields) -> Self {
        CivilYear(Year::align(fields))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilMonth(Fields);

impl CivilMonth {
    pub const fn new(y: YearType, m: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, 1, 0, 0, 0);

        Self::from_fields(fields)
    }

    const fn from_fields(fields: Fields) -> Self {
        CivilMonth(Month::align(fields))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilDay(Fields);

impl CivilDay {
    pub const fn new(y: YearType, m: DiffType, d: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, d, 0, 0, 0);

        Self::from_fields(fields)
    }

    const fn from_fields(fields: Fields) -> Self {
        CivilDay(Day::align(fields))
    }

    const fn const_add(self, n: DiffType) -> Self {
        let fields = Day::step(self.0, n);

        Self::from_fields(fields)
    }

    const fn const_sub(self, n: DiffType) -> Self {
        let fields = if n != DiffType::MIN {
            Day::step(self.0, -n)
        } else {
            Day::step(Day::step(self.0, -(n + 1)), 1)
        };

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

    const fn from_fields(fields: Fields) -> Self {
        CivilHour(Hour::align(fields))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CivilMinute(Fields);

impl CivilMinute {
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType, mm: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, d, hh, mm, 0);

        Self::from_fields(fields)
    }

    const fn from_fields(fields: Fields) -> Self {
        CivilMinute(Minute::align(fields))
    }
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

    const fn from_fields(fields: Fields) -> Self {
        CivilSecond(Second::align(fields))
    }

    const fn from_civil_day(cd: CivilDay) -> Self {
        CivilSecond::from_fields(cd.0)
    }

    const fn year(&self) -> YearType {
        self.0.y
    }

    const fn month(&self) -> i32 {
        self.0.m as i32
    }

    const fn day(&self) -> i32 {
        self.0.d as i32
    }

    const fn hour(&self) -> i32 {
        self.0.hh as i32
    }

    const fn minute(&self) -> i32 {
        self.0.mm as i32
    }

    const fn second(&self) -> i32 {
        self.0.ss as i32
    }
}

// TODO(evenyag): 1. Implement Add<Into<DiffType>> for CivilSecond; 2. Provide a const add/sub function.
impl Add<DiffType> for CivilSecond {
    type Output = Self;

    fn add(self, n: DiffType) -> Self::Output {
        let fields = Second::step(self.0, n);

        Self::from_fields(fields)
    }
}

impl Sub<DiffType> for CivilSecond {
    type Output = Self;

    fn sub(self, n: DiffType) -> Self::Output {
        let fields = if n != DiffType::MIN {
            Second::step(self.0, -n)
        } else {
            Second::step(Second::step(self.0, -(n + 1)), 1)
        };

        Self::from_fields(fields)
    }
}

impl Sub for CivilSecond {
    type Output = DiffType;

    fn sub(self, rhs: Self) -> Self::Output {
        Second::difference(self.0, rhs.0)
    }
}
