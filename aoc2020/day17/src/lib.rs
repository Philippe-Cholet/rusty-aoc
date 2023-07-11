use itertools::iproduct;
use ndarray::prelude::*;

use common::prelude::*;
use utils::parse_to_grid;

const N: usize = 6;

fn count_actives<'a>(it: impl IntoIterator<Item = &'a bool>) -> usize {
    it.into_iter().filter(|&active| *active).count()
}

/// Conway Cubes
pub fn solver(part: Part, input: &str) -> Result<String> {
    let initial_grid = parse_to_grid(input.lines(), |ch| match ch {
        '#' => Ok(true),
        '.' => Ok(false),
        _ => bail!("Wrong char: {}", ch),
    })?;
    let (nrows, ncols) = (initial_grid.len(), initial_grid[0].len());
    let flaten_array = match part {
        Part1 => {
            let shape = (N + 1 + N, N + nrows + N, N + ncols + N);
            let mut array = Array::default(shape);
            for (r, c) in iproduct!(0..nrows, 0..ncols) {
                array[(N, N + r, N + c)] = initial_grid[r][c];
            }
            for k in 1..=N {
                let mut new_array = Array::default(shape);
                for xyz in iproduct!(N - k..=N + k, N - k..N + k + nrows, N - k..N + k + ncols) {
                    let count = count_actives(array.slice(s![
                        xyz.0.saturating_sub(1)..shape.0.min(xyz.0 + 2),
                        xyz.1.saturating_sub(1)..shape.1.min(xyz.1 + 2),
                        xyz.2.saturating_sub(1)..shape.2.min(xyz.2 + 2),
                    ]));
                    // NOTE: "3 or 4" when active instead of "2 or 3" because `xyz` is counted.
                    new_array[xyz] = matches!((array[xyz], count), (true, 3 | 4) | (false, 3));
                }
                array = new_array;
            }
            array.into_raw_vec()
        }
        Part2 => {
            let shape = (N + 1 + N, N + 1 + N, N + nrows + N, N + ncols + N);
            let mut array = Array::default(shape);
            for (r, c) in iproduct!(0..nrows, 0..ncols) {
                array[(N, N, N + r, N + c)] = initial_grid[r][c];
            }
            for k in 1..=N {
                let mut new_array = Array::default(shape);
                for xyzw in iproduct!(
                    N - k..=N + k,
                    N - k..=N + k,
                    N - k..N + k + nrows,
                    N - k..N + k + ncols
                ) {
                    let count = count_actives(array.slice(s![
                        xyzw.0.saturating_sub(1)..shape.0.min(xyzw.0 + 2),
                        xyzw.1.saturating_sub(1)..shape.1.min(xyzw.1 + 2),
                        xyzw.2.saturating_sub(1)..shape.2.min(xyzw.2 + 2),
                        xyzw.3.saturating_sub(1)..shape.3.min(xyzw.3 + 2),
                    ]));
                    // NOTE: "3 or 4" when active instead of "2 or 3" because `xyzw` is counted.
                    new_array[xyzw] = matches!((array[xyzw], count), (true, 3 | 4) | (false, 3));
                }
                array = new_array;
            }
            array.into_raw_vec()
        }
    };
    Ok(count_actives(&flaten_array).to_string())
}

pub const INPUTS: [&str; 2] = [".#.\n..#\n###\n", include_str!("input.txt")];

#[test]
fn solver_20_17() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "112");
    assert_eq!(solver(Part1, INPUTS[1])?, "315");
    assert_eq!(solver(Part2, INPUTS[0])?, "848");
    assert_eq!(solver(Part2, INPUTS[1])?, "1520");
    Ok(())
}
