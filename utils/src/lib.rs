use common::{Context, Error, Result};

mod ok_iterator;
pub use ok_iterator::OkIterator;

pub fn parse_to_grid<'a, It, T, F>(s: It, mut parser: F) -> Result<Vec<Vec<T>>>
where
    It: IntoIterator<Item = &'a str>,
    F: FnMut(char) -> Result<T>,
{
    s.into_iter()
        .map(|line| line.chars().map(&mut parser).collect())
        .collect()
}

pub fn parse_to_grid_with_loc<'a, It, T, F>(s: It, mut parser: F) -> Result<Vec<Vec<T>>>
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

/// Neighbors of `(r, c)` in a `nrows` x `ncols` grid.
#[must_use]
pub fn neighbors(
    (r, c): (usize, usize),
    nrows: usize,
    ncols: usize,
    diagonally: bool,
) -> Vec<(usize, usize)> {
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

/// Arbitrary item that can be compared by its heuristic.
///
/// Meant to be used in a priority queue, where only the heuristic matter.
#[derive(Debug)]
pub struct HeuristicItem<H: Ord, T> {
    pub heuristic: H,
    pub item: T,
}

impl<H: Ord, T> PartialEq for HeuristicItem<H, T> {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic == other.heuristic
    }
}

impl<H: Ord, T> Eq for HeuristicItem<H, T> {}

impl<H: Ord, T> PartialOrd for HeuristicItem<H, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<H: Ord, T> Ord for HeuristicItem<H, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic)
    }
}

/// Convert a char representing a decimal digit to any type that can come from `u32`.
pub fn char10<T>(ch: char) -> Result<T>
where
    T: TryFrom<u32>,
    T::Error: Into<Error>,
{
    let dec = ch.to_digit(10).context("Not decimal")?;
    T::try_from(dec).map_err(Into::into)
}

/// Convert a char representing a hexadecimal digit to any type that can come from `u32`.
pub fn char16<T>(ch: char) -> Result<T>
where
    T: TryFrom<u32>,
    T::Error: Into<Error>,
{
    let hex = ch.to_digit(16).context("Not hexadecimal")?;
    T::try_from(hex).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char10() -> Result<()> {
        for (dec, ch) in "0123456789".chars().enumerate() {
            assert_eq!(char10::<u8>(ch)?, dec as u8);
            assert_eq!(char10::<i8>(ch)?, dec as i8);
            assert_eq!(char10::<u16>(ch)?, dec as u16);
            assert_eq!(char10::<i16>(ch)?, dec as i16);
            assert_eq!(char10::<i32>(ch)?, dec as i32);
            assert_eq!(char10::<u32>(ch)?, dec as u32);
            assert_eq!(char10::<i64>(ch)?, dec as i64);
            assert_eq!(char10::<u64>(ch)?, dec as u64);
            assert_eq!(char10::<i128>(ch)?, dec as i128);
            assert_eq!(char10::<u128>(ch)?, dec as u128);
            assert_eq!(char10::<usize>(ch)?, dec as usize);
            assert_eq!(char10::<isize>(ch)?, dec as isize);
            // As long as it can infer the type.
            let _x: Vec<i32> = vec![char10(ch)?];
            let _x: Vec<u32> = vec![char10(ch)?];
            let _x: Vec<usize> = vec![char10(ch)?];
            let _x: Vec<u8> = vec![char10(ch)?];
            let _x: Vec<i8> = vec![char10(ch)?];
        }
        let input = "123\n456\n789\n";
        let _grid = parse_to_grid(input.lines(), char10::<i8>)?;
        let grid = parse_to_grid(input.lines(), char10)?;
        let _n: u32 = grid[0][0];
        Ok(())
    }

    #[test]
    fn test_char16() -> Result<()> {
        for (hex, ch) in "0123456789abcdef".chars().enumerate() {
            assert_eq!(char16::<u8>(ch)?, hex as u8);
            assert_eq!(char16::<i8>(ch)?, hex as i8);
            assert_eq!(char16::<u16>(ch)?, hex as u16);
            assert_eq!(char16::<i16>(ch)?, hex as i16);
            assert_eq!(char16::<i32>(ch)?, hex as i32);
            assert_eq!(char16::<u32>(ch)?, hex as u32);
            assert_eq!(char16::<i64>(ch)?, hex as i64);
            assert_eq!(char16::<u64>(ch)?, hex as u64);
            assert_eq!(char16::<i128>(ch)?, hex as i128);
            assert_eq!(char16::<u128>(ch)?, hex as u128);
            assert_eq!(char16::<usize>(ch)?, hex as usize);
            assert_eq!(char16::<isize>(ch)?, hex as isize);
            // As long as it can infer the type.
            let _x: Vec<i32> = vec![char16(ch)?];
            let _x: Vec<u32> = vec![char16(ch)?];
            let _x: Vec<usize> = vec![char16(ch)?];
            let _x: Vec<u8> = vec![char16(ch)?];
            let _x: Vec<i8> = vec![char16(ch)?];
        }
        let input = "0123\n4567\n89ab\ncdef\n";
        let _grid = parse_to_grid(input.lines(), char16::<i8>)?;
        let grid = parse_to_grid(input.lines(), char16)?;
        let _n: u32 = grid[0][0];
        Ok(())
    }
}
