//! Conversion between civil time types.

use crate::{CivilDay, CivilHour, CivilMinute, CivilMonth, CivilSecond, CivilYear};

macro_rules! impl_from {
    ($Type: ty, $func: ident, $Other: ty) => {
        impl $Type {
            pub const fn $func(other: $Other) -> Self {
                Self::from_fields(other.0)
            }
        }

        impl From<$Other> for $Type {
            fn from(other: $Other) -> Self {
                Self::$func(other)
            }
        }
    };
}

// Implement from for CivilSecond.
impl_from!(CivilSecond, from_minute, CivilMinute);
impl_from!(CivilSecond, from_hour, CivilHour);
impl_from!(CivilSecond, from_day, CivilDay);
impl_from!(CivilSecond, from_month, CivilMonth);
impl_from!(CivilSecond, from_year, CivilYear);

// Implement from for CivilMinute.
impl_from!(CivilMinute, from_second, CivilSecond);
impl_from!(CivilMinute, from_hour, CivilHour);
impl_from!(CivilMinute, from_day, CivilDay);
impl_from!(CivilMinute, from_month, CivilMonth);
impl_from!(CivilMinute, from_year, CivilYear);

// Implement from for CivilHour.
impl_from!(CivilHour, from_second, CivilSecond);
impl_from!(CivilHour, from_minute, CivilMinute);
impl_from!(CivilHour, from_day, CivilDay);
impl_from!(CivilHour, from_month, CivilMonth);
impl_from!(CivilHour, from_year, CivilYear);

// Implement from for CivilDay.
impl_from!(CivilDay, from_second, CivilSecond);
impl_from!(CivilDay, from_minute, CivilMinute);
impl_from!(CivilDay, from_hour, CivilHour);
impl_from!(CivilDay, from_month, CivilMonth);
impl_from!(CivilDay, from_year, CivilYear);

// Implement from for CivilMonth.
impl_from!(CivilMonth, from_second, CivilSecond);
impl_from!(CivilMonth, from_minute, CivilMinute);
impl_from!(CivilMonth, from_hour, CivilHour);
impl_from!(CivilMonth, from_day, CivilDay);
impl_from!(CivilMonth, from_year, CivilYear);

// Implement from for CivilYear.
impl_from!(CivilYear, from_second, CivilSecond);
impl_from!(CivilYear, from_minute, CivilMinute);
impl_from!(CivilYear, from_hour, CivilHour);
impl_from!(CivilYear, from_day, CivilDay);
impl_from!(CivilYear, from_month, CivilMonth);

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions as sa;

    #[test]
    fn test_conversion() {
        const _CY: CivilYear = CivilYear::new(2016);
        sa::const_assert_eq!(2016, _CY.year());
        const _CM: CivilMonth = CivilMonth::from_year(_CY);
        sa::const_assert_eq!(1, _CM.month());
        const _CD: CivilDay = CivilDay::from_month(_CM);
        sa::const_assert_eq!(1, _CD.day());
        const _CHH: CivilHour = CivilHour::from_day(_CD);
        sa::const_assert_eq!(0, _CHH.hour());
        const _CMM: CivilMinute = CivilMinute::from_hour(_CHH);
        sa::const_assert_eq!(0, _CMM.minute());
        const _CSS: CivilSecond = CivilSecond::from_minute(_CMM);
        sa::const_assert_eq!(0, _CSS.minute());
    }

    // 2016-03-28T17:14:12
    macro_rules! check_from_second {
        ($Type: ident) => {{
            const _V: $Type = $Type::from_second(CivilSecond::new(2016, 3, 28, 17, 14, 12));
            sa::const_assert_eq!(0, _V.second());
        }};
    }

    // 2016-03-28T17:14:00
    macro_rules! check_from_minute {
        ($Type: ident, $expect: expr) => {{
            const _V: $Type = $Type::from_minute(CivilMinute::new(2016, 3, 28, 17, 14));
            sa::const_assert_eq!($expect, _V.minute());
        }};
    }

    // 2016-03-28T17:00:00
    macro_rules! check_from_hour {
        ($Type: ident, $expect: expr) => {{
            const _V: $Type = $Type::from_hour(CivilHour::new(2016, 3, 28, 17));
            sa::const_assert_eq!($expect, _V.hour());
        }};
    }

    // 2016-03-28T00:00:00
    macro_rules! check_from_day {
        ($Type: ident, $expect: expr) => {{
            const _V: $Type = $Type::from_day(CivilDay::new(2016, 3, 28));
            sa::const_assert_eq!($expect, _V.day());
        }};
    }

    // 2016-03-01T00:00:00
    macro_rules! check_from_month {
        ($Type: ident, $expect: expr) => {{
            const _V: $Type = $Type::from_month(CivilMonth::new(2016, 3));
            sa::const_assert_eq!($expect, _V.month());
        }};
    }

    // 2016-01-01T00:00:00
    macro_rules! check_from_year {
        ($Type: ident) => {{
            const _V: $Type = $Type::from_year(CivilYear::new(2016));
            sa::const_assert_eq!(2016, _V.year());
        }};
    }

    #[test]
    fn test_second_conversion() {
        check_from_minute!(CivilSecond, 14);
        check_from_hour!(CivilSecond, 17);
        check_from_day!(CivilSecond, 28);
        check_from_month!(CivilSecond, 3);
        check_from_year!(CivilSecond);
    }

    #[test]
    fn test_minute_conversion() {
        check_from_second!(CivilMinute);
        check_from_hour!(CivilMinute, 17);
        check_from_day!(CivilMinute, 28);
        check_from_month!(CivilMinute, 3);
        check_from_year!(CivilMinute);
    }

    #[test]
    fn test_hour_conversion() {
        check_from_second!(CivilHour);
        check_from_minute!(CivilHour, 0);
        check_from_day!(CivilHour, 28);
        check_from_month!(CivilHour, 3);
        check_from_year!(CivilHour);
    }

    #[test]
    fn test_day_conversion() {
        check_from_second!(CivilDay);
        check_from_minute!(CivilDay, 0);
        check_from_hour!(CivilDay, 0);
        check_from_month!(CivilDay, 3);
        check_from_year!(CivilDay);
    }

    #[test]
    fn test_month_conversion() {
        check_from_second!(CivilMonth);
        check_from_minute!(CivilMonth, 0);
        check_from_hour!(CivilMonth, 0);
        check_from_day!(CivilMonth, 1);
        check_from_year!(CivilMonth);
    }

    #[test]
    fn test_year_conversion() {
        check_from_second!(CivilYear);
        check_from_minute!(CivilYear, 0);
        check_from_hour!(CivilYear, 0);
        check_from_day!(CivilYear, 1);
        check_from_month!(CivilYear, 1);
    }
}
