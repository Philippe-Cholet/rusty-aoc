use itertools::iproduct;

use common::prelude::*;
use crate::utils::{char10, neighbors, parse_to_grid};

/// Smoke Basin
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let grid = parse_to_grid(input.lines(), char10::<u32>)?;
    let ncols = grid.first().context("No line")?.len();
    let nrows = grid.len();
    let low_points = iproduct!(0..nrows, 0..ncols).filter(|(r, c)| {
        neighbors((*r, *c), nrows, ncols, false)
            .into_iter()
            .all(|(r1, c1)| grid[r1][c1] > grid[*r][*c])
    });
    Ok(match part {
        Part1 => low_points.map(|(r, c)| grid[r][c] as usize + 1).sum(),
        Part2 => {
            let mut areas: Vec<usize> = low_points
                .map(|low| {
                    let mut stack = vec![low];
                    let mut been = HashSet::new();
                    loop {
                        let Some((r, c)) = stack.pop() else {
                            break been.len();
                        };
                        if been.insert((r, c)) {
                            for (r1, c1) in neighbors((r, c), nrows, ncols, false) {
                                if grid[r1][c1] < 9 && !been.contains(&(r1, c1)) {
                                    stack.push((r1, c1));
                                }
                            }
                        }
                    }
                })
                .collect();
            areas.sort_unstable();
            areas.into_iter().rev().take(3).product()
        }
    })
}

test_solver! {
    "\
2199943210
3987894921
9856789892
8767896789
9899965678
" => (15, 1134), // 9 * 14 * 9
    include_input!(21 09) => (524, 1235430),
}
