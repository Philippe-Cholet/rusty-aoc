use anyhow::Result;

pub fn parse<'a, It, T, F>(s: It, mut parser: F) -> Result<Vec<Vec<T>>>
where
    It: IntoIterator<Item = &'a str>,
    F: FnMut(char) -> Result<T>,
{
    s.into_iter()
        .map(|line| line.chars().map(&mut parser).collect())
        .collect()
}

pub fn parse_with_loc<'a, It, T, F>(s: It, mut parser: F) -> Result<Vec<Vec<T>>>
where
    It: IntoIterator<Item = &'a str>,
    F: FnMut((usize, usize), char) -> Result<T>,
{
    s.into_iter()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| parser((r, c), ch))
                .collect()
        })
        .collect()
}

/// Neighbors of `loc` in a `nrows` x `ncols` grid.
#[must_use]
pub fn neighbors(
    loc: (usize, usize),
    nrows: usize,
    ncols: usize,
    diagonally: bool,
) -> Vec<(usize, usize)> {
    let (r, c) = loc;
    debug_assert!(r < nrows && c < ncols); // r and c are positive since they are unsigned.
    let mut res = vec![];
    // Quite verbose but adding a signed integer (-1, 0, 1) to a `usize`
    // by casting into different integers types did not feel great.
    let (r0, r1) = (r != 0, r + 1 != nrows);
    let (c0, c1) = (c != 0, c + 1 != ncols);
    // SE
    if diagonally && r1 && c1 {
        res.push((r + 1, c + 1));
    }
    // S
    if r1 {
        res.push((r + 1, c));
    }
    // E
    if c1 {
        res.push((r, c + 1));
    }
    // NE
    if diagonally && r0 && c1 {
        res.push((r - 1, c + 1));
    }
    // SW
    if diagonally && r1 && c0 {
        res.push((r + 1, c - 1));
    }
    // W
    if c0 {
        res.push((r, c - 1));
    }
    // N
    if r0 {
        res.push((r - 1, c));
    }
    // NW
    if diagonally && r0 && c0 {
        res.push((r - 1, c - 1));
    }
    res
}
