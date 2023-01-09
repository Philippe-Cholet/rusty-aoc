use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    iter::repeat,
};

use common::{bail, Context, Error, Part, Part1, Part2, Result};
use utils::{parse_to_grid, HeuristicItem};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const ALL: [Option<Self>; 5] = [
        Some(Self::Down),
        Some(Self::Right),
        Some(Self::Up),
        Some(Self::Left),
        None,
    ];

    const fn get(self, r: usize, c: usize) -> Option<(usize, usize)> {
        Some(match self {
            Self::Up if r > 0 => (r - 1, c),
            Self::Down => (r + 1, c),
            Self::Left if c > 0 => (r, c - 1),
            Self::Right => (r, c + 1),
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
enum Cell {
    Wall,
    Blizzard(Option<Direction>),
}

#[derive(Debug)]
struct BlizzardGrid {
    time_grid: [Vec<Vec<Vec<bool>>>; 2],
    nrows: usize,
    ncols: usize,
    start: (usize, usize),
    goal: (usize, usize),
}

impl std::str::FromStr for BlizzardGrid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let initial_grid = parse_to_grid(s.lines(), |ch| match ch {
            '#' => Ok(Cell::Wall),
            '.' => Ok(Cell::Blizzard(None)),
            '<' => Ok(Cell::Blizzard(Some(Direction::Left))),
            '>' => Ok(Cell::Blizzard(Some(Direction::Right))),
            '^' => Ok(Cell::Blizzard(Some(Direction::Up))),
            'v' => Ok(Cell::Blizzard(Some(Direction::Down))),
            _ => bail!("Wrong char: {}", ch),
        })?;
        let nrows = initial_grid.len();
        let ncols = initial_grid[0].len();
        let mut time_grid = [
            vec![vec![vec![true; nrows - 2]; ncols - 2]; nrows - 2],
            vec![vec![vec![true; ncols - 2]; ncols - 2]; nrows - 2],
        ];
        for (r, row) in initial_grid.into_iter().enumerate() {
            for (c, cell) in row.into_iter().enumerate() {
                if 0 < r && 0 < c && r < nrows - 1 && c < ncols - 1 {
                    if let Cell::Blizzard(Some(d)) = cell {
                        let path: Vec<_> = match d {
                            Direction::Right => repeat(r).zip((c..ncols - 1).chain(1..c)).collect(),
                            Direction::Left => repeat(r)
                                .zip((1..=c).rev().chain((c + 1..ncols - 1).rev()))
                                .collect(),
                            Direction::Down => (r..nrows - 1).chain(1..r).zip(repeat(c)).collect(),
                            Direction::Up => (1..=r)
                                .rev()
                                .chain((r + 1..nrows - 1).rev())
                                .zip(repeat(c))
                                .collect(),
                        };
                        let grid = match d {
                            Direction::Down | Direction::Up => &mut time_grid[0],
                            Direction::Left | Direction::Right => &mut time_grid[1],
                        };
                        for (minute, (r, c)) in path.into_iter().enumerate() {
                            grid[r - 1][c - 1][minute] = false;
                        }
                    }
                }
            }
        }
        Ok(Self {
            time_grid,
            nrows,
            ncols,
            start: (0, 1),
            goal: (nrows - 1, ncols - 2),
        })
    }
}

impl BlizzardGrid {
    fn blizzard_free(&self, loc: (usize, usize), minutes: usize) -> bool {
        0 < loc.0
            && 0 < loc.1
            && loc.0 < self.nrows - 1
            && loc.1 < self.ncols - 1
            && self.time_grid[0][loc.0 - 1][loc.1 - 1][minutes % (self.nrows - 2)]
            && self.time_grid[1][loc.0 - 1][loc.1 - 1][minutes % (self.ncols - 2)]
    }

    const fn dist2goal(&self, loc: (usize, usize)) -> usize {
        self.goal.0.abs_diff(loc.0) + self.goal.1.abs_diff(loc.1)
    }

    fn find_path(&self, starting_minute: usize) -> Result<usize> {
        let mut heap = BinaryHeap::from([HeuristicItem {
            heuristic: Reverse(0), // This first heuristic does not matter.
            item: (self.start, starting_minute),
        }]);
        let mut been = HashSet::new();
        Ok(loop {
            let (loc, mut minutes) = heap.pop().context("Should be impossible")?.item;
            if loc == self.goal {
                break minutes;
            }
            if !been.insert((loc, minutes)) {
                continue;
            }
            minutes += 1;
            for opt_dir in Direction::ALL {
                let loc2 = opt_dir.map_or(Some(loc), |d| d.get(loc.0, loc.1));
                let Some(loc2) = loc2 else { continue; };
                if loc2 == self.start || loc2 == self.goal || self.blizzard_free(loc2, minutes) {
                    heap.push(HeuristicItem {
                        heuristic: Reverse(minutes + self.dist2goal(loc2)),
                        item: (loc2, minutes),
                    });
                }
            }
        })
    }

    fn find_multi_path(&mut self, get_back_times: usize) -> Result<usize> {
        let mut t = 0;
        for _ in 0..get_back_times * 2 {
            t = self.find_path(t)?;
            (self.start, self.goal) = (self.goal, self.start);
        }
        self.find_path(t)
    }
}

/// Blizzard Basin
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut bgrid: BlizzardGrid = input.parse()?;
    let times = match part {
        Part1 => 0,
        Part2 => 1,
    };
    Ok(bgrid.find_multi_path(times)?.to_string())
}

pub const INPUTS: [&str; 2] = [
    "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_24() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "18");
    assert_eq!(solver(Part1, INPUTS[1])?, "279");
    assert_eq!(solver(Part2, INPUTS[0])?, "54");
    assert_eq!(solver(Part2, INPUTS[1])?, "762");
    Ok(())
}
