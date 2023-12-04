use itertools::Itertools;

use common::prelude::*;
use utils::parse_to_grid;

use Direction::{E, N, S, W};

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}

const DIRS: [[Direction; 4]; 4] = [[N, S, W, E], [S, W, E, N], [W, E, N, S], [E, N, S, W]];

impl Direction {
    const fn opposite(self) -> Self {
        match self {
            N => S,
            S => N,
            W => E,
            E => W,
        }
    }
    const fn bitmask(self) -> u8 {
        match self {
            N => 1,
            S => 2,
            W => 4,
            E => 8,
        }
    }
    const fn from_bitmask(mask: u8) -> Option<Self> {
        match mask {
            1 => Some(N),
            2 => Some(S),
            4 => Some(W),
            8 => Some(E),
            _ => None,
        }
    }
    const fn get_position(self, r: usize, c: usize) -> (usize, usize) {
        match self {
            N => (r - 1, c),
            S => (r + 1, c),
            W => (r, c - 1),
            E => (r, c + 1),
        }
    }
    const fn adjacent_positions(self, r: usize, c: usize) -> [(usize, usize); 3] {
        match self {
            N => [(r - 1, c - 1), (r - 1, c), (r - 1, c + 1)],
            S => [(r + 1, c - 1), (r + 1, c), (r + 1, c + 1)],
            W => [(r - 1, c - 1), (r, c - 1), (r + 1, c - 1)],
            E => [(r - 1, c + 1), (r, c + 1), (r + 1, c + 1)],
        }
    }
}

fn enlarge(
    grid: &mut Vec<Vec<bool>>,
    nrows: &mut usize,
    ncols: &mut usize,
    [r0, r1, c0, c1]: [bool; 4],
) {
    if c0 {
        *ncols += 1;
    }
    if c1 {
        *ncols += 1;
    }
    for row in grid.iter_mut() {
        if c0 {
            row.insert(0, false);
        }
        if c1 {
            row.push(false);
        }
    }
    if r0 {
        grid.insert(0, vec![false; *ncols]);
        *nrows += 1;
    }
    if r1 {
        grid.push(vec![false; *ncols]);
        *nrows += 1;
    }
}

fn all_empty(grid: &[Vec<bool>], locs: &[(usize, usize)]) -> bool {
    locs.iter().all(|(r, c)| !grid[*r][*c])
}

fn display_grid(grid: &[Vec<bool>]) {
    if !cfg!(debug_assertions) {
        return;
    }
    for row in grid {
        for b in row {
            print!("{}", if *b { '#' } else { '.' });
        }
        println!();
    }
}

/// Unstable Diffusion
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut grid = parse_to_grid(input.lines(), |ch| {
        Ok(match ch {
            '.' => false,
            '#' => true,
            _ => bail!("Wrong char: {}", ch),
        })
    })?;
    ensure!(grid.iter().map(Vec::len).all_equal(), "Not rectangular");
    let (mut nrows, mut ncols) = (grid.len(), grid[0].len());
    enlarge(&mut grid, &mut nrows, &mut ncols, [true; 4]);
    for (round, dirs) in DIRS.into_iter().cycle().enumerate() {
        let mut counts = vec![vec![0_u8; ncols]; nrows];
        for (r, row) in grid.iter().enumerate() {
            for (c, has_elf) in row.iter().enumerate() {
                if !*has_elf {
                    continue;
                }
                if all_empty(
                    &grid,
                    &[
                        (r - 1, c - 1),
                        (r - 1, c),
                        (r - 1, c + 1),
                        (r, c - 1),
                        (r, c + 1),
                        (r + 1, c - 1),
                        (r + 1, c),
                        (r + 1, c + 1),
                    ],
                ) {
                    continue; // no elf around
                }
                if let Some(d) = dirs
                    .into_iter()
                    .find(|d| all_empty(&grid, &d.adjacent_positions(r, c)))
                {
                    let (r, c) = d.get_position(r, c);
                    counts[r][c] |= d.opposite().bitmask();
                }
            }
        }
        let mut nb_moves = 0_usize;
        let [mut r0, mut r1, mut c0, mut c1] = [false; 4];
        for (r, row) in counts.into_iter().enumerate() {
            for (c, bitmask) in row.into_iter().enumerate() {
                if let Some(d) = Direction::from_bitmask(bitmask) {
                    grid[r][c] = true;
                    r0 |= r == 0;
                    r1 |= r == nrows - 1;
                    c0 |= c == 0;
                    c1 |= c == ncols - 1;
                    nb_moves += 1;
                    let (r, c) = d.get_position(r, c);
                    grid[r][c] = false;
                }
            }
        }
        return Ok(match part {
            Part1 if round == 9 => {
                display_grid(&grid);
                let pos = grid
                    .into_iter()
                    .enumerate()
                    .flat_map(|(r, row)| {
                        row.into_iter()
                            .enumerate()
                            .filter_map(move |(c, b)| b.then_some((r, c)))
                    })
                    .collect_vec();
                let (min_r, max_r) = pos
                    .iter()
                    .map(|rc| rc.0)
                    .minmax()
                    .into_option()
                    .context("no elf")?;
                let (min_c, max_c) = pos
                    .iter()
                    .map(|rc| rc.1)
                    .minmax()
                    .into_option()
                    .context("no elf")?;
                (max_c - min_c + 1) * (max_r - min_r + 1) - pos.len()
            }
            Part2 if nb_moves == 0 => {
                display_grid(&grid);
                round + 1
            }
            _ => {
                enlarge(&mut grid, &mut nrows, &mut ncols, [r0, r1, c0, c1]);
                continue;
            }
        });
    }
    unreachable!("Endless loop");
}

pub const INPUTS: [&str; 2] = [
    "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_23() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 110);
    assert_eq!(solver(Part1, INPUTS[1])?, 4068);
    assert_eq!(solver(Part2, INPUTS[0])?, 20);
    assert_eq!(solver(Part2, INPUTS[1])?, 968);
    Ok(())
}
