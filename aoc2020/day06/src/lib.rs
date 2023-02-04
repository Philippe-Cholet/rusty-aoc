use std::ops::{BitAnd, BitOr};

use common::{ensure, Context, Part, Part1, Part2, Result};
use utils::OkIterator;

/// Custom Customs
pub fn solver(part: Part, input: &str) -> Result<String> {
    ensure!(
        input.chars().all(|ch| matches!(ch, 'a'..='z' | '\n')),
        "Not a-z",
    );
    // The first 26 bits of u32 represent the set of the 26 chars "a..=z".
    Ok(input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.bytes().map(|b| 1 << (b - b'a')).fold(0, BitOr::bitor))
                .reduce(match part {
                    Part1 => BitOr::bitor,   // All the chars.
                    Part2 => BitAnd::bitand, // Only the common chars.
                })
                .map(u32::count_ones) // Count the chars.
                .context("Empty group")
        })
        .ok_sum::<u32>()?
        .to_string())
}

pub const INPUTS: [&str; 2] = [
    "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb\n",
    include_str!("input.txt"),
];

#[test]
fn solver_20_06() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "11");
    assert_eq!(solver(Part1, INPUTS[1])?, "6748");
    assert_eq!(solver(Part2, INPUTS[0])?, "6");
    assert_eq!(solver(Part2, INPUTS[1])?, "3445");
    Ok(())
}
