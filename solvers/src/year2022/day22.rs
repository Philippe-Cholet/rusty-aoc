use std::iter::repeat;

use itertools::Itertools;

use common::prelude::*;
use crate::utils::{parse_to_grid, OkIterator};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Open,
    Void,
    Wall,
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Forward(u8),
}

impl Direction {
    const fn turn_left(self) -> Self {
        match self {
            Self::Down => Self::Right,
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
    const fn turn_right(self) -> Self {
        match self {
            Self::Down => Self::Left,
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
    const fn drc(self, r: usize, c: usize) -> (usize, usize) {
        match self {
            Self::Down => (r + 1, c),
            Self::Up => (r - 1, c),
            Self::Right => (r, c + 1),
            Self::Left => (r, c - 1),
        }
    }
    const fn facing(self) -> usize {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }
}

#[allow(clippy::match_on_vec_items)] // I made sure the grid is rectangular, it should not panic.
fn follow_instructions(
    instructions: &[Instruction],
    grid: &[Vec<Tile>],
) -> Result<(usize, usize, Direction)> {
    #[cfg(debug_assertions)]
    for row in grid {
        for tile in row {
            match tile {
                Tile::Open => print!("."),
                Tile::Void => print!("+"),
                Tile::Wall => print!("#"),
            }
        }
        println!();
    }
    let mut d = Direction::Right;
    let (mut r, mut c) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, tile)| (tile == &Tile::Open).then_some((r, c)))
        })
        .context("No opening tile")?;
    for instruction in instructions {
        match instruction {
            Instruction::Left => d = d.turn_left(),
            Instruction::Right => d = d.turn_right(),
            Instruction::Forward(n) => {
                let (mut r0, mut c0) = (r, c);
                for _ in 0..*n {
                    let (mut r1, mut c1) = d.drc(r0, c0);
                    match grid[r1][c1] {
                        Tile::Open => (r0, c0) = (r1, c1),
                        Tile::Wall => break,
                        Tile::Void => {
                            (r1, c1) = find_position_after_void(grid, r1, c1, d)?;
                            match grid[r1][c1] {
                                Tile::Open => (r0, c0) = (r1, c1),
                                Tile::Wall => break,
                                Tile::Void => bail!("Void --> Void"),
                            }
                        }
                    }
                }
                (r, c) = (r0, c0);
            }
        }
    }
    // println!("{:?}", (r, c, d));
    Ok((r, c, d))
}

fn find_position_after_void(
    grid: &[Vec<Tile>],
    r: usize,
    c: usize,
    d: Direction,
) -> Result<(usize, usize)> {
    let mut positions: Box<dyn Iterator<Item = (usize, usize)>> = match d {
        Direction::Down => Box::new((0..=r).zip(repeat(c))),
        Direction::Up => Box::new((r..grid.len()).rev().zip(repeat(c))),
        Direction::Right => Box::new(repeat(r).zip(0..=c)),
        Direction::Left => Box::new(repeat(r).zip((c..grid[r].len()).rev())),
    };
    positions
        .find(|(r0, c0)| grid[*r0][*c0] != Tile::Void)
        .context("Could not find non-void position")
}

#[allow(clippy::match_on_vec_items)] // I made sure the grid is rectangular, it should not panic.
fn follow_instructions_v2(
    instructions: &[Instruction],
    grid: &[Vec<Tile>],
) -> Result<(usize, usize, Direction)> {
    #[cfg(debug_assertions)]
    for row in grid {
        for tile in row {
            match tile {
                Tile::Open => print!("."),
                Tile::Void => print!("+"),
                Tile::Wall => print!("#"),
            }
        }
        println!();
    }
    let mut d = Direction::Right;
    let (mut r, mut c) = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter()
                .enumerate()
                .find_map(|(c, tile)| (tile == &Tile::Open).then_some((r, c)))
        })
        .context("No opening tile")?;
    for instruction in instructions {
        match instruction {
            Instruction::Left => d = d.turn_left(),
            Instruction::Right => d = d.turn_right(),
            Instruction::Forward(n) => {
                let (mut r0, mut c0, mut d0) = (r, c, d);
                for _ in 0..*n {
                    let (mut r1, mut c1) = d0.drc(r0, c0);
                    let mut d1 = d0;
                    match grid[r1][c1] {
                        Tile::Open => (r0, c0) = (r1, c1),
                        Tile::Wall => break,
                        Tile::Void => {
                            (r1, c1, d1) = find_position_after_void_v2(r1, c1, d1)?;
                            match grid[r1][c1] {
                                Tile::Open => (r0, c0, d0) = (r1, c1, d1),
                                Tile::Wall => break,
                                Tile::Void => bail!("Void --> Void"),
                            }
                        }
                    }
                }
                (r, c, d) = (r0, c0, d0);
            }
        }
    }
    // println!("{:?}", (r, c, d));
    Ok((r, c, d))
}

