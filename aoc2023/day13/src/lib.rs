use itertools::{Either, Itertools};

use common::prelude::*;

#[derive(Debug)]
/// Mirror grid (up to 32x32)
struct Grid {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

/// Point of Incidence
pub fn solver(part: Part, input: &str) -> Result<usize> {
    input
        .split("\n\n")
        .map(str::parse)
        .map_ok(|mut grid: Grid| {
            Ok(match part {
                Part1 => {
                    100 * grid.row_reflections().sum::<usize>()
                        + grid.col_reflections().sum::<usize>()
                }
                Part2 => match grid.find_smudge()? {
                    Either::Left(row) => 100 * row,
                    Either::Right(col) => col,
                },
            })
        })
        .flatten()
        .sum()
}

impl Grid {
    fn reflections(data: &[u32]) -> impl Iterator<Item = usize> + '_ {
        (1..data.len()).filter(|&idx| {
            data[..idx]
                .iter()
                .rev()
                .zip(&data[idx..])
                .all(|(before, after)| before == after)
        })
    }

    fn row_reflections(&self) -> impl Iterator<Item = usize> + '_ {
        Self::reflections(&self.rows)
    }

    fn col_reflections(&self) -> impl Iterator<Item = usize> + '_ {
        Self::reflections(&self.cols)
    }

    fn toggle(&mut self, r: usize, c: usize) {
        self.rows[r] ^= 1 << (self.cols.len() - 1 - c);
        self.cols[c] ^= 1 << (self.rows.len() - 1 - r);
    }

    fn find_smudge(&mut self) -> Result<Either<usize, usize>> {
        let old_rows = self.row_reflections().collect_vec();
        let old_cols = self.col_reflections().collect_vec();
        for r in 0..self.rows.len() {
            for c in 0..self.cols.len() {
                self.toggle(r, c);
                for idx in self.row_reflections() {
                    if !old_rows.contains(&idx) {
                        return Ok(Either::Left(idx));
                    }
                }
                for idx in self.col_reflections() {
                    if !old_cols.contains(&idx) {
                        return Ok(Either::Right(idx));
                    }
                }
                self.toggle(r, c);
            }
        }
        bail!("Failed to find the different line of reflection");
    }
}

impl std::str::FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let rows: Vec<_> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Ok(true),  // ash
                        '#' => Ok(false), // rock
                        _ => bail!("Wrong char: {}", ch),
                    })
                    .fold_ok(0, |acc, b| acc << 1 | u32::from(b))
            })
            .try_collect()?;
        let width = s.lines().map(str::len).max().context("Empty grid")?;
        let cols = (0..width)
            .rev()
            .map(|c| {
                rows.iter()
                    .fold(0, |acc, row| (acc << 1) | ((row >> c) & 1))
            })
            .collect();
        Ok(Self { rows, cols })
    }
}

// This visualization was helpful for debugging purposes.
impl std::fmt::Binary for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for slice in [&self.rows, &self.cols] {
            for data in slice {
                writeln!(f, "{data: >32b}")?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

pub const INPUTS: [&str; 2] = [
    "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
",
    include_str!("input.txt"),
];

#[test]
fn solver_23_13() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 405);
    assert_eq!(solver(Part1, INPUTS[1])?, 33735);
    assert_eq!(solver(Part2, INPUTS[0])?, 400);
    assert_eq!(solver(Part2, INPUTS[1])?, 38063);
    Ok(())
}
