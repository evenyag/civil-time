//! # Civil Time
//!
//! This is a library ported the civil time classes and functions from
//! Google's [cctz](https://github.com/google/cctz) library.
//!
//! The term "civil time" refers to the legally recognized human-scale time
//! that is represented by the six fields YYYY-MM-DD hh:mm:ss. Modern-day civil
//! time follows the Gregorian Calendar and is a time-zone-independent concept.
//! A "date" is perhaps the most common example of a civil time (represented in
//! this library as [`CivilDay`]). This library provides six structs that help
//! with rounding, iterating, and arithmetic on civil times while avoiding complications
//! like daylight-saving time (DST).
//!
//! ## Overview
//!
//! The following six structs form the core of this civil-time library:
//! - [`CivilSecond`]
//! - [`CivilMinute`]
//! - [`CivilHour`]
//! - [`CivilDay`]
//! - [`CivilMonth`]
//! - [`CivilYear`]
//!
//! Each struct is a simple value type with the same six accessors for each of the
//! civil fields (year, month, day, hour, minute, and second, aka YMDHMS). These structs
//! differ in their alignment, which is indicated by the type name and specifies the field on
//! which arithmetic operates.
//!
//! Each struct can be constructed by passing several integer arguments represeting the YMDHMS
//! fields (in that order) according to its alignment (one argument for [`CivilYear::new()`](CivilYear::new)
//! and up to six arguments for [`CivilSecond::new()`](CivilSecond::new)) to the `new()` method.
//! Remaining fields are assigned their minimum valid value. Hours, minutes, and seconds will be set to 0,
//! month and day will be set to 1, and since there is no minimum valid year, it will be set to 1970.
//! A default civil time struct will have YMDHMS fields representing "1970-01-01 00:00:00".
//! Fields that are out-of-range are normalized (e.g., October 32 -> November 1) so that all
//! civil-time structs represent valid values.
//!
//! Each civil-time struct is aligned to the civil-time field indicated in thate
//! struct's name after normalization. Alignment is performed by setting all the
//! inferior fields to their minimum valid value (as described above). The
//! following are examples of how each of the six types would align the fields
//! representing November 22, 2015 at 12:34:56 in the afternoon. (Note: the
//! string format used here is not important; it's just a shorthand way of
//! showing the six YMDHMS fields.)
//!
//! - [CivilSecond]  e.g. 2015-11-22 12:34:56
//! - [CivilMinute]  e.g. 2015-11-22 12:34:00
//! - [CivilHour]    e.g. 2015-11-22 12:00:00
//! - [CivilDay]     e.g. 2015-11-22 00:00:00
//! - [CivilMonth]   e.g. 2015-11-01 00:00:00
//! - [CivilYear]    e.g. 2015-01-01 00:00:00
//!
//! Each civil-time type performs arithmetic on the field to which it is
//! aligned. This means that adding 1 to a [CivilDay] increments the day field
//! (normalizing as necessary), and subtracting 7 from a [CivilMonth] operates
//! on the month field (normalizing as necessary). All arithmetic produces a
//! valid civil time. Difference requires two similarly aligned civil-time
//! structs and returns the scalar answer in units of the structs' alignment.
//! For example, the difference between two [CivilHour] structs will give an
//! answer in units of civil hours.
//!
//! ### Construction
//!
//! Each of the civil-time types can be constructed by `default()` method, or by directly
//! passing to the constructor up to six integers representing the
//! YMDHMS fields, or by [`build()`](`Builder::build`) method of the [`Builder`].
//!
//! ```rust
//! use civil_time::{Builder, CivilDay, CivilHour, CivilMinute, CivilMonth, CivilSecond, CivilYear};
//!
//! // Default value is "1970-01-01 00:00:00"
//! let default_value = CivilDay::default();
//! assert_eq!("1970-01-01", format!("{:?}", default_value));
//!
//! // Constructed by new(), e.g. "2015-02-03 00:00:00"
//! let a = CivilDay::new(2015, 2, 3);
//! assert_eq!("2015-02-03", format!("{:?}", a));
//!
//! // Constructed by Builder, e.g. "2015-01-01 00:00:00"
//! let b = Builder::new().year(2015).build_day();
//! assert_eq!("2015-01-01", format!("{:?}", b));
//!
//! // Constructed by Builder::build(), e.g. "2015-02-03 00:00:00"
//! let c: CivilDay = Builder::new()
//!     .year(2015)
//!     .month(2)
//!     .day(3)
//!     .hour(4)
//!     .minute(5)
//!     .second(6)
//!     .build();
//! assert_eq!("2015-02-03", format!("{:?}", c));
//!
//! // e.g. "2015-02-03 04:05:06"
//! let ss = CivilSecond::new(2015, 2, 3, 4, 5, 6);
//! assert_eq!("2015-02-03T04:05:06", format!("{:?}", ss));
//!
//! // e.g. "2015-02-03 04:05:00"
//! let mm = CivilMinute::new(2015, 2, 3, 4, 5);
//! assert_eq!("2015-02-03T04:05", format!("{:?}", mm));
//!
//! // e.g. "2015-02-03 04:00:00"
//! let hh = CivilHour::new(2015, 2, 3, 4);
//! assert_eq!("2015-02-03T04", format!("{:?}", hh));
//!
//! // e.g. "2015-02-03 00:00:00"
//! let d = CivilDay::new(2015, 2, 3);
//! assert_eq!("2015-02-03", format!("{:?}", d));
//!
//! // e.g. "2015-02-01 00:00:00"
//! let m = CivilMonth::new(2015, 2);
//! assert_eq!("2015-02", format!("{:?}", m));
//!
//! // e.g. "2015-01-01 00:00:00"
//! let y = CivilYear::new(2015);
//! assert_eq!("2015", format!("{:?}", y));
//! ```
//!
//! ### Conversion
//!
//! The alignment of a civil-time struct cannot change, but the struct may be
//! used to construct a new struct with a different alignment. This is referred
//! to as "realigning" and can be done by the `from()` method.
//!
//! ```rust
//! use civil_time::{CivilDay, CivilHour, CivilMinute, CivilMonth, CivilSecond, CivilYear};
//!
//! // e.g. "2015-02-03T04:05:06"
//! let ss = CivilSecond::new(2015, 2, 3, 4, 5, 6);
//! assert_eq!("2015-02-03T04:05:06", format!("{:?}", ss));
//! // e.g. "2015-02-03 04:05:00"
//! let mm = CivilMinute::from(ss);
//! assert_eq!("2015-02-03T04:05", format!("{:?}", mm));
//! // e.g. "2015-02-03 04:00:00"
//! let hh = CivilHour::from(mm);
//! assert_eq!("2015-02-03T04", format!("{:?}", hh));
//! // e.g. "2015-02-03 00:00:00"
//! let d = CivilDay::from(hh);
//! assert_eq!("2015-02-03", format!("{:?}", d));
//! // e.g. "2015-02-01 00:00:00"
//! let m = CivilMonth::from(d);
//! assert_eq!("2015-02", format!("{:?}", m));
//! // e.g. "2015-01-01 00:00:00"
//! let y = CivilYear::from(m);
//! assert_eq!("2015", format!("{:?}", y));
//! ```
//!
//! ### Normalization
//! Integer arguments passed to the `new()` may be out-of-range, in which
//! case they are normalized to produce a valid civil-time struct. This enables
//! natural arithmetic on `new()` arguments without worrying about the
//! field's range. Normalization guarantees that there are no invalid
//! civil-time structs.
//!
//! ```rust
//! use civil_time::CivilDay;
//!
//! // Out-of-range day; normalized to 2016-11-01
//! let d = CivilDay::new(2016, 10, 32);
//! assert_eq!("2016-11-01", format!("{:?}", d));
//! ```
//!
//! Note: If normalization is undesired, you can signal an error by comparing
//! the `new()` arguments to the normalized values returned by the YMDHMS
//! properties.
//!
//! ### Properties
//! All civil-time types have accessors for all six of the civil-time fields:
//! year, month, day, hour, minute, and second. Recall that fields inferior to
//! the type's alignment will be set to their minimum valid value.
//!
//! ```rust
//! use civil_time::CivilDay;
//!
//! let d = CivilDay::new(2015, 6, 28);
//! assert_eq!(2015, d.year());
//! assert_eq!(6, d.month());
//! assert_eq!(28, d.day());
//! assert_eq!(0, d.hour());
//! assert_eq!(0, d.minute());
//! assert_eq!(0, d.second());
//! ```
//!
//! ### Comparision
//!
//! Comparison always considers all six YMDHMS fields, regardless of the type's
//! alignment. Comparison between differently aligned civil-time types is
//! allowed.
//!
//! ```rust
//! use civil_time::{CivilDay, CivilSecond, CivilYear};
//!
//! let feb_3 = CivilDay::new(2015, 2, 3); // 2015-02-03 00:00:00
//! let mar_4 = CivilDay::new(2015, 3, 4); // 2015-03-04 00:00:00
//! assert!(feb_3 < mar_4);
//! assert_eq!(CivilYear::from(feb_3), CivilYear::from(mar_4));
//!
//! let feb_3_noon = CivilSecond::new(2015, 2, 3, 12, 0, 0); // 2015-02-03 12:00:00
//! assert!(feb_3 < feb_3_noon);
//! assert_eq!(feb_3, CivilDay::from(feb_3_noon));
//! ```
//!
//! ### Arithmetic
//!
//! Civil-time types support natural arithmetic operators such as addition,
//! subtraction, and difference. Arithmetic operates on the civil-time field
//! indicated in the type's name. Difference requires arguments with the same
//! alignment and returns the answer in units of the alignment.
//!
//! ```rust
//! use civil_time::{CivilDay, CivilMonth};
//!
//! let mut a = CivilDay::new(2015, 2, 3);
//! a += 1;
//! assert_eq!("2015-02-04", format!("{:?}", a));
//! a -= 1;
//! assert_eq!("2015-02-03", format!("{:?}", a));
//!
//! let b = a + 1;
//! assert_eq!("2015-02-04", format!("{:?}", b));
//! let c = b + 1;
//! assert_eq!("2015-02-05", format!("{:?}", c));
//!
//! let n = c - a;
//! assert_eq!(2, n);
//! // let m = c - CivilMonth::from(c); // Won't compile: different types.
//! ```
//!
//! ### Example: Adding a month to January 31.
//!
//! One of the classic questions that arises when considering a civil-time
//! library (or a date library or a date/time library) is this: "What happens
//! when you add a month to January 31?" This is an interesting question
//! because there could be a number of possible answers:
//!
//! 1. March 3 (or 2 if a leap year). This may make sense if the operation
//!    wants the equivalent of February 31.
//! 2. February 28 (or 29 if a leap year). This may make sense if the operation
//!    wants the last day of January to go to the last day of February.
//! 3. Error. The caller may get some error, an exception, an invalid date
//!    object, or maybe false is returned. This may make sense because there is
//!    no single unambiguously correct answer to the question.
//!
//! Practically speaking, any answer that is not what the programmer intended
//! is the wrong answer.
//!
//! This civil-time library avoids the problem by making it impossible to ask
//! ambiguous questions. All civil-time structs are aligned to a particular
//! civil-field boundary (such as aligned to a year, month, day, hour, minute,
//! or second), and arithmetic operates on the field to which the struct is
//! aligned. This means that in order to "add a month" the struct must first be
//! aligned to a month boundary, which is equivalent to the first day of that
//! month.
//!
//! Of course, there are ways to compute an answer the question at hand using
//! this civil-time library, but they require the programmer to be explicit
//! about the answer they expect. To illustrate, let's see how to compute all
//! three of the above possible answers to the question of "Jan 31 plus 1
//! month":
//!
//! ```rust
//! use civil_time::{CivilDay, CivilMonth};
//!
//! let d = CivilDay::new(2015, 1, 31);
//!
//! // Answer 1:
//! // Add 1 to the month field in the `new()`, and rely on normalization.
//! let ans_normalized = CivilDay::new(d.year(), (d.month() + 1).into(), d.day().into());
//! assert_eq!("2015-03-03", format!("{:?}", ans_normalized));
//!
//! // Answer 2:
//! // Add 1 to month field, capping to the end of next month.
//! let next_month = CivilMonth::from(d) + 1;
//! let last_day_of_next_month = CivilDay::from(next_month + 1) - 1;
//! let ans_capped = std::cmp::min(ans_normalized, last_day_of_next_month);
//! assert_eq!("2015-02-28", format!("{:?}", ans_capped));
//!
//! // Answer 3:
//! // Signal an error if the normalized answer is not in next month.
//! if CivilMonth::from(ans_normalized) != next_month {
//!     // error, month overflow
//! }
//! ```
//!
//! ### Weekday
//!
//! Returns the weekday for the given civil-time value.
//!
//! ```rust
//! use civil_time::{CivilDay, Weekday};
//!
//! let a = CivilDay::new(2015, 8, 13);
//! assert_eq!(Weekday::Thu, a.weekday());
//! ```
//!
//! Returns the [CivilDay] that strictly follows or precedes the given
//! [CivilDay], and that falls on the given weekday.
//!
//! For example, given:
//!
//! ```text
//!     August 2015
//! Su Mo Tu We Th Fr Sa
//!                    1
//!  2  3  4  5  6  7  8
//!  9 10 11 12 13 14 15
//! 16 17 18 19 20 21 22
//! 23 24 25 26 27 28 29
//! 30 31
//! ```
//!
//! ```rust
//! use civil_time::{CivilDay, Weekday};
//!
//! let a = CivilDay::new(2015, 8, 13);
//! assert_eq!(Weekday::Thu, a.weekday());
//! let b = a.next_weekday(Weekday::Thu);
//! assert_eq!("2015-08-20", format!("{:?}", b));
//! let c = a.prev_weekday(Weekday::Thu);
//! assert_eq!("2015-08-06", format!("{:?}", c));
//!
//! let d = a - 1;
//! // Gets the following Thursday if d is not already Thursday
//! let thurs1 = d.next_weekday(Weekday::Thu);
//! assert_eq!("2015-08-13", format!("{:?}", thurs1));
//! //  Gets the previous Thursday if d is not already Thursday
//! let d = a + 1;
//! let thurs2 = d.prev_weekday(Weekday::Thu);
//! assert_eq!("2015-08-13", format!("{:?}", thurs2));
//! ```
//!
//! ### Yearday
//!
//! Returns the day-of-year for the given civil-time value.
//!
//! ```rust
//! use civil_time::CivilDay;
//!
//! let a = CivilDay::new(2015, 1, 1);
//! assert_eq!(1, a.yearday());
//! let b = CivilDay::new(2015, 12, 31);
//! assert_eq!(365, b.yearday());
//! ```

