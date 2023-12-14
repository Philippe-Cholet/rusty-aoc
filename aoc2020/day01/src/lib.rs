use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

const TOTAL: u32 = 2020;

/// Report Repair
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let items = input.lines().map(str::parse::<u32>).ok_collect_hset()?;
    match part {
        Part1 => items.iter().find_map(|a| {
            let b = TOTAL.checked_sub(*a)?;
            items.contains(&b).then(|| a * b)
        }),
        Part2 => items.iter().tuple_combinations().find_map(|(a, b)| {
            let c = TOTAL.checked_sub(a + b)?;
            items.contains(&c).then(|| a * b * c)
        }),
    }
    .context("No solution")
}

pub const INPUTS: [&str; 2] = [
    "1721
979
366
299
675
1456
",
    include_input!(20 01),
];

#[test]
fn solver_20_01() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 514579); // 1721 * 299
    assert_eq!(solver(Part1, INPUTS[1])?, 1016964);
    assert_eq!(solver(Part2, INPUTS[0])?, 241861950); // 979 * 366 * 675
    assert_eq!(solver(Part2, INPUTS[1])?, 182588480);
    Ok(())
}
