/// The common prelude imports:
/// - the `Part` enum and its variants `Part1` and `Part2` ;
/// - re-export most of `anyhow`: `bail`, `ensure`, `format_err`, `Context`, `Error`, `Result`
/// but not its function `Ok` (available outside the prelude) to be able to match against result variants.
/// - `HashMap` and `HashSet` from the `rustc-hash` crate both extended with 2 methods
/// (`new` and `with_capacity`) to be a nearly drop-in replacement of the ones from `std::collections`
/// (but it does not implement `From<[(K, V); N]>`).
///
/// Then one can do `use common::prelude::*` in a solver and start get things done without looking back.
pub mod prelude {
    pub use crate::{bail, ensure, format_err, Context, Error, Result};
    pub use crate::{Part, Part1, Part2};
    // My solvers do not need `Day`, `Year` and `AocSolver`.
    pub use crate::hash::prelude::*;
    pub use crate::include_input;
}

use std::str::FromStr;
use std::time::{Duration, Instant};

pub use anyhow::{bail, ensure, format_err, Context, Error, Ok, Result};

pub use self::{Day::*, Part::*, Year::*};

#[macro_export]
macro_rules! include_input {
    ($year:literal $day:literal) => {
        include_str!(concat!(
            "../../../inputs/20",
            stringify!($year),
            "/",
            stringify!($day),
            ".txt"
        ));
    };
}

pub trait AocSolver {
    fn solve(&self, part: Part, input: &str) -> Result<String>;
    fn timed_solve(&self, part: Part, input: &str) -> Result<(String, Duration)>;
}

impl<T, F> AocSolver for F
where
    T: std::fmt::Display,
    F: Fn(Part, &str) -> Result<T>,
{
    fn solve(&self, part: Part, input: &str) -> Result<String> {
        self(part, input).map(|t| t.to_string())
    }

    fn timed_solve(&self, part: Part, input: &str) -> Result<(String, Duration)> {
        let now = Instant::now();
        let t = self(part, input)?;
        let elapsed = now.elapsed();
        Ok((t.to_string(), elapsed))
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Year {
    Year2015,
    Year2016,
    Year2017,
    Year2018,
    Year2019,
    Year2020,
    Year2021,
    Year2022,
    Year2023,
    Year2024,
}

impl FromStr for Year {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "2015" | "15" => Self::Year2015,
            "2016" | "16" => Self::Year2016,
            "2017" | "17" => Self::Year2017,
            "2018" | "18" => Self::Year2018,
            "2019" | "19" => Self::Year2019,
            "2020" | "20" => Self::Year2020,
            "2021" | "21" => Self::Year2021,
            "2022" | "22" => Self::Year2022,
            "2023" | "23" => Self::Year2023,
            "2024" | "24" => Self::Year2024,
            v => bail!("Failed to parse year ([20]15..): {}", v),
        })
    }
}

impl Year {
    pub const ALL: [Self; 10] = [
        Year2015, Year2016, Year2017, Year2018, Year2019, Year2020, Year2021, Year2022, Year2023,
        Year2024,
    ];
}

impl From<Year> for u8 {
    fn from(year: Year) -> Self {
        match year {
            Year2015 => 15,
            Year2016 => 16,
            Year2017 => 17,
            Year2018 => 18,
            Year2019 => 19,
            Year2020 => 20,
            Year2021 => 21,
            Year2022 => 22,
            Year2023 => 23,
            Year2024 => 24,
        }
    }
}

