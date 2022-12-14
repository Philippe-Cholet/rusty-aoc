use std::collections::VecDeque;

use itertools::iproduct;

use common::{Context, Part, Part2, Result};
use utils::{char10, neighbors, parse_to_grid};

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
    let mut queue = VecDeque::from([(0, 0)]);
    while let Some(loc) = queue.pop_front() {
        for (r, c) in neighbors(loc, nrows, ncols, false) {
            let new_risk = grid[r][c]
                + risks[loc.0][loc.1].context("The risk should exist for elements of the queue")?;
            if !matches!(risks[r][c], Some(old_risk) if old_risk <= new_risk) {
                risks[r][c].replace(new_risk);
                queue.push_back((r, c));
            }
        }
    }
    debug_assert!(risks.iter().flatten().all(Option::is_some));
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
    use common::Part1;
    assert_eq!(solver(Part1, INPUTS[0])?, "40");
    assert_eq!(solver(Part1, INPUTS[1])?, "656");
    assert_eq!(solver(Part2, INPUTS[0])?, "315");
    assert_eq!(solver(Part2, INPUTS[1])?, "2979");
    Ok(())
}
