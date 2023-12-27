use common::prelude::*;
use crate::utils::OkIterator;

use Direction::{East, North, South, West};
use Turn::{Back, Left, Right};

/// Rain Risk
pub fn solver(part: Part, input: &str) -> Result<i32> {
    let data: Vec<Action> = input.lines().map(str::parse).ok_collect()?;
    let instrument = match part {
        Part1 => Instrument::Directional(East),
        Part2 => {
            let mut pt = Instrument::Waypoint(Loc::default());
            pt.moving(East, 10);
            pt.moving(North, 1);
            pt
        }
    };
    let mut ship = Ship::new(instrument);
    for action in data {
        ship.navigate(action);
    }
    Ok(ship.manhattan())
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
    Back,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Direction(Direction, i32),
    Forward(i32),
    Turn(Turn),
}

#[derive(Debug, Clone, Copy, Default)]
struct Loc(i32, i32);

#[derive(Debug, Clone, Copy)]
enum Instrument {
    Directional(Direction),
    Waypoint(Loc),
}

#[derive(Debug)]
struct Ship {
    location: Loc,
    instrument: Instrument,
}

impl Loc {
    fn moving(&mut self, loc: Self, n: i32) {
        self.0 += loc.0 * n;
        self.1 += loc.1 * n;
    }
}

impl From<Direction> for Loc {
    fn from(value: Direction) -> Self {
        match value {
            North => Self(-1, 0),
            South => Self(1, 0),
            East => Self(0, 1),
            West => Self(0, -1),
        }
    }
}

impl From<Instrument> for Loc {
    fn from(value: Instrument) -> Self {
        match value {
            Instrument::Directional(dir) => dir.into(),
            Instrument::Waypoint(loc) => loc,
        }
    }
}

impl Instrument {
    fn turn(&mut self, turn: Turn) {
        match self {
            Self::Directional(dir) => {
                *dir = match (*dir, turn) {
                    (North, Back) | (West, Left) | (East, Right) => South,
                    (South, Back) | (East, Left) | (West, Right) => North,
                    (West, Back) | (South, Left) | (North, Right) => East,
                    (East, Back) | (North, Left) | (South, Right) => West,
                };
            }
            Self::Waypoint(Loc(row, col)) => {
                (*row, *col) = match turn {
                    Left => (-*col, *row),
                    Right => (*col, -*row),
                    Back => (-*row, -*col),
                };
            }
        }
    }

    fn moving(&mut self, direction: Direction, n: i32) -> bool {
        if let Self::Waypoint(loc) = self {
            loc.moving(direction.into(), n);
            true
        } else {
            false
        }
    }
}

impl Ship {
    fn new(instrument: Instrument) -> Self {
        Self {
            location: Loc::default(),
            instrument,
        }
    }

    fn navigate(&mut self, action: Action) {
        match action {
            Action::Direction(dir, n) => {
                if !self.instrument.moving(dir, n) {
                    self.location.moving(dir.into(), n);
                }
            }
            Action::Forward(n) => self.location.moving(self.instrument.into(), n),
            Action::Turn(turn) => self.instrument.turn(turn),
        }
    }

    const fn manhattan(&self) -> i32 {
        self.location.0.abs() + self.location.1.abs()
    }
}

impl std::str::FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (action, n) = s.split_at(1);
        let n = n.parse()?;
        Ok(match action {
            "N" => Self::Direction(North, n),
            "S" => Self::Direction(South, n),
            "E" => Self::Direction(East, n),
            "W" => Self::Direction(West, n),
            "F" => Self::Forward(n),
            "L" | "R" => match s {
                "L180" | "R180" => Self::Turn(Back),
                "L90" | "R270" => Self::Turn(Left),
                "L270" | "R90" => Self::Turn(Right),
                _ => bail!("Wrong turn: {}", s),
            },
            _ => bail!("Wrong action: {}", action),
        })
    }
}

pub const INPUTS: [&str; 2] = ["F10\nN3\nF7\nR90\nF11\n", include_input!(20 12)];

#[test]
fn solver_20_12() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 25);
    assert_eq!(solver(Part1, INPUTS[1])?, 415);
    assert_eq!(solver(Part2, INPUTS[0])?, 286);
    assert_eq!(solver(Part2, INPUTS[1])?, 29401);
    Ok(())
}
