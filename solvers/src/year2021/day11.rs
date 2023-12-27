use common::prelude::*;
use crate::utils::{char10, neighbors, parse_to_grid};

/// Dumbo Octopus
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut grid = parse_to_grid(input.lines(), char10)?;
    let ncols = grid.first().context("Empty grid")?.len();
    let nrows = grid.len();
    ensure!(
        grid.iter().all(|row| row.len() == ncols),
        "The grid is not rectangular"
    );
    let mut nb_flashes = 0;
    let mut stack = vec![];
    let mut been = HashSet::new();
    let mut step = 0;
    Ok(loop {
        step += 1;
        debug_assert!(stack.is_empty() && been.is_empty());
        for (r, row) in grid.iter_mut().enumerate() {
            for (c, n) in row.iter_mut().enumerate() {
                *n += 1;
                if *n > 9 {
                    stack.push((r, c));
                }
            }
        }
        let nb_new_flashes = loop {
            let (r, c) = match stack.pop() {
                Some(u) if been.contains(&u) => continue,
                Some(u) => u,
                None => break been.len(),
            };
            been.insert((r, c));
            grid[r][c] = 0;
            for (r1, c1) in neighbors((r, c), nrows, ncols, true) {
                if !been.contains(&(r1, c1)) {
                    grid[r1][c1] += 1;
                    if grid[r1][c1] > 9 {
                        stack.push((r1, c1));
                    }
                }
            }
        };
        nb_flashes += nb_new_flashes;
        #[cfg(debug_assertions)]
        if step <= 10 || step % 10 == 0 || nb_new_flashes == nrows * ncols {
            println!("----------");
            for row in &grid {
                for n in row {
                    print!("{n}");
                }
                println!();
            }
            println!(
                "{nb_new_flashes:?} new flashes at step {step:?} (total of {nb_flashes:?} flashes)"
            );
        }
        match part {
            Part1 if step == 100 => break nb_flashes,
            Part2 if nb_new_flashes == nrows * ncols => break step,
            _ => {}
        }
        been.clear();
    })
}

pub const INPUTS: [&str; 2] = [
    "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
",
    include_input!(21 11),
];

#[test]
fn solver_21_11() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 1656);
    assert_eq!(solver(Part1, INPUTS[1])?, 1617);
    assert_eq!(solver(Part2, INPUTS[0])?, 195);
    assert_eq!(solver(Part2, INPUTS[1])?, 258);
    Ok(())
}
