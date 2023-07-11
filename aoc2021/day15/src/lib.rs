use std::collections::BinaryHeap;

use itertools::iproduct;

use common::prelude::*;
use utils::{char10, neighbors, parse_to_grid, HeuristicItem};

/// Chiton
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (grid, nrows, ncols) = {
        let mut grid = parse_to_grid(input.lines(), char10::<u32>)?;
        let mut nrows = grid.len();
        let mut ncols = grid.first().context("Empty grid")?.len();
        // TODO: Eventually check if the grid is rectangular.
        if part == Part2 {
            let extension = |row: &[_], k: u32| -> Vec<_> {
                row.iter().map(|n| (*n + k - 1) % 9 + 1).collect()
            };
            for row in &mut grid {
                for k in 1..5 {
                    row.extend(extension(&row[0..ncols], k));
                }
            }
            grid.extend(
                iproduct!(1..5, 0..nrows)
                    .map(|(k, r)| extension(&grid[r], k))
                    .collect::<Vec<_>>(),
            );
            nrows *= 5;
            ncols *= 5;
        }
        (grid, nrows, ncols)
    }; // now immutables
    let mut risks = vec![vec![None; ncols]; nrows];
    risks[0][0] = Some(0);
    let mut heap = BinaryHeap::from([HeuristicItem::rev(0, (0, 0))]);
    let end = (nrows - 1, ncols - 1);
    // Minimize the risk to the end.
    while let Some(HeuristicItem { item: loc, .. }) = heap.pop() {
        if loc == end {
            break; // early exit
        }
        for (r, c) in neighbors(loc, nrows, ncols, false) {
            let new_risk = grid[r][c]
                + risks[loc.0][loc.1].context("The risk should exist for elements of the heap")?;
            if !matches!(risks[r][c], Some(old_risk) if old_risk <= new_risk) {
                risks[r][c].replace(new_risk);
                heap.push(HeuristicItem::rev(
                    // The manhattan distance to the end is a lower bound of the remaining risk.
                    new_risk as usize + end.0 - r + end.1 - c,
                    (r, c),
                ));
            }
        }
    }
    Ok(risks[nrows - 1][ncols - 1]
        .context("Did not reach the end")?
        .to_string())
}

pub const INPUTS: [&str; 2] = [
    "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_15() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "40");
    assert_eq!(solver(Part1, INPUTS[1])?, "656");
    assert_eq!(solver(Part2, INPUTS[0])?, "315");
    assert_eq!(solver(Part2, INPUTS[1])?, "2979");
    Ok(())
}
