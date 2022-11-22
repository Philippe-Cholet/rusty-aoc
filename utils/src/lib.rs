use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use common::Result;

#[derive(Debug)]
pub struct InputParser<'a>(pub &'a str);

impl<'a> InputParser<'a> {
    pub fn lines<T>(&self, parser: fn(&'a str) -> Result<T>) -> Result<Vec<T>> {
        self.0.lines().map(parser).collect()
    }

    pub fn lines_hmap<K, V>(&self, parser: fn(&'a str) -> Result<(K, V)>) -> Result<HashMap<K, V>>
    where
        K: Eq + Hash,
    {
        self.0.lines().map(parser).collect()
    }

    pub fn lines_hset<T>(&self, parser: fn(&'a str) -> Result<T>) -> Result<HashSet<T>>
    where
        T: Eq + Hash,
    {
        self.0.lines().map(parser).collect()
    }

    pub fn grid<T>(&self, parser: fn(char) -> Result<T>) -> Result<Vec<Vec<T>>> {
        self.0
            .lines()
            .map(|line| line.chars().map(parser).collect())
            .collect()
    }

    /*
    // No use for it yet! So I prefer to not include it and change it if needed.
    pub fn loc_grid<T>(
        &self,
        parser: fn((usize, usize), char) -> Result<T>,
    ) -> Result<Vec<Vec<T>>> {
        self.0
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c, ch)| parser((r, c), ch))
                    .collect()
            })
            .collect()
    }
    */
}

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
