use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

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

test_solver! {
    "16,1,2,0,4,2,7,1,2,14" => (37, 168),
    include_input!(21 07) => (356922, 100347031),
}
