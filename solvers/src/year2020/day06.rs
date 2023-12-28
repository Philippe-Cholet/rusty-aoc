use std::ops::{BitAnd, BitOr};

use common::prelude::*;

/// Custom Customs
pub fn solver(part: Part, input: &str) -> Result<u32> {
    ensure!(
        input.chars().all(|ch| matches!(ch, 'a'..='z' | '\n')),
        "Not a-z",
    );
    // The first 26 bits of u32 represent the set of the 26 chars "a..=z".
    input
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
        .sum()
}

test_solver! {
    "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb" => (11, 6),
    include_input!(20 06) => (6748, 3445),
}