use crate::alignment::{Day, Hour, Minute, Month, Second, Year};
use crate::core::Fields;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

mod alignment;
mod compare;
mod convert;
mod core;
mod weekday;

pub use crate::core::{DiffType, YearType};
pub use crate::weekday::Weekday;

/// Helper trait to construct a civil time type.
pub trait BuildCivilTime {
    /// Build civil time types by given year `y`, month `m`, day `d`,
    /// hour `hh`, minute `mm` and second `ss`.
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
    ($Type: ident, $Alignment: ident) => {
        impl $Type {
            /// Maximum representable civil time.
            pub const MAX: $Type = $Type::from_ymd_hms(DiffType::MAX, 12, 31, 23, 59, 59);
            /// Minimum representable civil time.
            pub const MIN: $Type = $Type::from_ymd_hms(DiffType::MIN, 1, 1, 0, 0, 0);

            /// Get the year of the given civil-time value.
            pub const fn year(&self) -> YearType {
                self.0.y
            }

            /// Get the month of the given civil-time value.
            ///
            /// The return value ranges from 1 to 12.
            pub const fn month(&self) -> i32 {
                self.0.m as i32
            }

            /// Get the day of the given civil-time value.
            ///
            /// The return value ranges from 1 to 31.
            pub const fn day(&self) -> i32 {
                self.0.d as i32
            }

            /// Get the hour of the given civil-time value.
            ///
            /// The return value ranges from 0 to 23.
            pub const fn hour(&self) -> i32 {
                self.0.hh as i32
            }

            /// Get the minute of the given civil-time value.
            ///
            /// The return value ranges from 0 to 59.
            pub const fn minute(&self) -> i32 {
                self.0.mm as i32
            }

            /// Get the second of the given civil-time value.
            ///
            /// The return value ranges from 0 to 59.
            pub const fn second(&self) -> i32 {
                self.0.ss as i32
            }

            /// Returns the weekday for the given civil-time value.
            pub const fn weekday(&self) -> Weekday {
                Weekday::from_second(CivilSecond::from_fields(self.0))
            }

            /// Returns the day-of-year for the given civil-time value.
            ///
            /// The return value ranges from 1 to 366.
            pub const fn yearday(&self) -> i32 {
                get_yearday(CivilSecond::from_fields(self.0))
            }

            const fn from_fields(fields: Fields) -> Self {
                $Type($Alignment::align(fields))
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

            const fn add_diff(self, n: DiffType) -> Self {
                let fields = $Alignment::step(self.0, n);

                Self::from_fields(fields)
            }

            const fn sub_diff(self, n: DiffType) -> Self {
                let fields = if n != DiffType::MIN {
                    $Alignment::step(self.0, -n)
                } else {
                    $Alignment::step($Alignment::step(self.0, -(n + 1)), 1)
                };

                Self::from_fields(fields)
            }

            const fn difference(self, other: Self) -> DiffType {
                $Alignment::difference(self.0, other.0)
            }
        }

        impl Add<DiffType> for $Type {
            type Output = Self;

            fn add(self, n: DiffType) -> Self::Output {
                self.add_diff(n)
            }
        }

        impl AddAssign<DiffType> for $Type {
            fn add_assign(&mut self, n: DiffType) {
                *self = self.add_diff(n);
            }
        }

        impl Sub<DiffType> for $Type {
            type Output = Self;

            fn sub(self, n: DiffType) -> Self::Output {
                self.sub_diff(n)
            }
        }

        impl SubAssign<DiffType> for $Type {
            fn sub_assign(&mut self, n: DiffType) {
                *self = self.sub_diff(n);
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

/// Civil time in second alignment.
#[derive(Clone, Copy)]
pub struct CivilSecond(Fields);

impl CivilSecond {
    /// Construct a [CivilSecond] instance by given year `y`, month `m`, day `d`,
    /// hour `hh`, minute `mm` and second `ss`.
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

impl fmt::Debug for CivilSecond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second()
        )
    }
}

/// Civil time in minute alignment.
#[derive(Clone, Copy)]
pub struct CivilMinute(Fields);

impl CivilMinute {
    /// Construct a [CivilMinute] instance by given year `y`, month `m`, day `d`,
    /// hour `hh`, minute `mm`.
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType, mm: DiffType) -> Self {
        Self::from_ymd_hms(y, m, d, hh, mm, 0)
    }
}

impl fmt::Debug for CivilMinute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{:0>2}-{:0>2}T{:0>2}:{:0>2}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute()
        )
    }
}

