//! Comparision between civil time types.

use crate::{CivilDay, CivilHour, CivilMinute, CivilMonth, CivilSecond, CivilYear};
use std::cmp::Ordering;

macro_rules! impl_partial_eq_for {
    ($Type: ty, $Other: ty) => {
        impl PartialEq<$Other> for $Type {
            fn eq(&self, other: &$Other) -> bool {
                self.0 == other.0
            }
        }
    };
}

macro_rules! impl_partial_eq {
    ($($Type: ty)*) => ($(
        impl_partial_eq_for!($Type, CivilSecond);
        impl_partial_eq_for!($Type, CivilMinute);
        impl_partial_eq_for!($Type, CivilHour);
        impl_partial_eq_for!($Type, CivilDay);
        impl_partial_eq_for!($Type, CivilMonth);
        impl_partial_eq_for!($Type, CivilYear);
    )*)
}

impl_partial_eq! {
    CivilSecond CivilMinute CivilHour CivilDay CivilMonth CivilYear
}

macro_rules! impl_eq {
    ($($Type: ty)*) => ($(
        impl Eq for $Type {}
    )*);
}

impl_eq! {
    CivilSecond CivilMinute CivilHour CivilDay CivilMonth CivilYear
}

macro_rules! impl_partial_ord_for {
    ($Type: ty, $Other: ty) => {
        impl PartialOrd<$Other> for $Type {
            fn partial_cmp(&self, other: &$Other) -> Option<Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }
    };
}

macro_rules! impl_partial_ord {
    ($($Type: ty)*) => ($(
        impl_partial_ord_for!($Type, CivilSecond);
        impl_partial_ord_for!($Type, CivilMinute);
        impl_partial_ord_for!($Type, CivilHour);
        impl_partial_ord_for!($Type, CivilDay);
        impl_partial_ord_for!($Type, CivilMonth);
        impl_partial_ord_for!($Type, CivilYear);
    )*)
}

impl_partial_ord! {
    CivilSecond CivilMinute CivilHour CivilDay CivilMonth CivilYear
}

macro_rules! impl_ord {
    ($($Type: ty)*) => ($(
        impl Ord for $Type {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
    )*)
}

impl_ord! {
    CivilSecond CivilMinute CivilHour CivilDay CivilMonth CivilYear
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relational_ignore_aligment() {
        // Test that the alignment unit is ignored in comparision.
        let year = CivilYear::new(2014);
        let month = CivilMonth::from(year);
        let day = CivilDay::from(month);
        let hour = CivilHour::from(day);
        let minute = CivilMinute::from(hour);
        let second = CivilSecond::from(minute);

        macro_rules! check_val {
            ($val: ident) => {
                assert_eq!($val, second);
                assert_eq!($val, minute);
                assert_eq!($val, hour);
                assert_eq!($val, day);
                assert_eq!($val, month);
                assert_eq!($val, year);

                assert!($val <= second);
                assert!($val <= minute);
                assert!($val <= hour);
                assert!($val <= day);
                assert!($val <= month);
                assert!($val <= year);
            };
        }

        check_val!(second);
        check_val!(minute);
        check_val!(hour);
        check_val!(day);
        check_val!(month);
        check_val!(year);
    }

    macro_rules! check_relational {
        ($older: expr, $younger: expr) => {
            assert!(!($older < $older));
            assert!(!($older > $older));
            assert!($older >= $older);
            assert!($older <= $older);
            assert!(!($younger < $younger));
            assert!(!($younger > $younger));
            assert!($younger >= $younger);
            assert!($younger <= $younger);
            assert_eq!($older, $older);
            assert_ne!($older, $younger);
            assert!($older < $younger);
            assert!($older <= $younger);
            assert!($younger > $older);
            assert!($younger >= $older);
        };
    }

    #[test]
    fn test_relational_of_fields() {
        // Year
        check_relational!(
            CivilSecond::new(2014, 1, 1, 0, 0, 0),
            CivilSecond::new(2015, 1, 1, 0, 0, 0)
        );
        // Month
        check_relational!(
            CivilSecond::new(2014, 1, 1, 0, 0, 0),
            CivilSecond::new(2014, 2, 1, 0, 0, 0)
        );
        // Day
        check_relational!(
            CivilSecond::new(2014, 1, 1, 0, 0, 0),
            CivilSecond::new(2014, 1, 2, 0, 0, 0)
        );
        // Hour
        check_relational!(
            CivilSecond::new(2014, 1, 1, 0, 0, 0),
            CivilSecond::new(2014, 1, 1, 1, 0, 0)
        );
        // Minute
        check_relational!(
            CivilSecond::new(2014, 1, 1, 1, 0, 0),
            CivilSecond::new(2014, 1, 1, 1, 1, 0)
        );
        // Second
        check_relational!(
            CivilSecond::new(2014, 1, 1, 1, 1, 0),
            CivilSecond::new(2014, 1, 1, 1, 1, 1)
        );
    }

    #[test]
    fn test_relational_cross_types() {
        // Tests the relational operators of two different civil-time types.
        check_relational!(
            CivilDay::new(2014, 1, 1),
            CivilMinute::new(2014, 1, 1, 1, 1)
        );
        check_relational!(CivilDay::new(2014, 1, 1), CivilMonth::new(2014, 2));
    }
}
