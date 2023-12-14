/*
use std::collections::VecDeque;
*/

use itertools::Itertools;

use common::prelude::*;

/// Tuning Trouble
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let size = part.value(4, 14);
    /*
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
    */
    // I implemented my own window above but there is more straightforward way I was looking for yesterday.
    let chars: Vec<_> = input.trim_end().chars().collect();
    let (idx, _) = chars
        .as_slice()
        .windows(size)
        .find_position(|window| window.iter().all_unique())
        .context("No solution")?;
    // It still calls `all_unique` on each window (and therefore make a hashset each time)
    // and collect chars to vec could be bad if the input was very large but it is faster on given tests.
    Ok(size + idx)
}

pub const INPUTS: [&str; 6] = [
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n",
    "bvwbjplbgvbhsrlpgdmjqwftvncz\n",
    "nppdvjthqldpwncqszvftbrmjlhg\n",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n",
    include_input!(22 06),
];

#[test]
fn solver_22_06() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 7);
    assert_eq!(solver(Part1, INPUTS[1])?, 5);
    assert_eq!(solver(Part1, INPUTS[2])?, 6);
    assert_eq!(solver(Part1, INPUTS[3])?, 10);
    assert_eq!(solver(Part1, INPUTS[4])?, 11);
    assert_eq!(solver(Part1, INPUTS[5])?, 1929);

    assert_eq!(solver(Part2, INPUTS[0])?, 19);
    assert_eq!(solver(Part2, INPUTS[1])?, 23);
    assert_eq!(solver(Part2, INPUTS[2])?, 23);
    assert_eq!(solver(Part2, INPUTS[3])?, 29);
    assert_eq!(solver(Part2, INPUTS[4])?, 26);
    assert_eq!(solver(Part2, INPUTS[5])?, 3298);

    Ok(())
}