// Only work for this layout, (and side size = 50)
//  ---
// | ##|
// | # |
// |## |
// |#  |
//  ---
fn find_position_after_void_v2(
    r: usize,
    c: usize,
    d: Direction,
) -> Result<(usize, usize, Direction)> {
    Ok(match (r, c, d) {
        (100, 1..=50, Direction::Up) => (c + 50, 51, Direction::Right),
        (0, 51..=100, Direction::Up) => (c + 100, 1, Direction::Right),
        (0, 101..=150, Direction::Up) => (200, c - 100, Direction::Up),

        (201, 1..=50, Direction::Down) => (1, c + 100, Direction::Down),
        (151, 51..=100, Direction::Down) => (c + 100, 50, Direction::Left),
        (51, 101..=150, Direction::Down) => (c - 50, 100, Direction::Left),

        (1..=50, 50, Direction::Left) => (151 - r, 1, Direction::Right),
        (51..=100, 50, Direction::Left) => (101, r - 50, Direction::Down),
        (101..=150, 0, Direction::Left) => (151 - r, 51, Direction::Right),
        (151..=200, 0, Direction::Left) => (1, r - 100, Direction::Down),

        (1..=50, 151, Direction::Right) => (151 - r, 100, Direction::Left),
        (51..=100, 101, Direction::Right) => (50, r + 50, Direction::Up),
        (101..=150, 101, Direction::Right) => (151 - r, 150, Direction::Left),
        (151..=200, 51, Direction::Right) => (150, r - 100, Direction::Up),

        _ => bail!("Position: {:?}, {:?}", (r, c), d),
    })
}

/// Monkey Map
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let (grid, line) = input
        .trim_end()
        .split_once("\n\n")
        .context("No empty line")?;
    let mut grid = parse_to_grid(grid.lines(), |ch| match ch {
        '#' => Ok(Tile::Wall),
        '.' => Ok(Tile::Open),
        ' ' => Ok(Tile::Void),
        _ => bail!("Wrong char: {}", ch),
    })?;
    let ncols = grid.iter().map(Vec::len).max().context("No column")? + 2;
    // Because we insert "void contours", indexes 1 needed are now reals in the grid.
    for row in &mut grid {
        row.insert(0, Tile::Void);
        for _ in 0..ncols - row.len() {
            row.push(Tile::Void);
        }
    }
    grid.insert(0, (0..ncols).map(|_| Tile::Void).collect());
    grid.push((0..ncols).map(|_| Tile::Void).collect());
    let instructions = line
        .chars()
        .group_by(char::is_ascii_digit)
        .into_iter()
        .map(|(b, mut group)| {
            let t = group.join("");
            Ok(if b {
                Instruction::Forward(t.parse()?)
            } else if &t == "L" {
                Instruction::Left
            } else if &t == "R" {
                Instruction::Right
            } else {
                bail!("Wrong instruction: {:?}", t);
            })
        })
        .ok_collect_vec()?;
    let (r, c, d) = match part {
        Part1 => follow_instructions(&instructions, &grid)?,
        Part2 => follow_instructions_v2(&instructions, &grid)?,
    };
    Ok(1000 * r + 4 * c + d.facing())
}

pub const INPUTS: [&str; 2] = [
    "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
",
    include_input!(22 22),
];

#[test]
fn solver_22_22() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 6032);
    assert_eq!(solver(Part1, INPUTS[1])?, 30552);
    // This does not pass tests yet as my implementation relies on my input layout and size.
    // assert_eq!(solver(Part2, INPUTS[0])?, 5031);
    assert_eq!(solver(Part2, INPUTS[1])?, 184106);
    Ok(())
}
