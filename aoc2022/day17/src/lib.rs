use std::collections::{HashMap, HashSet};

use common::prelude::*;
use utils::OkIterator;

const CAVE_WIDTH: usize = 7;
const NB_ROCKS: usize = 5;

#[derive(Debug, Clone, Copy)]
enum Rock {
    // ####
    Minus,
    // .#.
    // ###
    // .#.
    Plus,
    // ..#
    // ..#
    // ###
    Corner,
    // #
    // #
    // #
    // #
    Tall,
    // ##
    // ##
    Square,
}

#[derive(Debug, Clone, Copy)]
enum JetDirection {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

#[derive(Debug)]
struct TetrisSolver {
    period_detector: HashMap<String, (usize, usize)>,
    jets: std::iter::Cycle<std::vec::IntoIter<JetDirection>>,
    resting_rocks: HashSet<Loc>,
}

impl Rock {
    const ALL: [Self; NB_ROCKS] = [
        Self::Minus,
        Self::Plus,
        Self::Corner,
        Self::Tall,
        Self::Square,
    ];

    const fn width(self) -> usize {
        match self {
            Self::Minus => 4,
            Self::Plus | Self::Corner => 3,
            Self::Square => 2,
            Self::Tall => 1,
        }
    }

    const fn height(self) -> usize {
        match self {
            Self::Tall => 4,
            Self::Plus | Self::Corner => 3,
            Self::Square => 2,
            Self::Minus => 1,
        }
    }

    fn locs(self, pos: Loc) -> Vec<Loc> {
        let (x, y) = (pos.0, pos.1);
        match self {
            Self::Minus => vec![Loc(x, y), Loc(x + 1, y), Loc(x + 2, y), Loc(x + 3, y)],
            Self::Plus => vec![
                Loc(x + 1, y),
                Loc(x, y + 1),
                Loc(x + 1, y + 1),
                Loc(x + 2, y + 1),
                Loc(x + 1, y + 2),
            ],
            Self::Corner => vec![
                Loc(x, y),
                Loc(x + 1, y),
                Loc(x + 2, y),
                Loc(x + 2, y + 1),
                Loc(x + 2, y + 2),
            ],
            Self::Tall => vec![Loc(x, y), Loc(x, y + 1), Loc(x, y + 2), Loc(x, y + 3)],
            Self::Square => vec![Loc(x, y), Loc(x + 1, y), Loc(x, y + 1), Loc(x + 1, y + 1)],
        }
    }
}

impl Loc {
    const fn new(height: usize) -> Self {
        Self(2, height + 3)
    }

    const fn push(self, direction: JetDirection, rock: Rock) -> Self {
        let x = match direction {
            JetDirection::Right if self.0 < CAVE_WIDTH - rock.width() => self.0 + 1,
            JetDirection::Left if self.0 > 0 => self.0 - 1,
            _ => self.0,
        };
        Self(x, self.1)
    }

    const fn height(self) -> usize {
        self.1
    }

    const fn falls(self) -> Option<Self> {
        match self.1.checked_sub(1) {
            Some(y) => Some(Self(self.0, y)),
            None => None, // Rock meets ground.
        }
    }
}

impl TetrisSolver {
    fn visualize(&self, last: Option<usize>) -> String {
        let mut text = String::new();
        if let Some(height) = self.resting_rocks.iter().map(|p| p.1).max() {
            let start = last.map_or(0, |lim| height.saturating_sub(lim));
            for y in (start..=height).rev() {
                for x in 0..CAVE_WIDTH {
                    text.push(if self.resting_rocks.contains(&Loc(x, y)) {
                        '#'
                    } else {
                        '.'
                    });
                }
                text.push('\n');
            }
        }
        text
    }

    fn feed_detector(&mut self, step: usize, height: usize) -> Option<(usize, usize)> {
        let key = self.visualize(Some(100));
        if let Some((prev_step, prev_height)) = self.period_detector.get(&key) {
            Some((step - prev_step, height - prev_height))
        } else {
            self.period_detector.insert(key, (step, height));
            None
        }
    }

    #[allow(clippy::expect_used)]
    fn throw_rock(&mut self, rock: Rock, mut loc: Loc) -> usize {
        loop {
            // NOTE: "push" checks for collisions with walls.
            let new_loc = loc.push(self.jets.next().expect("non-empty cycle"), rock);
            if !self.is_collision(rock, new_loc) {
                // No collision with other rocks.
                loc = new_loc;
            }
            let Some(new_loc) = loc.falls() else {
                break; // hit the ground
            };
            if self.is_collision(rock, new_loc) {
                break; // there is a rock below
            }
            loc = new_loc;
        }
        // Come to rest
        self.resting_rocks.extend(rock.locs(loc));
        loc.height()
    }

    fn solve(mut self, nb_steps: usize) -> usize {
        let mut height = 0;
        let mut skipped_height = 0;
        let mut step = 0;
        while step != nb_steps {
            debug_assert_eq!(step % NB_ROCKS, 0);
            if let Some((period, periodic_height)) = self.feed_detector(step / NB_ROCKS, height) {
                let steps_in_one_period = period * NB_ROCKS;
                let nb_skip_periods = (nb_steps - step) / steps_in_one_period;
                step += nb_skip_periods * steps_in_one_period;
                skipped_height += nb_skip_periods * periodic_height;
            }
            if step == nb_steps {
                break;
            }
            for rock in Rock::ALL {
                let resting_height = self.throw_rock(rock, Loc::new(height));
                height = height.max(resting_height + rock.height());
                step += 1;
                if step == nb_steps {
                    break;
                }
            }
        }
        // self.visualize(None);
        height + skipped_height
    }

    fn is_collision(&self, rock: Rock, loc: Loc) -> bool {
        rock.locs(loc)
            .into_iter()
            .any(|pt| self.resting_rocks.contains(&pt))
    }
}

impl std::str::FromStr for TetrisSolver {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let jet_patterns = s
            .trim_end()
            .chars()
            .map(|ch| match ch {
                '>' => Ok(JetDirection::Right),
                '<' => Ok(JetDirection::Left),
                _ => bail!("Wrong char: {}", ch),
            })
            .ok_collect_vec()?;
        ensure!(!jet_patterns.is_empty(), "No jet");
        Ok(Self {
            period_detector: HashMap::new(),
            jets: jet_patterns.into_iter().cycle(),
            resting_rocks: HashSet::new(),
        })
    }
}

/// Pyroclastic Flow
pub fn solver(part: Part, input: &str) -> Result<String> {
    let nb_steps = match part {
        Part1 => 2022,
        Part2 => 1_000_000_000_000,
    };
    let height = input.parse::<TetrisSolver>()?.solve(nb_steps);
    Ok(height.to_string())
}

pub const INPUTS: [&str; 2] = [
    ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n",
    include_str!("input.txt"),
];

#[test]
fn solver_22_17() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "3068");
    assert_eq!(solver(Part1, INPUTS[1])?, "3157"); // 10h14
    assert_eq!(solver(Part2, INPUTS[0])?, "1514285714288");
    assert_eq!(solver(Part2, INPUTS[1])?, "1581449275319"); // 12h35
    Ok(())
}
