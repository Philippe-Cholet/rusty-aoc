use itertools::{iproduct, Itertools};

use common::{prelude::*, Ok};
use utils::OkIterator;

type Loc = (usize, usize);
const START: Loc = (500, 0);

/// Regolith Reservoir
pub fn solver(part: Part, input: &str) -> Result<String> {
    let rock_lines: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    let (a, b) = s.split_once(',').context("No comma delimiter")?;
                    Ok((a.parse()?, b.parse()?))
                })
                .collect()
        })
        .ok_collect()?;
    let (&x_min, &x_max) = rock_lines
        .iter()
        .flatten()
        .map(|(x, _y)| x)
        .minmax()
        .into_option()
        .context("No point ?!")?;
    let &y_max = rock_lines
        .iter()
        .flatten()
        .map(|(_x, y)| y)
        .max()
        .context("No point ?!")?;
    let rocks = rock_lines.into_iter().flat_map(|line| {
        line.into_iter()
            .tuple_windows()
            .flat_map(|((x0, y0), (x1, y1))| {
                iproduct!(x0.min(x1)..=x0.max(x1), y0.min(y1)..=y0.max(y1))
            })
    });
    let floor = y_max + 2;
    // let mut occupied = std::collections::HashSet::new();
    let mut occupied = grid::Grid::new(START.0, floor);
    occupied.extend(rocks);
    let mut sand_counter = 0;
    Ok('pouring: loop {
        let (mut x, mut y) = START;
        loop {
            let pos = [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                .into_iter()
                .find(|pos| !(pos.1 == floor || occupied.contains(pos)));
            // It either came to rest or not.
            match pos {
                None => {
                    if part.two() && (x, y) == START {
                        break 'pouring sand_counter + 1;
                    }
                    break;
                }
                Some(p) => {
                    (x, y) = p;
                    if part.one() && !(x_min <= x && x <= x_max && y <= y_max) {
                        break 'pouring sand_counter;
                    }
                }
            }
        }
        ensure!(occupied.insert((x, y)), "Sand already there: {:?}", (x, y));
        sand_counter += 1;
    }
    .to_string())
}

mod grid {
    use super::Loc;

    /// Except for the initialization, the methods `contains`, `extend` and `insert`
    /// behave like `HashSet<Loc>` methods.
    ///
    /// ## Note
    /// I am not sure if it even requires more memory
    /// but as it does not relies on hashes, it is faster!
    ///
    /// The total time of testing my inputs is divided by 13.
    pub struct Grid {
        x_min: usize,
        grid: Vec<Vec<bool>>,
    }

    impl Grid {
        pub fn new(x0: usize, y_max: usize) -> Self {
            Self {
                x_min: x0 - y_max,
                grid: vec![vec![false; y_max]; 2 * y_max + 1],
            }
        }

        pub fn contains(&self, (x, y): &Loc) -> bool {
            self.grid[*x - self.x_min][*y]
        }

        pub fn extend<I>(&mut self, rocks: I)
        where
            I: IntoIterator<Item = Loc>,
        {
            for (x, y) in rocks {
                self.grid[x - self.x_min][y] = true;
            }
        }

        pub fn insert(&mut self, (mut x, y): Loc) -> bool {
            x -= self.x_min;
            let is_new = !self.grid[x][y];
            self.grid[x][y] = true;
            is_new
        }
    }
}

pub const INPUTS: [&str; 2] = [
    "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_14() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "24");
    assert_eq!(solver(Part1, INPUTS[1])?, "843");
    assert_eq!(solver(Part2, INPUTS[0])?, "93");
    assert_eq!(solver(Part2, INPUTS[1])?, "27625");
    Ok(())
}