/// Civil time in hour alignment.
#[derive(Clone, Copy)]
pub struct CivilHour(Fields);

impl CivilHour {
    /// Construct a [CivilHour] instance by given year `y`, month `m`, day `d`,
    /// hour `hh`.
    pub const fn new(y: YearType, m: DiffType, d: DiffType, hh: DiffType) -> Self {
        Self::from_ymd_hms(y, m, d, hh, 0, 0)
    }
}

impl fmt::Debug for CivilHour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{:0>2}-{:0>2}T{:0>2}",
            self.year(),
            self.month(),
            self.day(),
            self.hour()
        )
    }
}

/// Civil time in day alignment.
#[derive(Clone, Copy)]
pub struct CivilDay(Fields);

impl CivilDay {
    /// Construct a [CivilDay] instance by given year `y`, month `m`, day `d`.
    pub const fn new(y: YearType, m: DiffType, d: DiffType) -> Self {
        Self::from_ymd_hms(y, m, d, 0, 0, 0)
    }
}

impl fmt::Debug for CivilDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:0>2}-{:0>2}", self.year(), self.month(), self.day())
    }
}

/// Civil time in month alignment.
#[derive(Clone, Copy)]
pub struct CivilMonth(Fields);

impl CivilMonth {
    /// Construct a [CivilMonth] instance by given year `y`, month `m`.
    pub const fn new(y: YearType, m: DiffType) -> Self {
        Self::from_ymd_hms(y, m, 1, 0, 0, 0)
    }
}

