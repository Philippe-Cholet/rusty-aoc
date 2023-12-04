use itertools::Itertools;

use common::prelude::*;

/// Doesn't He Have Intern-Elves For This?
pub fn solver(part: Part, input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .filter(|line| match part {
            Part1 => {
                line.matches(['a', 'e', 'i', 'o', 'u']).count() >= 3
                    && line.chars().tuple_windows().any(|(a, b)| a == b)
                    && ["ab", "cd", "pq", "xy"]
                        .into_iter()
                        .all(|s| !line.contains(s))
            }
            Part2 => {
                (2..line.len()).any(|k| line.matches(&line[k - 2..k]).count() >= 2)
                    && line.chars().tuple_windows().any(|(a, _, b)| a == b)
            }
        })
        .count())
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_15_05() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 236);
    assert_eq!(solver(Part2, INPUTS[0])?, 51);
    Ok(())
}
