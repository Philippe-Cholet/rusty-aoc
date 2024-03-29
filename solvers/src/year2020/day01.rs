use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

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

test_solver! {
    "1721
979
366
299
675
1456
" => (514579, 241861950), // 1721 * 299, 979 * 366 * 675
    include_input!(20 01) => (1016964, 182588480),
}