impl fmt::Debug for CivilMonth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:0>2}", self.year(), self.month())
    }
}

/// Civil time in year alignment.
#[derive(Clone, Copy)]
pub struct CivilYear(Fields);

impl CivilYear {
    /// Construct a [CivilYear] instance by given year `y`.
    pub const fn new(y: YearType) -> Self {
        Self::from_ymd_hms(y, 1, 1, 0, 0, 0)
    }
}

impl fmt::Debug for CivilYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.year())
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

/// A builder to build civil time instances.
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
    /// Create a new [Builder] with default value.
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

    /// Set the year field of the civil time instance.
    pub const fn year(mut self, y: YearType) -> Self {
        self.y = y;
        self
    }

    /// Set the month field of the civil time instance.
    pub const fn month(mut self, m: DiffType) -> Self {
        self.m = m;
        self
    }

    /// Set the day field of the civil time instance.
    pub const fn day(mut self, d: DiffType) -> Self {
        self.d = d;
        self
    }

    /// Set the hour field of the civil time instance.
    pub const fn hour(mut self, hour: DiffType) -> Self {
        self.hh = hour;
        self
    }

    /// Set the minute field of the civil time instance.
    pub const fn minute(mut self, minute: DiffType) -> Self {
        self.mm = minute;
        self
    }

    /// Set the second field of the civil time instance.
    pub const fn second(mut self, second: DiffType) -> Self {
        self.ss = second;
        self
    }

    /// Build a new civil time instance.
    ///
    /// ```rust
    /// use civil_time::{Builder, CivilDay};
    ///
    /// let d: CivilDay = Builder::new().build();
    /// assert_eq!("1970-01-01", format!("{:?}", d));
    /// ```
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
            /// Build civil time type of specific alignment.
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

