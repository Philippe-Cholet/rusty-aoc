use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

/// The Treachery of Whales
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let ns: Vec<u32> = input.trim_end().split(',').map(str::parse).ok_collect()?;
    let (&min_n, &max_n) = ns.iter().minmax().into_option().context("empty")?;
    (min_n..=max_n)
        .map(|h| {
            ns.iter()
                .map(|n| {
                    let diff = h.abs_diff(*n);
                    match part {
                        Part1 => diff,
                        Part2 => diff * (diff + 1) / 2,
                    }
                })
                .sum()
        })
        .min()
        .context("empty")
}

pub const INPUTS: [&str; 2] = ["16,1,2,0,4,2,7,1,2,14\n", include_input!(21 07)];

#[test]
fn solver_21_07() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 37);
    assert_eq!(solver(Part1, INPUTS[1])?, 356922);
    assert_eq!(solver(Part2, INPUTS[0])?, 168);
    assert_eq!(solver(Part2, INPUTS[1])?, 100347031);
    Ok(())
}
