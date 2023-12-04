use std::{fmt::Display, iter::repeat};

use itertools::iproduct;

use common::prelude::*;
use utils::{char10, parse_to_grid};

struct VisibleTrees<'a> {
    grid: &'a [Vec<u8>],
    nrows: usize,
    ncols: usize,
    // I first used a hashset instead of a grid of booleans,
    // but I use it in a quite basic way, it was not worth it.
    visible: Vec<Vec<bool>>,
}

impl<'a> VisibleTrees<'a> {
    fn new(grid: &'a [Vec<u8>]) -> Self {
        let (nrows, ncols) = (grid.len(), grid[0].len());
        Self {
            grid,
            nrows,
            ncols,
            visible: vec![vec![false; ncols]; nrows],
        }
    }

    fn process_locs<It>(&mut self, mut locs: It) -> Result<()>
    where
        It: Iterator<Item = (usize, usize)>,
    {
        let (r, c) = locs.next().context("Empty line")?;
        self.visible[r][c] = true;
        let mut h_max = self.grid[r][c];
        for (r, c) in locs {
            let h = self.grid[r][c];
            if h > h_max {
                self.visible[r][c] = true;
                if h == 9 {
                    break;
                }
                h_max = h;
            }
        }
        Ok(())
    }

    fn process(&mut self) -> Result<()> {
        for r in 0..self.nrows {
            self.process_locs(repeat(r).zip(0..self.ncols))?;
            self.process_locs(repeat(r).zip((0..self.ncols).rev()))?;
        }
        for c in 0..self.ncols {
            self.process_locs((0..self.nrows).zip(repeat(c)))?;
            self.process_locs((0..self.nrows).rev().zip(repeat(c)))?;
        }
        Ok(())
    }

    fn get_count(&self) -> usize {
        self.visible.iter().flatten().filter(|x| **x).count()
    }
}

impl Display for VisibleTrees<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.visible {
            for vis in row {
                write!(f, "{}", if *vis { 'x' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct TreeHouse<'a> {
    grid: &'a [Vec<u8>],
    nrows: usize,
    ncols: usize,
}

impl<'a> TreeHouse<'a> {
    fn new(grid: &'a [Vec<u8>]) -> Self {
        Self {
            grid,
            nrows: grid.len(),
            ncols: grid[0].len(),
        }
    }

    fn visibles_in<It>(&self, height: u8, locs: It) -> usize
    where
        It: Iterator<Item = (usize, usize)>,
    {
        let mut visibles = 0;
        for (r, c) in locs {
            visibles += 1;
            if self.grid[r][c] >= height {
                break;
            }
        }
        visibles
    }

    fn scenic_score(&self, r: usize, c: usize) -> usize {
        let height = self.grid[r][c];
        self.visibles_in(height, (0..r).rev().zip(repeat(c)))
            * self.visibles_in(height, (r + 1..self.nrows).zip(repeat(c)))
            * self.visibles_in(height, repeat(r).zip((0..c).rev()))
            * self.visibles_in(height, repeat(r).zip(c + 1..self.ncols))
    }

    fn best_scenic_score(&self) -> usize {
        // Trees at the border have a scenic score of 0, ignore them.
        iproduct!(1..self.nrows - 1, 1..self.ncols - 1)
            .map(|(r, c)| self.scenic_score(r, c))
            .max()
            .unwrap_or(0)
    }
}

/// Treetop Tree House
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let grid = parse_to_grid(input.lines(), char10::<u8>)?;
    Ok(match part {
        Part1 => {
            let mut vis_trees = VisibleTrees::new(&grid);
            vis_trees.process()?;
            // println!("{}", vis_trees);
            vis_trees.get_count()
        }
        Part2 => TreeHouse::new(&grid).best_scenic_score(),
    })
}

pub const INPUTS: [&str; 2] = [
    "30373
25512
65332
33549
35390
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_08() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 21);
    assert_eq!(solver(Part1, INPUTS[1])?, 1816);
    assert_eq!(solver(Part2, INPUTS[0])?, 8);
    assert_eq!(solver(Part2, INPUTS[1])?, 383520);
    Ok(())
}

/*
use std::iter::repeat;

use itertools::iproduct;

use common::prelude::*;
use utils::{char10, parse_to_grid};

/// Treetop Tree House
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let grid = parse_to_grid(input.lines(), char10::<u8>)?;
    let (nrows, ncols) = (grid.len(), grid[0].len());
    Ok(match part {
        Part1 => {
            // I first used a hashset instead of a grid of booleans, but this is quite basic.
            let mut visible = vec![vec![false; ncols]; nrows];
            macro_rules! add_visible_trees {
                ($locs: expr) => {
                    let (r, c) = $locs.next().expect("Empty line");
                    visible[r][c] = true;
                    let mut h_max = grid[r][c];
                    for (r, c) in $locs {
                        let h = grid[r][c];
                        if h > h_max {
                            visible[r][c] = true;
                            if h == 9 {
                                break;
                            }
                            h_max = h;
                        }
                    }
                };
            }
            for r in 0..nrows {
                add_visible_trees!(repeat(r).zip(0..ncols));
                add_visible_trees!(repeat(r).zip((0..ncols).rev()));
            }
            for c in 0..ncols {
                add_visible_trees!((0..nrows).zip(repeat(c)));
                add_visible_trees!((0..nrows).rev().zip(repeat(c)));
            }
            // for row in &visible {
            //     for vis in row {
            //         print!("{}", if *vis { 'x' } else { '.' });
            //     }
            //     println!();
            // }
            visible.into_iter().flatten().filter(|x| *x).count()
        }
        Part2 => {
            macro_rules! direction_score {
                ($locs: expr, $height: ident) => {{
                    let mut n = 0;
                    for (r, c) in $locs {
                        n += 1;
                        if grid[r][c] >= $height {
                            break;
                        }
                    }
                    n
                }};
            }
            // Trees at the border have a scenic score of 0, ignore them.
            iproduct!(1..nrows - 1, 1..ncols - 1)
                .map(|(r, c)| {
                    let height = grid[r][c];
                    direction_score!((0..r).rev().zip(repeat(c)), height)
                        * direction_score!((r + 1..nrows).zip(repeat(c)), height)
                        * direction_score!(repeat(r).zip((0..c).rev()), height)
                        * direction_score!(repeat(r).zip(c + 1..ncols), height)
                })
                .max()
                .unwrap_or(0)
        }
    })
}
*/