// TODO(evenyag): Port benchmarks.
#[cfg(test)]
pub mod tests {
    use super::*;
    use static_assertions as sa;

    pub fn expect_eq<T: fmt::Debug>(expect: &str, given: T) {
        assert_eq!(expect, format!("{:?}", given));
    }

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

    #[test]
    fn test_default() {
        let ss = CivilSecond::default();
        expect_eq("1970-01-01T00:00:00", ss);

        let mm = CivilMinute::default();
        expect_eq("1970-01-01T00:00", mm);

        let hh = CivilHour::default();
        expect_eq("1970-01-01T00", hh);

        let d = CivilDay::default();
        expect_eq("1970-01-01", d);

        let m = CivilMonth::default();
        expect_eq("1970-01", m);

        let y = CivilYear::default();
        expect_eq("1970", y);
    }

    #[test]
    fn test_builder_build_second() {
        expect_eq(
            "2015-01-02T03:04:05",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .second(5)
                .build_second(),
        );
        expect_eq(
            "2015-01-02T03:04:00",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .build_second(),
        );
        expect_eq(
            "2015-01-02T03:00:00",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .build_second(),
        );
        expect_eq(
            "2015-01-02T00:00:00",
            Builder::new().year(2015).month(1).day(2).build_second(),
        );
        expect_eq(
            "2015-01-01T00:00:00",
            Builder::new().year(2015).month(1).build_second(),
        );
        expect_eq(
            "2015-01-01T00:00:00",
            Builder::new().year(2015).build_second(),
        );
    }

    #[test]
    fn test_builder_build_minute() {
        expect_eq(
            "2015-01-02T03:04",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .second(5)
                .build_minute(),
        );
        expect_eq(
            "2015-01-02T03:04",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .build_minute(),
        );
        expect_eq(
            "2015-01-02T03:00",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .build_minute(),
        );
        expect_eq(
            "2015-01-02T00:00",
            Builder::new().year(2015).month(1).day(2).build_minute(),
        );
        expect_eq(
            "2015-01-01T00:00",
            Builder::new().year(2015).month(1).build_minute(),
        );
        expect_eq("2015-01-01T00:00", Builder::new().year(2015).build_minute());
    }

    #[test]
    fn test_builder_build_hour() {
        expect_eq(
            "2015-01-02T03",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .second(5)
                .build_hour(),
        );
        expect_eq(
            "2015-01-02T03",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .build_hour(),
        );
        expect_eq(
            "2015-01-02T03",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .build_hour(),
        );
        expect_eq(
            "2015-01-02T00",
            Builder::new().year(2015).month(1).day(2).build_hour(),
        );
        expect_eq(
            "2015-01-01T00",
            Builder::new().year(2015).month(1).build_hour(),
        );
        expect_eq("2015-01-01T00", Builder::new().year(2015).build_hour());
    }

