use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

/// Sonar Sweep
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut v: Vec<u32> = input.lines().map(str::parse).ok_collect()?;
    if part == Part2 {
        v = v
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .collect();
    }
    let pairs = v.iter().tuple_windows();
    let res = pairs.filter(|(a, b)| a < b).count();
    Ok(res.to_string())
}

pub const INPUTS: [&str; 2] = [
    "199
200
208
210
200
207
240
269
260
263
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_01() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "7");
    assert_eq!(solver(Part1, INPUTS[1])?, "1665");
    assert_eq!(solver(Part2, INPUTS[0])?, "5");
    assert_eq!(solver(Part2, INPUTS[1])?, "1702");
    Ok(())
}
