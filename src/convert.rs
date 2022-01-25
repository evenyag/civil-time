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

// Impl from for CivilSecond.
impl_from!(CivilSecond, from_minute, CivilMinute);
impl_from!(CivilSecond, from_hour, CivilHour);
impl_from!(CivilSecond, from_day, CivilDay);
impl_from!(CivilSecond, from_month, CivilMonth);
impl_from!(CivilSecond, from_year, CivilYear);

// Impl from for CivilMinute.
impl_from!(CivilMinute, from_second, CivilSecond);
impl_from!(CivilMinute, from_hour, CivilHour);
impl_from!(CivilMinute, from_day, CivilDay);
impl_from!(CivilMinute, from_month, CivilMonth);
impl_from!(CivilMinute, from_year, CivilYear);

// Impl from for CivilHour.
impl_from!(CivilHour, from_second, CivilSecond);
impl_from!(CivilHour, from_minute, CivilMinute);
impl_from!(CivilHour, from_day, CivilDay);
impl_from!(CivilHour, from_month, CivilMonth);
impl_from!(CivilHour, from_year, CivilYear);

// Impl from for CivilDay.
impl_from!(CivilDay, from_second, CivilSecond);
impl_from!(CivilDay, from_minute, CivilMinute);
impl_from!(CivilDay, from_hour, CivilHour);
impl_from!(CivilDay, from_month, CivilMonth);
impl_from!(CivilDay, from_year, CivilYear);

// Impl from for CivilMonth.
impl_from!(CivilMonth, from_second, CivilSecond);
impl_from!(CivilMonth, from_minute, CivilMinute);
impl_from!(CivilMonth, from_hour, CivilHour);
impl_from!(CivilMonth, from_day, CivilDay);
impl_from!(CivilMonth, from_year, CivilYear);

// Impl from for CivilYear.
impl_from!(CivilYear, from_second, CivilSecond);
impl_from!(CivilYear, from_minute, CivilMinute);
impl_from!(CivilYear, from_hour, CivilHour);
impl_from!(CivilYear, from_day, CivilDay);
impl_from!(CivilYear, from_month, CivilMonth);
