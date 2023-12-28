use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

/// Encoding Error
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let xmas: Vec<u64> = input.lines().map(str::parse).ok_collect()?;
    let invalid_idx = get_invalid(&xmas, 25).context("No invalid number")?;
    match part {
        Part1 => Ok(xmas[invalid_idx]),
        Part2 => get_contiguous_set(&xmas, invalid_idx)
            .context("No contiguous set that sums to the invalid number"),
    }
}

fn get_invalid(xmas: &[u64], n: usize) -> Option<usize> {
    xmas.windows(n)
        .zip(&xmas[n..])
        .position(|(preamble, &last)| {
            preamble
                .iter()
                .tuple_combinations()
                .all(|(a, b)| a + b != last)
        })
        .map(|idx| idx + n)
}

#[allow(clippy::expect_used)]
fn get_contiguous_set(xmas: &[u64], invalid_idx: usize) -> Option<u64> {
    // For a given "start", there is a point where any "end" would result in a sum larger than "invalid".
    // Cut the loops would probably be a bit faster but this is already fast enough.
    let invalid = xmas[invalid_idx];
    (0..invalid_idx)
        .tuple_combinations()
        .find_map(|(start, end)| {
            let set = &xmas[start..=end];
            (invalid == set.iter().sum::<u64>()).then(|| {
                let (mini, maxi) = set.iter().minmax().into_option().expect("can not be empty");
                mini + maxi
            })
        })
}

test_solver!(include_input!(20 09) => (10884537, 1261309));

#[test]
#[ignore]
fn small_example() -> Result<()> {
    let data = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    let idx = get_invalid(&data, 5).context("No invalid")?;
    assert_eq!(data[idx], 127);
    assert_eq!(get_contiguous_set(&data, idx), Some(15 + 47));
    Ok(())
}