    #[test]
    fn test_builder_build_day() {
        expect_eq(
            "2015-01-02",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .second(5)
                .build_day(),
        );
        expect_eq(
            "2015-01-02",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .build_day(),
        );
        expect_eq(
            "2015-01-02",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .build_day(),
        );
        expect_eq(
            "2015-01-02",
            Builder::new().year(2015).month(1).day(2).build_day(),
        );
        expect_eq("2015-01-01", Builder::new().year(2015).month(1).build_day());
        expect_eq("2015-01-01", Builder::new().year(2015).build_day());
    }

    #[test]
    fn test_builder_build_month() {
        expect_eq(
            "2015-01",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .second(5)
                .build_month(),
        );
        expect_eq(
            "2015-01",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .build_month(),
        );
        expect_eq(
            "2015-01",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .build_month(),
        );
        expect_eq(
            "2015-01",
            Builder::new().year(2015).month(1).day(2).build_month(),
        );
        expect_eq("2015-01", Builder::new().year(2015).month(1).build_month());
        expect_eq("2015-01", Builder::new().year(2015).build_month());
    }

    #[test]
    fn test_builder_build_year() {
        expect_eq(
            "2015",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .second(5)
                .build_year(),
        );
        expect_eq(
            "2015",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .minute(4)
                .build_year(),
        );
        expect_eq(
            "2015",
            Builder::new()
                .year(2015)
                .month(1)
                .day(2)
                .hour(3)
                .build_year(),
        );
        expect_eq(
            "2015",
            Builder::new().year(2015).month(1).day(2).build_year(),
        );
        expect_eq("2015", Builder::new().year(2015).month(1).build_year());
        expect_eq("2015", Builder::new().year(2015).build_year());
    }

    #[test]
    fn test_builder_build_by_hint() {
        let builder = Builder::new()
            .year(2015)
            .month(1)
            .day(2)
            .hour(3)
            .minute(4)
            .second(5);

        let ss: CivilSecond = builder.build();
        expect_eq("2015-01-02T03:04:05", ss);

        let mm: CivilMinute = builder.build();
        expect_eq("2015-01-02T03:04", mm);

        let hh: CivilHour = builder.build();
        expect_eq("2015-01-02T03", hh);

        let d: CivilDay = builder.build();
        expect_eq("2015-01-02", d);

        let m: CivilMonth = builder.build();
        expect_eq("2015-01", m);

        let y: CivilYear = builder.build();
        expect_eq("2015", y);
    }

    #[test]
    fn test_new_limits() {
        let max = i64::from(i32::MAX);
        expect_eq(
            "2038-01-19T03:14:07",
            CivilSecond::new(1970, 1, 1, 0, 0, max),
        );
        expect_eq(
            "6121-02-11T05:21:07",
            CivilSecond::new(1970, 1, 1, 0, max, max),
        );
        expect_eq(
            "251104-11-20T12:21:07",
            CivilSecond::new(1970, 1, 1, max, max, max),
        );
        expect_eq(
            "6130715-05-30T12:21:07",
            CivilSecond::new(1970, 1, max, max, max, max),
        );
        expect_eq(
            "185087685-11-26T12:21:07",
            CivilSecond::new(1970, max, max, max, max, max),
        );

        let min = i64::from(i32::MIN);
        expect_eq(
            "1901-12-13T20:45:52",
            CivilSecond::new(1970, 1, 1, 0, 0, min),
        );
        expect_eq(
            "-2182-11-20T18:37:52",
            CivilSecond::new(1970, 1, 1, 0, min, min),
        );
        expect_eq(
            "-247165-02-11T10:37:52",
            CivilSecond::new(1970, 1, 1, min, min, min),
        );
        expect_eq(
            "-6126776-08-01T10:37:52",
            CivilSecond::new(1970, 1, min, min, min, min),
        );
        expect_eq(
            "-185083747-10-31T10:37:52",
            CivilSecond::new(1970, min, min, min, min, min),
        );
    }

    #[test]
    fn test_arithmetic() {
        let mut second = CivilSecond::new(2015, 1, 2, 3, 4, 5);
        second += 1;
        expect_eq("2015-01-02T03:04:06", second);
        expect_eq("2015-01-02T03:04:07", second + 1);
        expect_eq("2015-01-02T03:04:05", second - 1);
        second -= 1;
        expect_eq("2015-01-02T03:04:05", second);

        let mut minute = CivilMinute::new(2015, 1, 2, 3, 4);
        minute += 1;
        expect_eq("2015-01-02T03:05", minute);
        expect_eq("2015-01-02T03:06", minute + 1);
        expect_eq("2015-01-02T03:04", minute - 1);
        minute -= 1;
        expect_eq("2015-01-02T03:04", minute);

        let mut hour = CivilHour::new(2015, 1, 2, 3);
        hour += 1;
        expect_eq("2015-01-02T04", hour);
        expect_eq("2015-01-02T05", hour + 1);
        expect_eq("2015-01-02T03", hour - 1);
        hour -= 1;
        expect_eq("2015-01-02T03", hour);

        let mut day = CivilDay::new(2015, 1, 2);
        day += 1;
        expect_eq("2015-01-03", day);
        expect_eq("2015-01-04", day + 1);
        expect_eq("2015-01-02", day - 1);
        day -= 1;
        expect_eq("2015-01-02", day);

        let mut month = CivilMonth::new(2015, 1);
        month += 1;
        expect_eq("2015-02", month);
        expect_eq("2015-03", month + 1);
        expect_eq("2015-01", month - 1);
        month -= 1;
        expect_eq("2015-01", month);

        let mut year = CivilYear::new(2015);
        year += 1;
        expect_eq("2016", year);
        expect_eq("2017", year + 1);
        expect_eq("2015", year - 1);
        year -= 1;
        expect_eq("2015", year);
    }

