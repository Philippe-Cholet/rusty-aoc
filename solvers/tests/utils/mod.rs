/// ## Usage example
/// For `solvers/tests/name.rs`:
/// ```text
/// #[macro_use]
/// mod utils;
///
/// test_collection!(
///     name,
///     22 => {
///         01: (12345, "abcde"),
///         02: (_, "abcde"), // Do not run Part1
///         03: (12345, _),   // Do not run Part2
///         // 04: (_, _),
///         // 05: (_, _),
///         // 06: (_, _),
///         // 07: (_, _),
///         // 08: (_, _),
///         // 09: (_, _),
///         // 10: (_, _),
///         // 11: (_, _),
///         // 12: (_, _),
///         // 13: (_, _),
///         // 14: (_, _),
///         // 15: (_, _),
///         // 16: (_, _),
///         // 17: (_, _),
///         // 18: (_, _),
///         // 19: (_, _),
///         // 20: (_, _),
///         // 21: (_, _),
///         // 22: (_, _),
///         // 23: (_, _),
///         // 24: (_, _),
///         // 25: (_),       // No Part2, note the mandatory parentheses.
///     }
/// );
/// ```
macro_rules! test_collection {
    (
        $username:ident,
        $(
            $year:literal => {
                $(
                    $day:literal: ($p1:tt$(, $p2:tt)?),
                )*
            }
        )*
    ) => {
        paste::paste! {
            $(
                #[allow(clippy::zero_prefixed_literal)] // 0s needed for `stringify`.
                #[allow(non_snake_case)] // Usernames are not formatted.
                #[test]
                fn [<$username _ $year>]() -> common::Result<()> {
                    let year = $year.to_string().parse()?;
                    $(
                        let day = $day.to_string().parse()?;
                        if let Ok((solver, _)) = solvers::aoc(year, day) {
                            let input = include_str!(concat!(
                                "../../inputs/other/",
                                stringify!($username),
                                "/20",
                                stringify!($year),
                                "/",
                                stringify!($day),
                                ".txt"
                            ));
                            test_collection!(@test $username year day solver input Part1 $p1);
                            $(
                                test_collection!(@test $username year day solver input Part2 $p2);
                            )?
                        }
                    )*
                    Ok(())
                }

                // Or more precise tests, but build tests is slower:
                // $(
                //     #[allow(non_snake_case)] // Usernames are not formatted.
                //     #[test]
                //     fn [<$username _year $year _day $day>]() -> common::Result<()> {
                //         let year = $year.to_string().parse()?;
                //         let day = $day.to_string().parse()?;
                //         if let Ok((solver, _)) = solvers::aoc(year, day) {
                //             let input = include_str!(concat!(
                //                 "../../inputs/other/",
                //                 stringify!($username),
                //                 "/20",
                //                 stringify!($year),
                //                 "/",
                //                 stringify!($day),
                //                 ".txt"
                //             ));

                //             test_collection!(@test $username year day solver input Part1 $p1);
                //             $(
                //                 test_collection!(@test $username year day solver input Part2 $p2);
                //             )?
                //         }
                //         Ok(())
                //     }
                // )*
            )*
        }
    };
    // Ignore the test if the answer is `_`.
    (@test $username:ident $year:ident $day:ident $solver:ident $input:ident $part:ident _) => {};
    (@test $username:ident $year:ident $day:ident $solver:ident $input:ident $part:ident $answer:literal) => {
        assert_eq!(
            $solver.solve(common::$part, $input)?,
            utils::Literal::from($answer),
            "{}: {:?} {:?} {:?}",
            stringify!($username), $year, $day, common::$part
        );
    };
}

/// Test answers can either be (unsigned) integers or string.
/// This enum is useful for type inference for integers to not be
/// the default `i32` when it obviously is `u64`.
///
/// I could do without this enum by quoting every integer answer but that would not be nice.
pub enum Literal {
    Int(u128),
    Str(&'static str),
}

// Debug values without showing variant names
impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Int(n) => n.fmt(f),
            Self::Str(s) => s.fmt(f),
        }
    }
}

impl From<u128> for Literal {
    #[inline]
    fn from(value: u128) -> Self {
        Self::Int(value)
    }
}

impl From<&'static str> for Literal {
    #[inline]
    fn from(value: &'static str) -> Self {
        Self::Str(value)
    }
}

impl PartialEq<Literal> for String {
    fn eq(&self, other: &Literal) -> bool {
        match other {
            Literal::Int(n) => self == &n.to_string(),
            Literal::Str(s) => self.as_str() == *s,
        }
    }
}
