macro_rules! include_input {
    ($year:literal $day:literal) => {
        include_str!(concat!(
            "../../../inputs/20",
            stringify!($year),
            "/",
            stringify!($day),
            ".txt"
        ))
    };
}

/// Defines the public constant `INPUTS: [&str; _]` and write a test named `test_solver`.
///
/// ## Usage
/// ```text
/// test_solver! {
///     "1" => ("p1", ),     // Part1 only, Part2 soon
///     "2" => ("p1", "p2"), // both parts
///     "3" => ((), "p2"),   // Part2 only
///     "25" => "p1",        // Part1 only
///     include_input!(15 01) => (12345, 67890),
/// }
/// ```
/// or if I have attributes to give to the test function
/// ```text
/// test_solver! {
///     #[cfg_attr(feature = "lp", ignore)] // too slow
///     {
///         // input => answers,
///     }
/// }
/// ```
macro_rules! test_solver {
    (
        $(#[$attr:meta])*
        {
            $(
                $input:expr => $answers:expr
            ),*
            $(,)?
        }
    ) => {
        pub const INPUTS: [&str; count_exprs!($($input,)*)] = [$($input,)*];

        #[test]
        $(#[$attr])*
        fn test_solver() -> ::common::Result<()> {
            use crate::traits::TestAnswers;
            let inputs = INPUTS;
            let all_answers = [$($answers.test_answers()),*];
            for (part_idx, part) in ::common::Part::ALL.into_iter().enumerate() {
                for (test_idx, (input, answers)) in inputs.iter().zip(&all_answers).enumerate() {
                    if let Some(answer) = answers[part_idx] {
                        assert_eq!(&solver(part, input)?, answer, "{:?} input #{}", part, test_idx);
                    }
                }
            }
            Ok(())
        }
    };
    (
        $(
            $input:expr => $answers:expr
        ),*
        $(,)?
    ) => {
        test_solver! {{
            $($input => $answers),*
        }}
    };
}

macro_rules! count_exprs {
    () => {
        0
    };
    ($v0:expr, $($v:expr,)*) => {
        1 + count_exprs!($($v,)*)
    };
}
