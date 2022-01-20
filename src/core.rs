//! Core types, functions and field normalization (without avoidable overflow).

/// Support years that at least span the range of 64-bit time_t values.
pub type YearType = i64;
/// Type alias that indicates an argument is not normalized (e.g., the
/// constructor parameters and operands/results of addition/subtraction).
pub type DiffType = i64;
// Type aliases that indicate normalized argument values.
/// Normalized month [1:12].
pub(crate) type MonthType = i8;
/// Normalized day [1:31].
pub(crate) type DayType = i8;
/// Normalized hour [0:23].
pub(crate) type HourType = i8;
/// Normalized minute [0:59].
pub(crate) type MinuteType = i8;
/// Normalized second [0:59].
pub(crate) type SecondType = i8;

pub(crate) const fn is_leap_year(y: YearType) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

/// Returns the mod-400 index year.
const fn year_index(y: YearType, m: MonthType) -> usize {
    let yi = (y + if m > 2 { 1 } else { 0 }) % 400;
    if yi < 0 {
        (yi + 400) as usize
    } else {
        yi as usize
    }
}

/// Returns the number of days in the 100 years starting in the mod-400 index year `yi`.
const fn days_per_century(yi: usize) -> i64 {
    36524 + if yi == 0 || yi > 300 { 1 } else { 0 }
}

/// Returns the number of days in the 4 years starting in the mod-400 index year `yi`.
const fn days_per_4years(yi: usize) -> i64 {
    1460 + if yi == 0 || yi > 300 || (yi - 1) % 100 < 96 {
        1
    } else {
        0
    }
}

const fn days_per_year(y: YearType, m: MonthType) -> i64 {
    let leap = is_leap_year(y + if m > 2 { 1 } else { 0 });
    if leap {
        366
    } else {
        365
    }
}

const fn days_per_month(y: YearType, m: MonthType) -> i64 {
    // non leap year
    const DAYS_PER_MONTH: [i64; 13] = [-1, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let non_leap_days = DAYS_PER_MONTH[m as usize];
    non_leap_days + if m == 2 && is_leap_year(y) { 1 } else { 0 }
}

/// Normalized civil-time fields: Y-M-D HH:MM:SS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fields {
    pub y: YearType,
    pub m: MonthType,
    pub d: DayType,
    pub hh: HourType,
    pub mm: MinuteType,
    pub ss: SecondType,
}

impl Fields {
    pub const fn n_day(
        y: YearType,
        mut m: MonthType,
        mut d: DiffType,
        mut cd: DiffType,
        hh: HourType,
        mm: MinuteType,
        ss: SecondType,
    ) -> Fields {
        let mut ey = y % 400;
        let oey = ey;

        ey += (cd / 146097) * 400;
        cd %= 146097;
        if cd < 0 {
            ey -= 400;
            cd += 146097;
        }

        ey += (d / 146097) * 400;
        d = d % 146097 + cd;
        if d > 0 {
            if d > 146097 {
                ey += 400;
                d -= 146097;
            }
        } else {
            if d > -365 {
                // We often hit the previous year when stepping a civil time backwards,
                // so special case it to avoid counting up by 100/4/1-year chunks.
                ey -= 1;
                d += days_per_year(ey, m);
            } else {
                ey -= 400;
                d += 146097;
            }
        }

        if d > 365 {
            let mut yi = year_index(ey, m); // Index into Gregorian 400 year cycle.
            loop {
                let n = days_per_century(yi);
                if d <= n {
                    break;
                }
                d -= n;
                ey += 100;
                yi += 100;
                if yi >= 400 {
                    yi -= 400;
                }
            }
            loop {
                let n = days_per_4years(yi);
                if d <= n {
                    break;
                }
                d -= n;
                ey += 4;
                yi += 4;
                if yi >= 400 {
                    yi -= 400;
                }
            }
            loop {
                let n = days_per_year(ey, m);
                if d <= n {
                    break;
                }
                d -= n;
                ey += 1;
            }
        }

        if d > 28 {
            loop {
                let n = days_per_month(ey, m);
                if d <= n {
                    break;
                }
                d -= n;
                m += 1;
                if m > 12 {
                    ey += 1;
                    m = 1;
                }
            }
        }

        Fields {
            y: y + (ey - oey),
            m,
            d: d as DayType,
            hh,
            mm,
            ss,
        }
    }

    pub const fn n_mon(
        mut y: YearType,
        mut m: DiffType,
        d: DiffType,
        cd: DiffType,
        hh: HourType,
        mm: MinuteType,
        ss: SecondType,
    ) -> Fields {
        if m != 12 {
            y += m / 12;
            m %= 12;
            if m <= 0 {
                y -= 1;
                m += 12;
            }
        }
        Self::n_day(y, m as MonthType, d, cd, hh, mm, ss)
    }

    pub const fn n_hour(
        y: YearType,
        m: DiffType,
        d: DiffType,
        mut cd: DiffType,
        mut hh: DiffType,
        mm: MinuteType,
        ss: SecondType,
    ) -> Fields {
        cd += hh / 24;
        hh %= 24;
        if hh < 0 {
            cd -= 1;
            hh += 24;
        }
        Self::n_mon(y, m, d, cd, hh as HourType, mm, ss)
    }

    pub const fn n_min(
        y: YearType,
        m: DiffType,
        d: DiffType,
        hh: DiffType,
        mut ch: DiffType,
        mut mm: DiffType,
        ss: SecondType,
    ) -> Fields {
        ch += mm / 60;
        mm %= 60;
        if mm < 0 {
            ch -= 1;
            mm += 60;
        }
        Self::n_hour(
            y,
            m,
            d,
            hh / 24 + ch / 24,
            hh % 24 + ch % 24,
            mm as MinuteType,
            ss,
        )
    }

    pub const fn n_sec(
        y: YearType,
        m: DiffType,
        d: DiffType,
        hh: DiffType,
        mm: DiffType,
        mut ss: DiffType,
    ) -> Fields {
        // Optimization for when (non-constexpr) fields are already normalized.
        if 0 <= ss && ss < 60 {
            let nss = ss as SecondType;
            if 0 <= mm && mm < 60 {
                let nmm = mm as MinuteType;
                if 0 <= hh && hh < 24 {
                    let nhh = hh as HourType;
                    if 1 <= d && d <= 28 && 1 <= m && m <= 12 {
                        let nd = d as DayType;
                        let nm = m as MonthType;
                        return Fields {
                            y,
                            m: nm,
                            d: nd,
                            hh: nhh,
                            mm: nmm,
                            ss: nss,
                        };
                    }
                    return Self::n_mon(y, m, d, 0, nhh, nmm, nss);
                }
                return Self::n_hour(y, m, d, hh / 24, hh % 24, nmm, nss);
            }
            return Self::n_min(y, m, d, hh, mm / 60, mm % 60, nss);
        }
        let mut cm = ss / 60;
        ss %= 60;
        if ss < 0 {
            cm -= 1;
            ss += 60;
        }
        Self::n_min(
            y,
            m,
            d,
            hh,
            mm / 60 + cm / 60,
            mm % 60 + cm % 60,
            ss as SecondType,
        )
    }
}
