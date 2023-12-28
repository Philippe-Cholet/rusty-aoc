use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

/// Sonar Sweep
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut v: Vec<u32> = input.lines().map(str::parse).ok_collect()?;
    if part.two() {
        v = v
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect();
    }
    let pairs = v.iter().tuple_windows();
    Ok(pairs.filter(|(a, b)| a < b).count())
}

test_solver! {
    "\
199
200
208
210
200
207
240
269
260
263
" => (7, 5),
    include_input!(21 01) => (1665, 1702),
}