    #[test]
    fn test_arithmetic_limits() {
        let (max, min) = (i64::from(i32::MAX), i64::from(i32::MIN));

        let mut second = CivilSecond::new(1970, 1, 1, 0, 0, 0);
        second += max;
        expect_eq("2038-01-19T03:14:07", second);
        second -= max;
        expect_eq("1970-01-01T00:00:00", second);
        second += min;
        expect_eq("1901-12-13T20:45:52", second);
        second -= min;
        expect_eq("1970-01-01T00:00:00", second);

        let mut minute = CivilMinute::new(1970, 1, 1, 0, 0);
        minute += max;
        expect_eq("6053-01-23T02:07", minute);
        minute -= max;
        expect_eq("1970-01-01T00:00", minute);
        minute += min;
        expect_eq("-2114-12-08T21:52", minute);
        minute -= min;
        expect_eq("1970-01-01T00:00", minute);

        let mut hour = CivilHour::new(1970, 1, 1, 0);
        hour += max;
        expect_eq("246953-10-09T07", hour);
        hour -= max;
        expect_eq("1970-01-01T00", hour);
        hour += min;
        expect_eq("-243014-03-24T16", hour);
        hour -= min;
        expect_eq("1970-01-01T00", hour);

        let mut day = CivilDay::new(1970, 1, 1);
        day += max;
        expect_eq("5881580-07-11", day);
        day -= max;
        expect_eq("1970-01-01", day);
        day += min;
        expect_eq("-5877641-06-23", day);
        day -= min;
        expect_eq("1970-01-01", day);

        let mut month = CivilMonth::new(1970, 1);
        month += max;
        expect_eq("178958940-08", month);
        month -= max;
        expect_eq("1970-01", month);
        month += min;
        expect_eq("-178955001-05", month);
        month -= min;
        expect_eq("1970-01", month);

        let mut year = CivilYear::new(0);
        year += max;
        expect_eq("2147483647", year);
        year -= max;
        expect_eq("0", year);
        year += min;
        expect_eq("-2147483648", year);
        year -= min;
        expect_eq("0", year);
    }

    #[test]
    fn test_arithmetic_difference() {
        let second = CivilSecond::new(2015, 1, 2, 3, 4, 5);
        assert_eq!(0, second - second);
        assert_eq!(10, (second + 10) - second);
        assert_eq!(-10, (second - 10) - second);

        let minute = CivilMinute::new(2015, 1, 2, 3, 4);
        assert_eq!(0, minute - minute);
        assert_eq!(10, (minute + 10) - minute);
        assert_eq!(-10, (minute - 10) - minute);

        let hour = CivilHour::new(2015, 1, 2, 3);
        assert_eq!(0, hour - hour);
        assert_eq!(10, (hour + 10) - hour);
        assert_eq!(-10, (hour - 10) - hour);

        let day = CivilDay::new(2015, 1, 2);
        assert_eq!(0, day - day);
        assert_eq!(10, (day + 10) - day);
        assert_eq!(-10, (day - 10) - day);

        let month = CivilMonth::new(2015, 1);
        assert_eq!(0, month - month);
        assert_eq!(10, (month + 10) - month);
        assert_eq!(-10, (month - 10) - month);

        let year = CivilYear::new(2015);
        assert_eq!(0, year - year);
        assert_eq!(10, (year + 10) - year);
        assert_eq!(-10, (year - 10) - year);
    }

    #[test]
    fn test_difference_limits() {
        let (max, min) = (i64::MAX, i64::MIN);

        // Check day arithmetic at the end of the year range.
        let max_day = CivilDay::new(max, 12, 31);
        assert_eq!(1, max_day - (max_day - 1));
        assert_eq!(-1, (max_day - 1) - max_day);

        // Check day arithmetic at the end of the year range.
        let min_day = CivilDay::new(min, 1, 1);
        assert_eq!(1, (min_day + 1) - min_day);
        assert_eq!(-1, min_day - (min_day + 1));

        let (d1, d2) = (CivilDay::new(1970, 1, 1), CivilDay::new(5881580, 7, 11));
        assert_eq!(i64::from(i32::MAX), d2 - d1);
        assert_eq!(i64::from(i32::MIN), d1 - (d2 + 1));
    }

