use itertools::iproduct;
use ndarray::{s, Array2};

use common::prelude::*;
use crate::utils::parse_to_grid;

const N: usize = 100;

/// Like a GIF For Your Yard
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let grid = parse_to_grid(input.lines(), |ch| match ch {
        '#' => Ok(true),
        '.' => Ok(false),
        _ => bail!("Expected # or . but got {:?}.", ch),
    })?;
    let mut grid = Array2::<bool>::from_shape_fn((N, N), |(r, c)| grid[r][c]);
    let turn_on_corners = |grid: &mut Array2<bool>| {
        if part.two() {
            for loc in iproduct!([0, N - 1], [0, N - 1]) {
                grid[loc] = true;
            }
        }
    };
    turn_on_corners(&mut grid);
    for _step in 0..100 {
        let mut new = Array2::<bool>::default((N, N));
        for (r, c) in iproduct!(0..N, 0..N) {
            let nb = grid
                .slice(s![
                    r.saturating_sub(1)..(r + 2).min(N),
                    c.saturating_sub(1)..(c + 2).min(N),
                ])
                .iter()
                .filter(|&&is_on| is_on)
                .count();
            // I count `grid[(r, c)]` if it's ON so `2 | 3` becomes `3 | 4`.
            new[(r, c)] = matches!((grid[(r, c)], nb), (true, 3 | 4) | (false, 3));
        }
        grid = new;
        turn_on_corners(&mut grid);
    }
    Ok(grid
        .into_raw_vec()
        .into_iter()
        .filter(|&is_on| is_on)
        .count())
}

test_solver!(include_input!(15 18) => (768, 781));
