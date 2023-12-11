use itertools::Itertools;

use common::prelude::*;

/// Cosmic Expansion
pub fn solver(part: Part, input: &str) -> Result<usize> {
    ensure!(
        input.chars().all(|ch| matches!(ch, '.' | '#' | '\n')),
        "Unexpected char"
    );
    let lights = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| line.chars().positions(|ch| ch == '#').map(move |c| (r, c)))
        .collect_vec();
    let (max_row, max_col) = lights
        .iter()
        .copied()
        .fold((0, 0), |(row, col), (r, c)| (row.max(r), col.max(c)));
    let mut empty_rows = vec![true; max_row + 1];
    let mut empty_cols = vec![true; max_col + 1];
    for &(r, c) in &lights {
        empty_rows[r] = false;
        empty_cols[c] = false;
    }
    let factor = part.value(2, 1_000_000) - 1;
    Ok(lights
        .iter()
        .copied()
        .tuple_combinations()
        .map(|((r0, c0), (r1, c1))| {
            // Manhattan distance + total expansion.
            let expansion = empty_rows[r0.min(r1)..r0.max(r1)]
                .iter()
                .chain(&empty_cols[c0.min(c1)..c0.max(c1)])
                .filter(|&&empty| empty)
                .count();
            r1.abs_diff(r0) + c1.abs_diff(c0) + expansion * factor
        })
        .sum())
}

pub const INPUTS: [&str; 2] = [
    "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
",
    include_str!("input.txt"),
];

#[test]
fn solver_23_11() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 374);
    assert_eq!(solver(Part1, INPUTS[1])?, 9403026);
    assert_eq!(solver(Part2, INPUTS[1])?, 543018317006);
    Ok(())
}
