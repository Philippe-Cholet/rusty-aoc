use std::collections::HashSet;

use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

const AZAZ: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Rucksack Reorganization
pub fn solver(part: Part, input: &str) -> Result<String> {
    let priority = |ch| Ok(AZAZ.find(ch).context("Not a-zA-Z")? + 1);
    let common_item = |h1: HashSet<_>, h2| {
        h1.intersection(&h2)
            .exactly_one()
            .ok()
            .copied()
            .context("The intersection is not of a single item")
    };
    Ok(match part {
        Part1 => input
            .lines()
            .map(|line| {
                let n = line.len();
                ensure!(n % 2 == 0, "A rucksack should have an even number of items");
                let h1 = line[0..n / 2].chars().collect();
                let h2 = line[n / 2..n].chars().collect();
                priority(common_item(h1, h2)?)
            })
            .ok_sum::<usize>()?,
        Part2 => input
            .lines()
            .chunks(3)
            .into_iter()
            .map(|chunk| {
                let (h1, h2, h3) = chunk
                    .map(|line| line.chars().collect::<HashSet<_>>())
                    .collect_tuple()
                    .context("Not a chunk of 3 lines")?;
                priority(common_item(h1, &h2 & &h3)?)
            })
            .ok_sum()?,
    }
    .to_string())
}

pub const INPUTS: [&str; 2] = [
    "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_03() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "157");
    assert_eq!(solver(Part1, INPUTS[1])?, "7997");
    assert_eq!(solver(Part2, INPUTS[0])?, "70");
    assert_eq!(solver(Part2, INPUTS[1])?, "2545");
    Ok(())
}
