use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use common::{format_err, Result};

type Parser<S, T> = fn(S) -> Result<T>;

/// Map `Iterator<Item=&str>` and try to collect results to clear types.
pub trait FromIterStr<'a>: Sized + Iterator<Item = &'a str> {
    fn parse_to_vec<T>(self, parser: Parser<&'a str, T>) -> Result<Vec<T>> {
        self.map(parser).collect()
    }

    fn parse_to_hmap<K, V>(self, parser: Parser<&'a str, (K, V)>) -> Result<HashMap<K, V>>
    where
        K: Eq + Hash,
    {
        self.map(parser).collect()
    }

    fn parse_to_hset<T>(self, parser: Parser<&'a str, T>) -> Result<HashSet<T>>
    where
        T: Eq + Hash,
    {
        self.map(parser).collect()
    }

    fn parse_to_grid<T>(self, parser: Parser<char, T>) -> Result<Vec<Vec<T>>> {
        self.map(|line| line.chars().map(parser).collect())
            .collect()
    }

    /*
    // No use for it yet! So I prefer to not include it and change it if needed.
    fn parse_to_string
    fn parse_to_btmap<K, V> // std::collections::BTreeMap
    fn parse_to_btset<T>    // std::collections::BTreeSet

    fn to_loc_grid<T>(self, parser: Parser<((usize, usize), char), T>) -> Result<Vec<Vec<T>>> {
        self.enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| parser(((r, c), ch)))
                    .collect()
            })
            .collect()
    }
    */

    // Sometimes `str::parse` would do the job directly.
    // However the error type would not necessarily be anyhow-compatible
    // so to make it work we can not propagate any useful message.

    fn parse_str_to_vec<T>(self) -> Result<Vec<T>>
    where
        T: FromStr,
    {
        self.map(str::parse)
            .collect::<Result<_, _>>()
            .map_err(|_| format_err!("Failed to parse from str"))
    }

    /*
    fn parse_str_to_string
    fn parse_str_to_hmap
    fn parse_str_to_hset
    fn parse_str_to_grid
    fn parse_str_to_btmap
    fn parse_str_to_btset
    */
}

impl<'a, T> FromIterStr<'a> for T where T: Sized + Iterator<Item = &'a str> {}

/// Neighbors of `(r, c)` in a `nrows` x `ncols` grid.
#[must_use]
pub fn neighbors(
    r: usize,
    c: usize,
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
