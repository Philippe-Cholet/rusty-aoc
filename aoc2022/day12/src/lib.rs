use std::collections::VecDeque;

use itertools::iproduct;

use common::prelude::*;
use utils::{neighbors, parse_to_grid_with_loc};

const AZ_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";

/// Hill Climbing Algorithm
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let (mut start, mut end) = (None, None);
    let grid = parse_to_grid_with_loc(input.lines(), |loc, mut ch| {
        match ch {
            'S' => (ch, start) = ('a', Some(loc)),
            'E' => (ch, end) = ('z', Some(loc)),
            _ => {}
        }
        AZ_LOWER.find(ch).context("Not a-z")
    })?;
    let (start, end) = (start.context("No start")?, end.context("No end")?);
    let (nrows, ncols) = (grid.len(), grid[0].len());
    // Going backward, find start(s) from the end.
    // A "previous location" `(r0, c0)` from a "next location" `(r1, c1)`.
    let mut frontier = VecDeque::from([end]);
    let mut dist2end = vec![vec![None; ncols]; nrows];
    dist2end[end.0][end.1] = Some(0u32);
    while let Some((r1, c1)) = frontier.pop_front() {
        let dist = dist2end[r1][c1].context("A frontier element has a distance")?;
        for (r0, c0) in neighbors((r1, c1), nrows, ncols, false) {
            if grid[r1][c1] <= grid[r0][c0] + 1 && dist2end[r0][c0].is_none() {
                if part.one() && (r0, c0) == start {
                    return Ok(dist + 1);
                }
                dist2end[r0][c0] = Some(dist + 1);
                frontier.push_back((r0, c0));
            }
        }
    }
    ensure!(part.two(), "Failed part 1");
    iproduct!(0..nrows, 0..ncols)
        .filter_map(|(r, c)| {
            if grid[r][c] == 0 {
                dist2end[r][c]
            } else {
                None
            }
        })
        .min()
        .context("No start?!")
}

pub const INPUTS: [&str; 2] = [
    "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_12() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 31);
    assert_eq!(solver(Part1, INPUTS[1])?, 449);
    assert_eq!(solver(Part2, INPUTS[0])?, 29);
    assert_eq!(solver(Part2, INPUTS[1])?, 443);
    Ok(())
}
