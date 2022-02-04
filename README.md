# Civil Time
A library ported from the [`civil_time`](https://github.com/google/cctz/blob/master/include/cctz/civil_time.h) library in [cctz](https://github.com/google/cctz).

This library supports computing with human-scale time, such as dates (which are represented by the `CivilDay` struct) and can be considered as an supplement to the [`chrono`](https://github.com/chronotope/chrono) library. In particular, it provides:
- Types with different alignments (from second to year).
- Auto normalization if arguments passed to the constructor are out-of-range.
- Constant constructors of civil time types.
- Natural arithmetic operators such as addition, subtraction, and difference.
- Utilities to get next or previous weekday.

**NOTE: Now the API is still unstable** and may be changed in the future.

## Usage
Put this in your `Cargo.toml`:
```toml
[dependencies]
civil-time = { version = "0.1", git = "https://github.com/evenyag/civil-time.git" }
```

### Overview

The following six structs form the core of this civil-time library:
- `CivilSecond`
- `CivilMinute`
- `CivilHour`
- `CivilDay`
- `CivilMonth`
- `CivilYear`

Each struct is a simple value type with the same six accessors for each of the
civil fields (year, month, day, hour, minute, and second, aka YMDHMS). These structs
differ in their alignment, which is indicated by the type name and specifies the field on
which arithmetic operates.

### Construction
Each of the civil-time types can be constructed by `default()` method, or by directly
passing to the constructor up to six integers representing the
YMDHMS fields, or by `Builder::build`.

```rust
use civil_time::{Builder, CivilDay, CivilHour, CivilMinute, CivilMonth, CivilSecond, CivilYear};

// Default value is "1970-01-01 00:00:00"
let default_value = CivilDay::default();
assert_eq!("1970-01-01", format!("{:?}", default_value));

// Constructed by new(), e.g. "2015-02-03 00:00:00"
let a = CivilDay::new(2015, 2, 3);
assert_eq!("2015-02-03", format!("{:?}", a));

// Constructed by Builder, e.g. "2015-01-01 00:00:00"
let b = Builder::new().year(2015).build_day();
assert_eq!("2015-01-01", format!("{:?}", b));

// Constructed by Builder::build(), e.g. "2015-02-03 00:00:00"
let c: CivilDay = Builder::new()
    .year(2015)
    .month(2)
    .day(3)
    .hour(4)
    .minute(5)
    .second(6)
    .build();
assert_eq!("2015-02-03", format!("{:?}", c));

// e.g. "2015-02-03 04:05:06"
let ss = CivilSecond::new(2015, 2, 3, 4, 5, 6);
assert_eq!("2015-02-03T04:05:06", format!("{:?}", ss));

// e.g. "2015-02-03 04:05:00"
let mm = CivilMinute::new(2015, 2, 3, 4, 5);
assert_eq!("2015-02-03T04:05", format!("{:?}", mm));

// e.g. "2015-02-03 04:00:00"
let hh = CivilHour::new(2015, 2, 3, 4);
assert_eq!("2015-02-03T04", format!("{:?}", hh));

// e.g. "2015-02-03 00:00:00"
let d = CivilDay::new(2015, 2, 3);
assert_eq!("2015-02-03", format!("{:?}", d));

// e.g. "2015-02-01 00:00:00"
let m = CivilMonth::new(2015, 2);
assert_eq!("2015-02", format!("{:?}", m));

// e.g. "2015-01-01 00:00:00"
let y = CivilYear::new(2015);
assert_eq!("2015", format!("{:?}", y));
```

### Normalization
Integer arguments passed to the `new()` constructor may be out-of-range, in which
case they are normalized to produce a valid civil-time struct. This enables
natural arithmetic on `new()` arguments without worrying about the
field's range. Normalization guarantees that there are no invalid
civil-time structs.

```rust
use civil_time::CivilDay;

// Out-of-range day; normalized to 2016-11-01
let d = CivilDay::new(2016, 10, 32);
assert_eq!("2016-11-01", format!("{:?}", d));
```

### Arithmetic
Arithmetic operates on the civil-time field indicated in the type's name. Difference requires arguments with the same
alignment and returns the answer in units of the alignment.

```rust
use civil_time::{CivilDay, CivilMonth};

let mut a = CivilDay::new(2015, 2, 3);
a += 1;
assert_eq!("2015-02-04", format!("{:?}", a));
a -= 1;
assert_eq!("2015-02-03", format!("{:?}", a));

let b = a + 1;
assert_eq!("2015-02-04", format!("{:?}", b));
let c = b + 1;
assert_eq!("2015-02-05", format!("{:?}", c));

let n = c - a;
assert_eq!(2, n);
// let m = c - CivilMonth::from(c); // Won't compile: different types.
```

### Weekday
Returns the `CivilDay` that strictly follows or precedes the given
`CivilDay`, and that falls on the given weekday.

```rust
use civil_time::{CivilDay, Weekday};

let a = CivilDay::new(2015, 8, 13);
assert_eq!(Weekday::Thu, a.weekday());
let b = a.next_weekday(Weekday::Thu);
assert_eq!("2015-08-20", format!("{:?}", b));
let c = a.prev_weekday(Weekday::Thu);
assert_eq!("2015-08-06", format!("{:?}", c));

let d = a - 1;
// Gets the following Thursday if d is not already Thursday
let thurs1 = d.next_weekday(Weekday::Thu);
assert_eq!("2015-08-13", format!("{:?}", thurs1));
//  Gets the previous Thursday if d is not already Thursday
let d = a + 1;
let thurs2 = d.prev_weekday(Weekday::Thu);
assert_eq!("2015-08-13", format!("{:?}", thurs2));
```
