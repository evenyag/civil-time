use crate::fields::Fields;
use crate::granularity::{Day, Hour, Minute, Month, Second, Year};
use std::marker::PhantomData;

mod fields;
mod granularity;

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

pub struct CivilYear(Fields);

impl CivilYear {
    pub const fn new(y: YearType) -> Self {
        let fields = Fields::n_sec(y, 1, 1, 0, 0, 0);

        CivilYear(Year::align(fields))
    }
}

pub struct CivilMonth(Fields);

impl CivilMonth {
    pub const fn new(y: YearType, m: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, 1, 0, 0, 0);

        CivilMonth(Month::align(fields))
    }
}

pub struct CivilDay(Fields);

impl CivilDay {
    pub const fn new(y: YearType, m: DiffType, d: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, d, 0, 0, 0);

        CivilDay(Day::align(fields))
    }
}

pub struct CivilHour(Fields);

impl CivilHour {
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, d, hh, 0, 0);

        CivilHour(Hour::align(fields))
    }
}

pub struct CivilMinute(Fields);

impl CivilMinute {
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType, mm: DiffType) -> Self {
        let fields = Fields::n_sec(y, m, d, hh, mm, 0);

        CivilMinute(Minute::align(fields))
    }
}

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
        let fields = Fields::n_sec(y, m, d, hh, mm, ss);

        CivilSecond(Second::align(fields))
    }
}
