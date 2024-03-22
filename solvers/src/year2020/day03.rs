use common::prelude::*;
use crate::utils::parse_to_grid;

/// Toboggan Trajectory
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let data = parse_to_grid(input.lines(), |ch| match ch {
        '#' => Ok(true),
        '.' => Ok(false),
        _ => bail!("Wrong char: {}", ch),
    })?;
    let width = data.first().context("Empty grid")?.len();
    ensure!(
        data.iter().all(|row| row.len() == width),
        "The grid is not rectangular",
    );
    let count_trees = |(right, down)| {
        data.iter()
            .step_by(down)
            .enumerate()
            .filter(|(idx, row)| row[(idx * right) % width])
            .count()
    };
    Ok(match part {
        Part1 => count_trees((3, 1)),
        Part2 => [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .map(count_trees)
            .into_iter()
            .product(),
    })
}

test_solver! {
    "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
" => (7, 336),
    include_input!(20 03) => (218, 3847183340),
}
