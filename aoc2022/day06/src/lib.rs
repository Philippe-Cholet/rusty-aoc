use std::collections::VecDeque;

use itertools::Itertools;

use common::{Context, Part, Part1, Part2, Result};

/// Tuning Trouble
pub fn solver(part: Part, input: &str) -> Result<String> {
    let size = match part {
        Part1 => 4,
        Part2 => 14,
    };
    let mut chars = input.trim_end().chars();
    let mut window: VecDeque<_> = chars.by_ref().take(size).collect();
    let (idx, _) = chars
        .find_position(|ch| {
            if window.len() < size {
                false // Not enough characters in the window anymore.
            } else if window.iter().all_unique() {
                true
            } else {
                window.push_back(*ch);
                window.pop_front();
                false
            }
        })
        .context("No solution")?;
    Ok((size + idx).to_string())
}

pub const INPUTS: [&str; 6] = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n",
    "bvwbjplbgvbhsrlpgdmjqwftvncz\n",
    "nppdvjthqldpwncqszvftbrmjlhg\n",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n",
    include_str!("input.txt"),
];

#[test]
fn solver_22_06() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "7");
    assert_eq!(solver(Part1, INPUTS[1])?, "5");
    assert_eq!(solver(Part1, INPUTS[2])?, "6");
    assert_eq!(solver(Part1, INPUTS[3])?, "10");
    assert_eq!(solver(Part1, INPUTS[4])?, "11");
    assert_eq!(solver(Part1, INPUTS[5])?, "1929");

    assert_eq!(solver(Part2, INPUTS[0])?, "19");
    assert_eq!(solver(Part2, INPUTS[1])?, "23");
    assert_eq!(solver(Part2, INPUTS[2])?, "23");
    assert_eq!(solver(Part2, INPUTS[3])?, "29");
    assert_eq!(solver(Part2, INPUTS[4])?, "26");
    assert_eq!(solver(Part2, INPUTS[5])?, "3298");

    Ok(())
}
