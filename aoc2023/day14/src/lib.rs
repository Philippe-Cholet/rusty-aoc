use itertools::Itertools;

use common::prelude::*;
use utils::parse_to_grid;

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    CubeShapeRock,
    RoundedRock,
}

struct Grid {
    data: Vec<Vec<Cell>>,
    nrows: usize,
    ncols: usize,
}

/// Parabolic Reflector Dish
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut grid: Grid = input.parse()?;
    match part {
        Part1 => grid.roll_north(),
        Part2 => {
            const NB_CYCLES: u32 = 1_000_000_000;
            let mut cache = HashMap::with_capacity(150);
            for step in 0..NB_CYCLES {
                grid.roll_cycle();
                if let Some(prev_step) = cache.insert(grid.id(), step) {
                    let remaining = NB_CYCLES - 1 - step;
                    let period = step - prev_step;
                    for _ in 0..remaining % period {
                        grid.roll_cycle();
                    }
                    break;
                }
            }
        }
    }
    Ok(grid.total_load())
}

impl Cell {
    #[inline]
    const fn is_round(self) -> bool {
        matches!(self, Self::RoundedRock)
    }

    #[inline]
    const fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl Grid {
    fn total_load(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|cell| cell.is_round()).count())
            .rev()
            .enumerate()
            .map(|(idx, count)| (idx + 1) * count)
            .sum()
    }

    /// Encode the locations of the rounded rocks, assuming grid is 100x100 max.
    ///
    /// NOTE: `64 * 157 > 100 * 100`
    fn id(&self) -> [u64; 157] {
        let mut bits = [0; 157];
        for (idx, obj) in self.data.iter().flatten().enumerate() {
            if obj.is_round() {
                bits[idx / 64] |= 1 << (idx % 64);
            }
        }
        bits
    }

    /// Roll one cycle: North West South East.
    #[inline]
    fn roll_cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    #[inline]
    fn roll_helper<C, R>(&mut self, coords: C, ray: impl Fn((usize, usize)) -> R)
    where
        C: Iterator<Item = (usize, usize)>,
        R: Iterator<Item = (usize, usize)>,
    {
        for (r, c) in coords {
            if self.data[r][c].is_round() {
                let empties = ray((r, c)).take_while(|&(r0, c0)| self.data[r0][c0].is_empty());
                if let Some((r1, c1)) = empties.last() {
                    self.data[r][c] = Cell::Empty;
                    self.data[r1][c1] = Cell::RoundedRock;
                }
            }
        }
    }

    fn roll_north(&mut self) {
        self.roll_helper(
            (1..self.nrows).cartesian_product(0..self.ncols),
            |(r, c)| (0..r).rev().map(move |i| (i, c)),
        );
    }

    fn roll_south(&mut self) {
        let nrows = self.nrows;
        self.roll_helper(
            (0..nrows - 1).rev().cartesian_product(0..self.ncols),
            |(r, c)| (r + 1..nrows).map(move |i| (i, c)),
        );
    }

    fn roll_west(&mut self) {
        self.roll_helper(
            (1..self.ncols)
                .cartesian_product(0..self.nrows)
                .map(|(c, r)| (r, c)),
            |(r, c)| (0..c).rev().map(move |i| (r, i)),
        );
    }

    fn roll_east(&mut self) {
        let ncols = self.ncols;
        self.roll_helper(
            (0..ncols - 1)
                .rev()
                .cartesian_product(0..self.nrows)
                .map(|(c, r)| (r, c)),
            |(r, c)| (c + 1..ncols).map(move |i| (r, i)),
        );
    }
}

impl std::str::FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let data = parse_to_grid(s.lines(), |ch| match ch {
            '.' => Ok(Cell::Empty),
            'O' => Ok(Cell::RoundedRock),
            '#' => Ok(Cell::CubeShapeRock),
            _ => bail!("Wrong char: {}", ch),
        })?;
        let nrows = data.len();
        let ncols = data[0].len();
        // TODO: ensure the grid is rectangular.
        Ok(Self { data, nrows, ncols })
    }
}

// Useful to visually check the example grid after cycles 1 2 3.
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for obj in row {
                match obj {
                    Cell::Empty => f.write_str(".")?,
                    Cell::RoundedRock => f.write_str("O")?,
                    Cell::CubeShapeRock => f.write_str("#")?,
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

pub const INPUTS: [&str; 2] = [
    "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
",
    include_str!("input.txt"),
];

#[test]
fn solver_23_14() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 136);
    assert_eq!(solver(Part1, INPUTS[1])?, 108935);
    assert_eq!(solver(Part2, INPUTS[0])?, 64);
    assert_eq!(solver(Part2, INPUTS[1])?, 100876);
    Ok(())
}
