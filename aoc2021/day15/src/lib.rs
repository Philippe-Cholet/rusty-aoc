use std::collections::{HashMap, VecDeque};

use itertools::iproduct;

use common::{Context, Part, Part2, Result};
use utils::{char10, neighbors, FromIterStr};

/// Chiton
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (grid, nrows, ncols) = {
        let mut grid = input.lines().parse_to_grid(char10::<u32>)?;
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
    let (start, end) = ((0, 0), (nrows - 1, ncols - 1));
    let mut risks = HashMap::from([(start, 0)]);
    let mut queue = VecDeque::from([start]);
    while let Some((r, c)) = queue.pop_front() {
        for rc in neighbors(r, c, nrows, ncols, false) {
            let new_risk = risks[&(r, c)] + grid[rc.0][rc.1];
            match risks.get_mut(&rc) {
                Some(risk) if *risk > new_risk => {
                    *risk = new_risk;
                    queue.push_back(rc);
                }
                None => {
                    risks.insert(rc, new_risk);
                    queue.push_back(rc);
                }
                _ => {}
            }
        }
    }
    debug_assert_eq!(risks.len(), nrows * ncols);
    Ok(risks[&end].to_string())
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
