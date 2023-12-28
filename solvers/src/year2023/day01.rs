use itertools::Itertools;

use common::prelude::*;

/// Trebuchet?!
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let digits: &[&str] = part.value(
        &["1", "2", "3", "4", "5", "6", "7", "8", "9"],
        &[
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ],
    );
    #[allow(clippy::cast_possible_truncation)] // `digits.len() <= u32::MAX`
    input
        .lines()
        .map(|line| {
            // find + min  vs  rfind + max
            let (_, first) = digits
                .iter()
                .enumerate()
                .filter_map(|(idx, digit)| line.find(digit).map(|pos| (pos, idx)))
                .min()
                .context("No first digit found")?;
            let (_, last) = digits
                .iter()
                .enumerate()
                .filter_map(|(idx, digit)| line.rfind(digit).map(|pos| (pos, idx)))
                .max()
                .context("No last digit found, but a first was?!")?;
            Ok((first as u32 % 9 + 1, last as u32 % 9 + 1))
        })
        .process_results(|it| it.map(|(c1, c2)| c1 * 10 + c2).sum())
}

test_solver! {
    "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
" => 142,
    "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
" => ((), 281),
    include_input!(23 01) => (54990, 54473),
}
