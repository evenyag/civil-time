use crate::fields::Fields;
use crate::{DayType, DiffType, MonthType, YearType};

// TODO(evenyag): Use trait once rust supports declaring const functions in trait.
// See issue #57563 <https://github.com/rust-lang/rust/issues/57563>

/// Second granularity.
pub(crate) struct Second;

impl Second {
    /// Increments the indicated (normalized) field by "n".
    pub(crate) const fn step(f: Fields, n: DiffType) -> Fields {
        Fields::n_sec(
            f.y,
            f.m as DiffType,
            f.d as DiffType,
            f.hh as DiffType,
            f.mm as DiffType + n / 60,
            f.ss as DiffType + n % 60,
        )
    }

    /// Returns the difference between fields structs using the indicated unit.
    pub(crate) const fn difference(f1: Fields, f2: Fields) -> DiffType {
        scale_add(Minute::difference(f1, f2), 60, (f1.ss - f2.ss) as DiffType)
    }

    /// Aligns the (normalized) fields struct to the indicated field.
    pub(crate) const fn align(f: Fields) -> Fields {
        f
    }
}

/// Minute granularity.
pub(crate) struct Minute;

impl Minute {
    /// Increments the indicated (normalized) field by "n".
    pub(crate) const fn step(f: Fields, n: DiffType) -> Fields {
        Fields::n_min(
            f.y,
            f.m as DiffType,
            f.d as DiffType,
            f.hh as DiffType + n / 60,
            0,
            f.mm as DiffType + n % 60,
            f.ss,
        )
    }

    /// Returns the difference between fields structs using the indicated unit.
    pub(crate) const fn difference(f1: Fields, f2: Fields) -> DiffType {
        scale_add(Hour::difference(f1, f2), 60, (f1.mm - f2.mm) as DiffType)
    }

    /// Aligns the (normalized) fields struct to the indicated field.
    pub(crate) const fn align(f: Fields) -> Fields {
        Fields {
            y: f.y,
            m: f.m,
            d: f.d,
            hh: f.hh,
            mm: f.mm,
            ss: 0,
        }
    }
}

/// Hour granularity.
pub(crate) struct Hour;

impl Hour {
    /// Increments the indicated (normalized) field by "n".
    pub(crate) const fn step(f: Fields, n: DiffType) -> Fields {
        Fields::n_hour(
            f.y,
            f.m as DiffType,
            f.d as DiffType + n / 24,
            0,
            f.hh as DiffType + n % 24,
            f.mm,
            f.ss,
        )
    }

    /// Returns the difference between fields structs using the indicated unit.
    pub(crate) const fn difference(f1: Fields, f2: Fields) -> DiffType {
        scale_add(Day::difference(f1, f2), 24, (f1.hh - f2.hh) as DiffType)
    }

    /// Aligns the (normalized) fields struct to the indicated field.
    pub(crate) const fn align(f: Fields) -> Fields {
        Fields {
            y: f.y,
            m: f.m,
            d: f.d,
            hh: f.hh,
            mm: 0,
            ss: 0,
        }
    }
}

/// Day granularity.
pub(crate) struct Day;

impl Day {
    /// Increments the indicated (normalized) field by "n".
    pub(crate) const fn step(f: Fields, n: DiffType) -> Fields {
        Fields::n_day(f.y, f.m, f.d as DiffType, n, f.hh, f.mm, f.ss)
    }

    /// Returns the difference between fields structs using the indicated unit.
    pub(crate) const fn difference(f1: Fields, f2: Fields) -> DiffType {
        day_difference(f1.y, f1.m, f1.d, f2.y, f2.m, f2.d)
    }

    /// Aligns the (normalized) fields struct to the indicated field.
    pub(crate) const fn align(f: Fields) -> Fields {
        Fields {
            y: f.y,
            m: f.m,
            d: f.d,
            hh: 0,
            mm: 0,
            ss: 0,
        }
    }
}

/// Month granularity.
pub(crate) struct Month;

impl Month {
    /// Increments the indicated (normalized) field by "n".
    pub(crate) const fn step(f: Fields, n: DiffType) -> Fields {
        Fields::n_mon(
            f.y + n / 12,
            f.m as DiffType + n % 12,
            f.d as DiffType,
            0,
            f.hh,
            f.mm,
            f.ss,
        )
    }

    /// Returns the difference between fields structs using the indicated unit.
    pub(crate) const fn difference(f1: Fields, f2: Fields) -> DiffType {
        scale_add(Year::difference(f1, f2), 12, (f1.m - f2.m) as DiffType)
    }

    /// Aligns the (normalized) fields struct to the indicated field.
    pub(crate) const fn align(f: Fields) -> Fields {
        Fields {
            y: f.y,
            m: f.m,
            d: 1,
            hh: 0,
            mm: 0,
            ss: 0,
        }
    }
}

/// Year granularity.
pub(crate) struct Year;

impl Year {
    /// Increments the indicated (normalized) field by "n".
    pub(crate) const fn step(mut f: Fields, n: DiffType) -> Fields {
        f.y += n;
        f
    }

    /// Returns the difference between fields structs using the indicated unit.
    pub(crate) const fn difference(f1: Fields, f2: Fields) -> DiffType {
        f1.y - f2.y
    }

    /// Aligns the (normalized) fields struct to the indicated field.
    pub(crate) const fn align(f: Fields) -> Fields {
        Fields {
            y: f.y,
            m: 1,
            d: 1,
            hh: 0,
            mm: 0,
            ss: 0,
        }
    }
}

/// Returns (v * f + a) but avoiding intermediate overflow when possible.
const fn scale_add(v: DiffType, f: DiffType, a: DiffType) -> DiffType {
    if v < 0 {
        ((v + 1) * f + a) - f
    } else {
        ((v - 1) * f + a) + f
    }
}

/// Map a (normalized) Y/M/D to the number of days before/after 1970-01-01.
/// Probably overflows for years outside [-292277022656:292277026595].
const fn ymd_ord(y: YearType, m: MonthType, d: DayType) -> DiffType {
    let eyear = if m <= 2 { y - 1 } else { y };
    let era = (if eyear >= 0 { eyear } else { eyear - 399 }) / 400;
    let yoe = eyear - era * 400;
    let mp = (m + if m > 2 { -3 } else { 9 }) as DiffType;
    let doy = (153 * mp + 2) / 5 + d as DiffType - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy as DiffType;
    era * 146097 + doe - 719468
}

/// Returns the difference in days between two normalized Y-M-D tuples.
/// ymd_ord() will encounter integer overflow given extreme year values,
/// yet the difference between two such extreme values may actually be
/// small, so we take a little care to avoid overflow when possible by
/// exploiting the 146097-day cycle.
const fn day_difference(
    y1: YearType,
    m1: MonthType,
    d1: DayType,
    y2: YearType,
    m2: MonthType,
    d2: DayType,
) -> DiffType {
    let a_c4_off = y1 % 400;
    let b_c4_off = y2 % 400;
    let mut c4_diff = (y1 - a_c4_off) - (y2 - b_c4_off);
    let mut delta = ymd_ord(a_c4_off, m1, d1) - ymd_ord(b_c4_off, m2, d2);
    if c4_diff > 0 && delta < 0 {
        delta += 2 * 146097;
        c4_diff -= 2 * 400;
    } else if c4_diff < 0 && delta > 0 {
        delta -= 2 * 146097;
        c4_diff += 2 * 400;
    }
    (c4_diff / 400 * 146097) + delta
}
