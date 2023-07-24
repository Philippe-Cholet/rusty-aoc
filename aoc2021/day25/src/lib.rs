use common::prelude::*;
use utils::parse_to_grid;

use SeaCucumberKind::{East, South};

#[derive(Debug, Clone, PartialEq)]
enum SeaCucumberKind {
    East,
    South,
}

/// Sea Cucumber
pub fn solver(part: Part, input: &str) -> Result<String> {
    if part.two() {
        return Ok(SUCCESS.to_owned());
    }
    let mut grid = parse_to_grid(input.lines(), |ch| {
        Ok(match ch {
            '.' => None,
            'v' => Some(South),
            '>' => Some(East),
            _ => bail!("Wrong char: {}", ch),
        })
    })?;
    let nrows = grid.len();
    let ncols = grid.first().context("Empty grid")?.len();
    ensure!(
        grid.iter().all(|row| row.len() == ncols),
        "The grid is not rectangular"
    );
    let mut steps = 0;
    loop {
        steps += 1;
        let mut nb_changes = 0;
        // println!("Step #{steps}");
        // display_grid(&grid);
        for direction in [East, South] {
            let mut new_grid = vec![vec![None; ncols]; nrows];
            for (r, row) in grid.iter().enumerate() {
                for (c, cucumber) in row.iter().enumerate() {
                    let (r2, c2) = match cucumber {
                        None => continue,
                        Some(dir) if dir != &direction => (r, c),
                        _ => {
                            let (r1, c1) = match direction {
                                East => (r, (c + 1) % ncols),
                                South => ((r + 1) % nrows, c),
                            };
                            if grid[r1][c1].is_none() {
                                nb_changes += 1;
                                (r1, c1)
                            } else {
                                (r, c)
                            }
                        }
                    };
                    new_grid[r2][c2] = cucumber.clone();
                }
            }
            grid = new_grid;
        }
        if nb_changes == 0 {
            display_grid(&grid);
            break;
        }
    }
    Ok(steps.to_string())
}

fn display_grid(grid: &Vec<Vec<Option<SeaCucumberKind>>>) {
    if !cfg!(debug_assertions) {
        return;
    }
    for row in grid {
        for c in row {
            let ch = match c {
                None => '.',
                Some(East) => '>',
                Some(South) => 'v',
            };
            print!("{ch}");
        }
        println!();
    }
    println!();
}

pub const INPUTS: [&str; 2] = [
    "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
",
    include_str!("input.txt"),
];

const SUCCESS: &str = "\
The sleigh keys are detected directly under the submarine.
And you remotely start the sleigh.
You can now go back to the surface.
It's Christmas! Congratulations!";

#[test]
fn solver_21_25() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "58");
    assert_eq!(solver(Part1, INPUTS[1])?, "549");
    assert_eq!(solver(Part2, INPUTS[0])?, SUCCESS);
    assert_eq!(solver(Part2, INPUTS[1])?, SUCCESS);
    Ok(())
}
