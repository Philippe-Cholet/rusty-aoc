use std::str::FromStr;

use itertools::Itertools;

use common::prelude::*;
use crate::utils::{neighbors, parse_to_grid_with_loc};

/// Pipe Maze
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut grid: PipeGrid = input.parse()?;
    match part {
        Part1 => {
            let loop_length = grid.detect_loop()?;
            ensure!(loop_length % 2 == 0, "What pipe loop is this?!");
            Ok(loop_length / 2)
        }
        Part2 => grid.detect_interior(),
    }
}

const GROUND: u8 = 0;
const NORTH: u8 = 1 << 0;
const SOUTH: u8 = 1 << 1;
const WEST: u8 = 1 << 2;
const EAST: u8 = 1 << 3;
const PIPE: u8 = NORTH | SOUTH | WEST | EAST;
const LOOP: u8 = 1 << 4;

#[derive(Debug)]
struct PipeGrid {
    nrows: usize,
    ncols: usize,
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    start_flags: (u8, u8),
}

impl FromStr for PipeGrid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let mut grid = parse_to_grid_with_loc(s.lines(), |loc, ch| match ch {
            '|' => Ok(NORTH | SOUTH),
            '-' => Ok(EAST | WEST),
            'L' => Ok(NORTH | EAST),
            'J' => Ok(NORTH | WEST),
            '7' => Ok(SOUTH | WEST),
            'F' => Ok(SOUTH | EAST),
            '.' => Ok(GROUND),
            'S' => {
                ensure!(start.is_none(), "Multiple starts");
                start = Some(loc);
                Ok(GROUND)
            }
            _ => bail!("Wrong char: {}", ch),
        })?;
        // TODO: ensure it's rectangular.
        let start = start.context("No start")?;
        let (r, c) = start;
        // Detect pipe flags of the start:
        let start_flags = {
            if let Some(r0) = r.checked_sub(1) {
                if grid[r0][c] & SOUTH != 0 {
                    grid[r][c] |= NORTH;
                }
            }
            if let Some(c0) = c.checked_sub(1) {
                if grid[r][c0] & EAST != 0 {
                    grid[r][c] |= WEST;
                }
            }
            if let Some(col) = grid.get(r + 1) {
                if col[c] & NORTH != 0 {
                    grid[r][c] |= SOUTH;
                }
            }
            if let Some(&cell) = grid[r].get(c + 1) {
                if cell & WEST != 0 {
                    grid[r][c] |= EAST;
                }
            }
            #[cfg(debug_assertions)]
            println!("start_pipes = {:b}", grid[r][c]);
            [NORTH, EAST, SOUTH, WEST]
                .into_iter()
                .filter(|flag| grid[r][c] & *flag != 0)
                .collect_tuple()
                .context("The start pipe, on the loop, should go in 2 directions")?
        };
        Ok(Self {
            nrows: grid.len(),
            ncols: grid[0].len(),
            grid,
            start,
            start_flags,
        })
    }
}

impl PipeGrid {
    /// Mark grid cells with LOOP flags and return the length of loop.
    fn detect_loop(&mut self) -> Result<usize> {
        let mut nb_steps = 0;
        let mut flag = self.start_flags.0; // or `.1`, it should not matter!
        let (mut r, mut c) = self.start;
        // I just follow the PIPE flags and add LOOP ones until we find start again.
        loop {
            nb_steps += 1;
            self.grid[r][c] |= LOOP;
            let opp_flag = match flag {
                NORTH => {
                    r -= 1;
                    SOUTH
                }
                SOUTH => {
                    r += 1;
                    NORTH
                }
                WEST => {
                    c -= 1;
                    EAST
                }
                EAST => {
                    c += 1;
                    WEST
                }
                _ => unreachable!(),
            };
            let pipe_flags = self.grid[r][c] & PIPE;
            ensure!(pipe_flags & opp_flag != 0);
            ensure!(pipe_flags.count_ones() == 2);
            flag = pipe_flags ^ opp_flag;
            if (r, c) == self.start {
                break Ok(nb_steps);
            }
        }
    }

    /// Detect the loop and then the cells enclosed by it. Return the count of them.
    fn detect_interior(&mut self) -> Result<usize> {
        self.detect_loop()?;
        // Zoom on the grid: add a little space between cells of the grid.
        // And cells on the loop fill that space.
        // That way, all outside is clearly connected.
        let ext_nrows = 2 * self.nrows + 1;
        let ext_ncols = 2 * self.ncols + 1;
        // It's a big vector (78961 for me) but at least it's not a nested one like `grid`.
        let mut extended_grid = vec![true; ext_nrows * ext_ncols];
        for r in 0..self.nrows {
            for c in 0..self.ncols {
                let cell = self.grid[r][c];
                if cell & LOOP != 0 {
                    extended_grid[(2 * r + 1) * ext_ncols + 2 * c + 1] = false;
                    if cell & NORTH != 0 {
                        extended_grid[(2 * r) * ext_ncols + 2 * c + 1] = false;
                    }
                    if cell & SOUTH != 0 {
                        extended_grid[(2 * r + 2) * ext_ncols + 2 * c + 1] = false;
                    }
                    if cell & WEST != 0 {
                        extended_grid[(2 * r + 1) * ext_ncols + 2 * c] = false;
                    }
                    if cell & EAST != 0 {
                        extended_grid[(2 * r + 1) * ext_ncols + 2 * c + 2] = false;
                    }
                }
            }
        }
        // We added an empty border, so one BFS from (0, 0) is enough to detect the outside.
        let mut queue = vec![(0, 0)];
        while let Some((r, c)) = queue.pop() {
            if extended_grid[r * ext_ncols + c] {
                extended_grid[r * ext_ncols + c] = false;
                queue.extend(
                    neighbors((r, c), ext_nrows, ext_ncols, false)
                        .into_iter()
                        .filter(|&(r, c)| extended_grid[r * ext_ncols + c]),
                );
            }
        }
        Ok((0..self.nrows)
            .cartesian_product(0..self.ncols)
            .filter(|&(r, c)| extended_grid[(2 * r + 1) * ext_ncols + 2 * c + 1])
            .count())
    }
}

test_solver! {
    "\
.....
.S-7.
.|.|.
.L-J.
.....
" => 4,
    "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
" => 8,
    "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
" => ((), 4),
    "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
" => ((), 8),
    "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
" => ((), 10),
    include_input!(23 10) => (6800, 483),
}
