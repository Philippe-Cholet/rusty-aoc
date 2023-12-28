use std::collections::VecDeque;

use common::prelude::*;
use crate::utils::{neighbors, parse_to_grid_with_loc};

#[derive(Debug, Clone)]
enum Cell {
    Rock,
    GardenPlot(Option<u32>),
}

#[derive(Debug)]
struct Garden {
    start: (usize, usize),
    grid: Vec<Vec<Cell>>,
}

/// Step Counter
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let mut garden: Garden = input.parse()?;
    garden.read_distances();
    Ok(match part {
        Part1 => garden.exact_steps_no_infinite(64),
        Part2 => garden.exact_steps(26_501_365),
    })
}

impl Garden {
    fn shape(&self) -> (usize, usize) {
        let nrows = self.grid.len();
        let ncols = self.grid[0].len();
        (nrows, ncols)
    }

    // Simple BFS
    fn read_distances(&mut self) {
        let (nrows, ncols) = self.shape();
        let mut queue = VecDeque::from([(0, self.start)]);
        while let Some((dist, (r, c))) = queue.pop_front() {
            if let Cell::GardenPlot(rc_dist @ None) = &mut self.grid[r][c] {
                *rc_dist = Some(dist);
                for (r0, c0) in neighbors((r, c), nrows, ncols, false) {
                    if matches!(self.grid[r0][c0], Cell::GardenPlot(None)) {
                        queue.push_back((dist + 1, (r0, c0)));
                    }
                }
            }
        }
    }

    fn exact_steps_no_infinite(&self, steps: u32) -> u64 {
        self.grid
            .iter()
            .flatten()
            .filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist <= steps && *dist % 2 == steps % 2))
            .count() as u64
    }

    #[allow(clippy::cast_possible_truncation)]
    fn exact_steps(&self, steps: u32) -> u64 {
        let size = self.shape().0;
        // The square grid has 4 corner zones:
        // +-----+
        // |  ^  |
        // | / \ |
        // |<   >|
        // | \ / |
        // |  v  |
        // +-----+
        let strictly_in_corner = |r, c| {
            usize::min(size - 1 - r, r) + c < (size - 1) / 2
                || usize::max(size - 1 - r, r) + c > 3 * (size - 1) / 2
        };
        let in_corner = |r, c| {
            usize::min(size - 1 - r, r) + c <= (size - 1) / 2
                || usize::max(size - 1 - r, r) + c >= 3 * (size - 1) / 2
        };
        // Count the plots accessible with an even/odd number of steps for the non-infinite whole grid.
        let whole_even = self
            .grid
            .iter()
            .flatten()
            .filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist % 2 == 0))
            .count() as u64;
        let whole_odd = self
            .grid
            .iter()
            .flatten()
            .filter(|cell| matches!(cell, Cell::GardenPlot(Some(dist)) if *dist % 2 == 1))
            .count() as u64;
        // The whole center square (3x3 with x) is repeated 1 (center) + 4 * 2k for k in 1..
        // Then another whole square (3x3 with o) is repeated 4 * (2k-1) for k in 1..
        // On the exterior of the diamond, (upper) squares are truncated.
        //           O
        //          OOO
        //         XOOOX
        //        OOxxxOO
        //       OOOxxxOOO
        //      XOOOxxxOOOX
        //     OOxxxoooxxxOO
        //    OOOxxxoooxxxOOO
        //   XOOOxxxoooxxxOOOX
        //  OOxxxoooxxxoooxxxOO
        // OOOxxxoooxSxoooxxxOOO
        //  OOxxxoooxxxoooxxxOO
        //   XOOOxxxoooxxxOOOX
        //    OOOxxxoooxxxOOO
        //     OOxxxoooxxxOO
        //      XOOOxxxOOOX
        //       OOOxxxOOO
        //        OOxxxOO
        //         XOOOX
        //          OOO
        //           O
        let (center, other) = if steps % 2 == 0 {
            (whole_even, whole_odd)
        } else {
            (whole_odd, whole_even)
        };
        let middle = self.start.0;
        let q = (steps - middle as u32) / size as u32;
        let r = (steps - middle as u32) % size as u32;
        assert_eq!(r, 0);
        center
            * (1 + 4
                * (2..)
                    .step_by(2)
                    .take_while(|k| *k < q)
                    .map(u64::from)
                    .sum::<u64>())
            + other
                * 4
                * (1..)
                    .step_by(2)
                    .take_while(|k| *k < q)
                    .map(u64::from)
                    .sum::<u64>()
            + self
                .grid
                .iter()
                .enumerate()
                .flat_map(|(r, col)| col.iter().enumerate().map(move |(c, cell)| ((r, c), cell)))
                .map(|((r, c), cell)| {
                    if let Cell::GardenPlot(Some(dist)) = cell {
                        if dist % 2 == q % 2 {
                            if in_corner(r, c) {
                                // X
                                u64::from(q)
                            } else {
                                0
                            }
                        } else {
                            //  OO        O
                            // OOO s and OOO s
                            // OOO       OOO
                            if strictly_in_corner(r, c) {
                                3 * u64::from(q - 1) + 2
                            } else {
                                4 * u64::from(q - 1) + 4
                            }
                        }
                    } else {
                        0
                    }
                })
                .sum::<u64>()
    }
}

impl std::str::FromStr for Garden {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let grid = parse_to_grid_with_loc(s.lines(), |loc, ch| match ch {
            '#' => Ok(Cell::Rock),
            '.' => Ok(Cell::GardenPlot(None)),
            'S' => {
                start = Some(loc);
                Ok(Cell::GardenPlot(None))
            }
            _ => bail!("Wrong char: {}", ch),
        })?;
        // TODO: ensure the grid is rectangular.
        let start = start.context("No start")?;
        Ok(Self { start, grid })
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => f.write_str(" ███")?,
            Self::GardenPlot(None) => f.write_str(" ░░░")?,
            Self::GardenPlot(Some(dist)) => write!(f, "{dist: >4}")?,
        }
        Ok(())
    }
}

impl std::fmt::Display for Garden {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{cell}")?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[test]
fn example() {
    let _s = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";
    let s = "\
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##..S####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
";
    let mut g: Garden = s.parse().unwrap();
    g.read_distances();
    assert_eq!(g.exact_steps_no_infinite(6), 16);
    assert_eq!(g.exact_steps_no_infinite(10), 50);
    // assert_eq!(g.exact_steps(6), 16);
    // assert_eq!(g.exact_steps(10), 50);
    // assert_eq!(g.exact_steps(50), 1_594);
    // assert_eq!(g.exact_steps(100), 6_536);
    // assert_eq!(g.exact_steps(500), 167_004);
    // assert_eq!(g.exact_steps(1_000), 668_697);
    // assert_eq!(g.exact_steps(5_000), 16_733_044);
}

test_solver!(include_input!(23 21) => (3591, 598_044_246_091_826));
