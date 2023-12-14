use common::prelude::*;
use utils::parse_to_grid;

/// Toboggan Trajectory
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let data = parse_to_grid(input.lines(), |ch| match ch {
        '#' => Ok(true),
        '.' => Ok(false),
        _ => bail!("Wrong char: {}", ch),
    })?;
    let width = data.get(0).context("Empty grid")?.len();
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

pub const INPUTS: [&str; 2] = [
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
",
    include_input!(20 03),
];

#[test]
fn solver_20_03() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 7);
    assert_eq!(solver(Part1, INPUTS[1])?, 218);
    assert_eq!(solver(Part2, INPUTS[0])?, 336);
    assert_eq!(solver(Part2, INPUTS[1])?, 3847183340);
    Ok(())
}
