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

test_solver! {
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb\n" => (7, 19),
    "bvwbjplbgvbhsrlpgdmjqwftvncz\n" => (5, 23),
    "nppdvjthqldpwncqszvftbrmjlhg\n" => (6, 23),
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n" => (10, 29),
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n" => (11, 26),
    include_input!(22 06) => (1929, 3298),
}