impl From<Year> for i32 {
    fn from(year: Year) -> Self {
        match year {
            Year2015 => 2015,
            Year2016 => 2016,
            Year2017 => 2017,
            Year2018 => 2018,
            Year2019 => 2019,
            Year2020 => 2020,
            Year2021 => 2021,
            Year2022 => 2022,
            Year2023 => 2023,
            Year2024 => 2024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "01" | "1" => Self::Day1,
            "02" | "2" => Self::Day2,
            "03" | "3" => Self::Day3,
            "04" | "4" => Self::Day4,
            "05" | "5" => Self::Day5,
            "06" | "6" => Self::Day6,
            "07" | "7" => Self::Day7,
            "08" | "8" => Self::Day8,
            "09" | "9" => Self::Day9,
            "10" => Self::Day10,
            "11" => Self::Day11,
            "12" => Self::Day12,
            "13" => Self::Day13,
            "14" => Self::Day14,
            "15" => Self::Day15,
            "16" => Self::Day16,
            "17" => Self::Day17,
            "18" => Self::Day18,
            "19" => Self::Day19,
            "20" => Self::Day20,
            "21" => Self::Day21,
            "22" => Self::Day22,
            "23" => Self::Day23,
            "24" => Self::Day24,
            "25" => Self::Day25,
            v => bail!("Failed to parse day (1..=25): {}", v),
        })
    }
}

impl Day {
    pub const ALL: [Self; 25] = [
        Day1, Day2, Day3, Day4, Day5, Day6, Day7, Day8, Day9, Day10, Day11, Day12, Day13, Day14,
        Day15, Day16, Day17, Day18, Day19, Day20, Day21, Day22, Day23, Day24, Day25,
    ];
}

impl From<Day> for u8 {
    fn from(day: Day) -> Self {
        match day {
            Day1 => 1,
            Day2 => 2,
            Day3 => 3,
            Day4 => 4,
            Day5 => 5,
            Day6 => 6,
            Day7 => 7,
            Day8 => 8,
            Day9 => 9,
            Day10 => 10,
            Day11 => 11,
            Day12 => 12,
            Day13 => 13,
            Day14 => 14,
            Day15 => 15,
            Day16 => 16,
            Day17 => 17,
            Day18 => 18,
            Day19 => 19,
            Day20 => 20,
            Day21 => 21,
            Day22 => 22,
            Day23 => 23,
            Day24 => 24,
            Day25 => 25,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Part {
    Part1,
    Part2,
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "1" => Self::Part1,
            "2" => Self::Part2,
            v => bail!("Failed to parse part (1..=2): {}", v),
        })
    }
}

impl Part {
    pub const ALL: [Self; 2] = [Part1, Part2];

    #[must_use]
    #[inline]
    pub const fn one(self) -> bool {
        matches!(self, Part1)
    }

    #[must_use]
    #[inline]
    pub const fn two(self) -> bool {
        matches!(self, Part2)
    }

    // Can not be const because `one` or `two` is dropped and the destructor might not be evaluated at compile time.
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    pub fn value<T>(self, one: T, two: T) -> T {
        match self {
            Part1 => one,
            Part2 => two,
        }
    }
}

pub mod hash {
    #![allow(clippy::default_trait_access, clippy::implicit_hasher)]

    /// `HashMap` and `HashSet` from the `rustc-hash` crate.
    /// And an unnamed trait to mimic the ones in `std::collections`.
    pub mod prelude {
        pub use super::{FxHasherHack as _, HashMap, HashSet};
    }

    #[allow(clippy::module_name_repetitions)]
    pub use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

    /// `std::collections::HashMap<K, V>` has 2 methods (`new` and `with_capacity`) that
    /// `rustc_hash::FxHashMap<K, V>` does not have.
    /// Same for the `Set` variants.
    ///
    /// It's because `rustc_hash::FxHasher` is not `std::collections::hash_map::RandomState`.
    /// This trait is intended to mimic this behavior.
    pub trait FxHasherHack: Default {
        #[must_use]
        #[inline]
        fn new() -> Self {
            Self::default()
        }
        fn with_capacity(capacity: usize) -> Self;
    }

    impl<K, V> FxHasherHack for HashMap<K, V> {
        #[must_use]
        #[inline]
        fn with_capacity(capacity: usize) -> Self {
            Self::with_capacity_and_hasher(capacity, Default::default())
        }
    }

    impl<T> FxHasherHack for HashSet<T> {
        #[must_use]
        #[inline]
        fn with_capacity(capacity: usize) -> Self {
            Self::with_capacity_and_hasher(capacity, Default::default())
        }
    }
}
