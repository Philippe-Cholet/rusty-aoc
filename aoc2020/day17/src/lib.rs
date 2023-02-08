use itertools::iproduct;

use common::{bail, Part, Part1, Part2, Result};
use utils::parse_to_grid;

const N: usize = 6;

/// Conway Cubes
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut initial_grid = parse_to_grid(input.lines(), |ch| match ch {
        '#' => Ok(true),
        '.' => Ok(false),
        _ => bail!("Wrong char: {}", ch),
    })?;
    let (nrows, ncols) = (initial_grid.len(), initial_grid[0].len());
    for row in &mut initial_grid {
        for _ in 0..N {
            row.insert(0, false);
            row.push(false);
        }
    }
    for _ in 0..N {
        initial_grid.insert(0, vec![false; N + ncols + N]);
        initial_grid.push(vec![false; N + ncols + N]);
    }
    Ok(match part {
        Part1 => {
            let shape = (N + 1 + N, N + nrows + N, N + ncols + N);
            let mut grid = vec![vec![vec![false; shape.2]; shape.1]; shape.0];
            grid[N] = initial_grid;
            for k in 1..=N {
                let mut new_grid = grid.clone();
                for (x, y, z) in
                    iproduct!(N - k..=N + k, N - k..N + k + nrows, N - k..N + k + ncols)
                {
                    let count = iproduct!(-1..=1, -1..=1, -1..=1)
                        .filter(|xyz| xyz != &(0, 0, 0))
                        .filter_map(|(dx, dy, dz)| {
                            let x0 = x.checked_add_signed(dx)?;
                            let y0 = y.checked_add_signed(dy)?;
                            let z0 = z.checked_add_signed(dz)?;
                            (x0 < shape.0 && y0 < shape.1 && z0 < shape.2).then_some((x0, y0, z0))
                        })
                        .filter(|&(x, y, z)| grid[x][y][z])
                        .take(4)
                        .count();
                    new_grid[x][y][z] =
                        matches!((grid[x][y][z], count), (true, 2 | 3) | (false, 3));
                }
                grid = new_grid;
            }
            grid.into_iter()
                .flatten()
                .flatten()
                .filter(|&active| active)
                .count()
        }
        Part2 => {
            let shape = (N + 1 + N, N + 1 + N, N + nrows + N, N + ncols + N);
            let mut grid = vec![vec![vec![vec![false; shape.3]; shape.2]; shape.1]; shape.0];
            grid[N][N] = initial_grid;
            for k in 1..=N {
                let mut new_grid = grid.clone();
                for (x, y, z, w) in iproduct!(
                    N - k..=N + k,
                    N - k..=N + k,
                    N - k..N + k + nrows,
                    N - k..N + k + ncols
                ) {
                    let count = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
                        .filter(|xyzt| xyzt != &(0, 0, 0, 0))
                        .filter_map(|(dx, dy, dz, dw)| {
                            let x0 = x.checked_add_signed(dx)?;
                            let y0 = y.checked_add_signed(dy)?;
                            let z0 = z.checked_add_signed(dz)?;
                            let w0 = w.checked_add_signed(dw)?;
                            (x0 < shape.0 && y0 < shape.1 && z0 < shape.2 && w0 < shape.3)
                                .then_some((x0, y0, z0, w0))
                        })
                        .filter(|&(x, y, z, w)| grid[x][y][z][w])
                        .take(4)
                        .count();
                    new_grid[x][y][z][w] =
                        matches!((grid[x][y][z][w], count), (true, 2 | 3) | (false, 3));
                }
                grid = new_grid;
            }
            grid.into_iter()
                .flatten()
                .flatten()
                .flatten()
                .filter(|&active| active)
                .count()
        }
    }
    .to_string())
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
