use common::prelude::*;
use crate::utils::{OkIterator, SliceExt};

const DECRYPTION_KEY: i64 = 811_589_153;

/// Grove Positioning System
pub fn solver(part: Part, input: &str) -> Result<i64> {
    let mut file: Vec<i64> = input.lines().map(str::parse).ok_collect()?;
    let nb = file.len();
    // Otherwise, `nb - 1` would overflow or `rem_euclid(modulus)` below would panic.
    ensure!(nb >= 2, "Not enough numbers");
    if part.two() {
        file.iter_mut().for_each(|elem| *elem *= DECRYPTION_KEY);
    }
    let mut data: Vec<_> = file.into_iter().enumerate().collect();
    let modulus = i64::try_from(nb - 1)?;
    for _ in 0..part.value(1, 10) {
        for idx in 0..nb {
            let i0 = data
                .iter()
                .position(|(i, _)| i == &idx)
                .context("Missing index")?;
            let i1 = ((i64::try_from(i0)? + data[i0].1 - 1).rem_euclid(modulus) + 1).try_into()?;
            data.remove_insert(i0, i1);
        }
    }
    let i0 = data
        .iter()
        .position(|(_, v)| v == &0)
        .context("data does not contain 0")?;
    Ok(data[(i0 + 1000) % nb].1 + data[(i0 + 2000) % nb].1 + data[(i0 + 3000) % nb].1)
}

pub const INPUTS: [&str; 2] = ["1\n2\n-3\n3\n-2\n0\n4\n", include_input!(22 20)];

#[test]
fn solver_22_20() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[1])?, 988);
    assert_eq!(solver(Part2, INPUTS[1])?, 7768531372516);
    Ok(())
}
