use common::prelude::*;
use crate::utils::parse_to_grid;

#[derive(Debug, Clone, Copy)]
enum Object {
    SplitterEW,
    SplitterNS,
    MirrorNwSe,
    MirrorSwNe,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

// Merely a shortcut.
type Pt = (usize, usize);
// Each cell as four direction energy flags (u8), and sometimes an object.
type Grid = Vec<Vec<(u8, Option<Object>)>>;

/// The Floor Will Be Lava
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut grid = parse_to_grid(input.lines(), |ch| match ch {
        '\\' => Ok((0u8, Some(Object::MirrorNwSe))),
        '/' => Ok((0, Some(Object::MirrorSwNe))),
        '|' => Ok((0, Some(Object::SplitterNS))),
        '-' => Ok((0, Some(Object::SplitterEW))),
        '.' => Ok((0, None)),
        _ => bail!("Wrong char: {}", ch),
    })?;
    // TODO: ensure the grid is rectangular.
    Ok(match part {
        Part1 => count_energized_from(&mut grid, (0, 0), Direction::E),
        Part2 => {
            let (nrows, ncols) = (grid.len(), grid[0].len());
            ensure!(nrows > 0 && ncols > 0);
            let starts = itertools::chain!(
                (0..nrows).map(|r| ((r, 0), Direction::E)),
                (0..nrows).map(|r| ((r, ncols - 1), Direction::W)),
                (0..ncols).map(|c| ((0, c), Direction::S)),
                (0..ncols).map(|c| ((nrows - 1, c), Direction::N)),
            );
            #[allow(clippy::expect_used)]
            starts
                .map(|(loc, dir)| count_energized_from(&mut grid, loc, dir))
                .max()
                .expect("Empty grid")
        }
    })
}

// The grid is only temporarily changed, it is unchanged once the job here is done.
fn count_energized_from(grid: &mut Grid, loc: Pt, dir: Direction) -> usize {
    let shape = (grid.len(), grid[0].len());
    let mut stack = vec![(loc, dir)];
    while let Some((loc, dir)) = stack.pop() {
        let (flags, ref obj) = &mut grid[loc.0][loc.1];
        if *flags & dir.as_flag() != 0 {
            continue; // Been there with the same direction already!
        }
        *flags |= dir.as_flag(); // Energized from the current direction.
        match obj {
            None => stack.extend(dir.next_loc(loc, shape).map(|rc| (rc, dir))),
            Some(obj) => stack.extend(obj.next_loc(loc, dir, shape).into_iter().flatten()),
        }
    }
    // Count energized cells AND clear flags.
    grid.iter_mut().flatten().fold(0, |count, (flags, _)| {
        if *flags == 0 {
            count
        } else {
            *flags = 0;
            count + 1
        }
    })
}

impl Direction {
    const fn as_flag(self) -> u8 {
        match self {
            Self::N => 1 << 0,
            Self::S => 1 << 1,
            Self::E => 1 << 2,
            Self::W => 1 << 3,
        }
    }

    fn next_loc(self, (r, c): Pt, shape: Pt) -> Option<Pt> {
        Some(match self {
            Self::N => (r.checked_sub(1)?, c),
            Self::S => ((r + 1 < shape.0).then_some(r + 1)?, c),
            Self::E => (r, (c + 1 < shape.1).then_some(c + 1)?),
            Self::W => (r, c.checked_sub(1)?),
        })
    }
}

impl Object {
    fn next_loc(self, loc: Pt, dir: Direction, shape: Pt) -> [Option<(Pt, Direction)>; 2] {
        match (self, dir) {
            // Mirrors
            (Self::MirrorNwSe, Direction::W) | (Self::MirrorSwNe, Direction::E) => {
                [Some(Direction::N), None]
            }
            (Self::MirrorNwSe, Direction::E) | (Self::MirrorSwNe, Direction::W) => {
                [Some(Direction::S), None]
            }
            (Self::MirrorNwSe, Direction::N) | (Self::MirrorSwNe, Direction::S) => {
                [Some(Direction::W), None]
            }
            (Self::MirrorNwSe, Direction::S) | (Self::MirrorSwNe, Direction::N) => {
                [Some(Direction::E), None]
            }
            // Splitters
            (Self::SplitterEW, Direction::E | Direction::W)
            | (Self::SplitterNS, Direction::N | Direction::S) => [Some(dir), None],
            (Self::SplitterEW, Direction::N | Direction::S) => {
                [Some(Direction::E), Some(Direction::W)]
            }
            (Self::SplitterNS, Direction::E | Direction::W) => {
                [Some(Direction::N), Some(Direction::S)]
            }
        }
        .map(|d| d.and_then(|dir| dir.next_loc(loc, shape).map(|rc| (rc, dir))))
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid) {
    for row in grid {
        for (flags, obj) in row {
            match obj {
                Some(Object::SplitterEW) => print!("-"),
                Some(Object::SplitterNS) => print!("|"),
                Some(Object::MirrorNwSe) => print!("\\"),
                Some(Object::MirrorSwNe) => print!("/"),
                None => {
                    if *flags == 0 {
                        print!(".");
                    } else {
                        // A 4-bits flag is representable by 1 hex char.
                        print!("{flags:X}");
                    }
                }
            }
        }
        println!();
    }
}

pub const INPUTS: [&str; 2] = [
    r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
",
    include_input!(23 16),
];

#[test]
fn solver_23_16() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 46);
    assert_eq!(solver(Part1, INPUTS[1])?, 8112);
    assert_eq!(solver(Part2, INPUTS[0])?, 51);
    assert_eq!(solver(Part2, INPUTS[1])?, 8314);
    Ok(())
}
