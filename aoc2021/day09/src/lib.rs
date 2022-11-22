use std::collections::HashSet;

use itertools::iproduct;

use common::{Context, Part, Part1, Part2, Result};
use utils::{neighbors, InputParser};

/// Smoke Basin
pub fn solver(part: Part, input: &str) -> Result<String> {
    let grid = InputParser(input).grid(|ch| ch.to_digit(10).context("Not decimal"))?;
    let ncols = grid.first().context("No line")?.len();
    let nrows = grid.len();
    let low_points = iproduct!(0..nrows, 0..ncols).filter(|(r, c)| {
        neighbors(*r, *c, nrows, ncols, false)
            .into_iter()
            .all(|(r1, c1)| grid[r1][c1] > grid[*r][*c])
    });
    match part {
        Part1 => {
            let result: u32 = low_points.map(|(r, c)| grid[r][c] + 1).sum();
            Ok(result.to_string())
        }
        Part2 => {
            let mut areas: Vec<usize> = low_points
                .map(|low| {
                    let mut stack = vec![low];
                    let mut been = HashSet::new();
                    loop {
                        let (r, c) = match stack.pop() {
                            Some(u) => u,
                            None => break been.len(),
                        };
                        if been.insert((r, c)) {
                            for (r1, c1) in neighbors(r, c, nrows, ncols, false) {
                                if grid[r1][c1] < 9 && !been.contains(&(r1, c1)) {
                                    stack.push((r1, c1));
                                }
                            }
                        }
                    }
                })
                .collect();
            areas.sort_unstable();
            let result: usize = areas.into_iter().rev().take(3).product();
            Ok(result.to_string())
        }
    }
}

pub const INPUTS: [&str; 2] = [
    "2199943210
3987894921
9856789892
8767896789
9899965678
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_09() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "15");
    assert_eq!(solver(Part1, INPUTS[1])?, "524");
    assert_eq!(solver(Part2, INPUTS[0])?, "1134"); // 9 * 14 * 9
    assert_eq!(solver(Part2, INPUTS[1])?, "1235430");
    Ok(())
}
