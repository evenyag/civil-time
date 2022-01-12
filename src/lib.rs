use std::marker::PhantomData;

mod granularity;
mod normalize;

use granularity::Granularity;

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

/// Normalized civil-time fields: Y-M-D HH:MM:SS.
#[derive(Clone, Copy)]
struct Fields {
    y: YearType,
    m: MonthType,
    d: DayType,
    hh: HourType,
    mm: MinuteType,
    ss: SecondType,
}

struct CivilTime<T> {
    f: Fields,
    tag: PhantomData<T>,
}

impl<T: Granularity> CivilTime<T> {
    fn new(
        y: YearType,
        m: DiffType,
        d: DiffType,
        hh: DiffType,
        mm: DiffType,
        ss: DiffType,
    ) -> CivilTime<T> {
        let fields = normalize::n_sec(y, m, d, hh, mm, ss);

        CivilTime {
            f: T::align(fields),
            tag: PhantomData,
        }
    }
}