    #[test]
    fn test_properties() {
        let ss = CivilSecond::new(2015, 2, 3, 4, 5, 6);
        assert_eq!(2015, ss.year());
        assert_eq!(2, ss.month());
        assert_eq!(3, ss.day());
        assert_eq!(4, ss.hour());
        assert_eq!(5, ss.minute());
        assert_eq!(6, ss.second());
        assert_eq!(Weekday::Tue, ss.weekday());
        assert_eq!(34, ss.yearday());

        let mm = Builder::new()
            .year(2015)
            .month(2)
            .day(3)
            .hour(4)
            .minute(5)
            .second(6)
            .build_minute();
        assert_eq!(2015, mm.year());
        assert_eq!(2, mm.month());
        assert_eq!(3, mm.day());
        assert_eq!(4, mm.hour());
        assert_eq!(5, mm.minute());
        assert_eq!(0, mm.second());
        assert_eq!(Weekday::Tue, mm.weekday());
        assert_eq!(34, mm.yearday());

        let hh = Builder::new()
            .year(2015)
            .month(2)
            .day(3)
            .hour(4)
            .minute(5)
            .second(6)
            .build_hour();
        assert_eq!(2015, hh.year());
        assert_eq!(2, hh.month());
        assert_eq!(3, hh.day());
        assert_eq!(4, hh.hour());
        assert_eq!(0, hh.minute());
        assert_eq!(0, hh.second());
        assert_eq!(Weekday::Tue, hh.weekday());
        assert_eq!(34, hh.yearday());

        let d = Builder::new()
            .year(2015)
            .month(2)
            .day(3)
            .hour(4)
            .minute(5)
            .second(6)
            .build_day();
        assert_eq!(2015, d.year());
        assert_eq!(2, d.month());
        assert_eq!(3, d.day());
        assert_eq!(0, d.hour());
        assert_eq!(0, d.minute());
        assert_eq!(0, d.second());
        assert_eq!(Weekday::Tue, d.weekday());
        assert_eq!(34, d.yearday());

        let m = Builder::new()
            .year(2015)
            .month(2)
            .day(3)
            .hour(4)
            .minute(5)
            .second(6)
            .build_month();
        assert_eq!(2015, m.year());
        assert_eq!(2, m.month());
        assert_eq!(1, m.day());
        assert_eq!(0, m.hour());
        assert_eq!(0, m.minute());
        assert_eq!(0, m.second());
        assert_eq!(Weekday::Sun, m.weekday());
        assert_eq!(32, m.yearday());

        let y = Builder::new()
            .year(2015)
            .month(2)
            .day(3)
            .hour(4)
            .minute(5)
            .second(6)
            .build_year();
        assert_eq!(2015, y.year());
        assert_eq!(1, y.month());
        assert_eq!(1, y.day());
        assert_eq!(0, y.hour());
        assert_eq!(0, y.minute());
        assert_eq!(0, y.second());
        assert_eq!(Weekday::Thu, y.weekday());
        assert_eq!(1, y.yearday());
    }

    #[test]
    fn test_debug_format() {
        // Tests formatting of CivilYear, which does not pad.
        expect_eq("2016", CivilYear::new(2016));
        expect_eq("123", CivilYear::new(123));
        expect_eq("0", CivilYear::new(0));
        expect_eq("-1", CivilYear::new(-1));

        // Tests formatting of sub-year types, which pad to 2 digits.
        expect_eq("2016-02", CivilMonth::new(2016, 2));
        expect_eq("2016-02-03", CivilDay::new(2016, 2, 3));
        expect_eq("2016-02-03T04", CivilHour::new(2016, 2, 3, 4));
        expect_eq("2016-02-03T04:05", CivilMinute::new(2016, 2, 3, 4, 5));
        expect_eq("2016-02-03T04:05:06", CivilSecond::new(2016, 2, 3, 4, 5, 6));
    }

    #[test]
    fn test_normalize_with_huge_year() {
        let c = CivilMonth::new(9223372036854775807, 1);
        expect_eq("9223372036854775807-01", c);
        // Causes normalization
        let c = c - 1;
        expect_eq("9223372036854775806-12", c);

        let c = CivilMonth::new(-9223372036854775807 - 1, 1);
        expect_eq("-9223372036854775808-01", c);
        // Causes normalization
        let c = c + 12;
        expect_eq("-9223372036854775807-01", c);
    }

    #[test]
    fn test_leap_years() {
        // Test data for leap years.
        let leap_year_table = [
            // (year, days, month and day after Feb 28)
            (1900, 365, 3, 1),
            (1999, 365, 3, 1),
            (2000, 366, 2, 29), // leap year
            (2001, 365, 3, 1),
            (2002, 365, 3, 1),
            (2003, 365, 3, 1),
            (2004, 366, 2, 29), // leap year
            (2005, 365, 3, 1),
            (2006, 365, 3, 1),
            (2007, 365, 3, 1),
            (2008, 366, 2, 29), // leap year
            (2009, 365, 3, 1),
            (2100, 365, 3, 1),
        ];

        for e in leap_year_table {
            // Tests incrementing through the leap day.
            let feb28 = CivilDay::new(e.0, 2, 28);
            let next_day = feb28 + 1;
            assert_eq!(e.2, next_day.month());
            assert_eq!(e.3, next_day.day());

            // Tests difference in days of leap years.
            let year = CivilYear::from(feb28);
            let next_year = year + 1;
            assert_eq!(e.1, CivilDay::from(next_year) - CivilDay::from(year));
        }
    }
}
