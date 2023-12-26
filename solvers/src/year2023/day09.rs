use std::fmt;
use std::ops::Index;

use itertools::Itertools;

use common::prelude::*;

struct CachedBinomials {
    data: Vec<i64>,
    width: usize,
}

/// Mirage Maintenance
pub fn solver(part: Part, input: &str) -> Result<i64> {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.split_whitespace().map(str::parse).try_collect())
        .try_collect()?;
    let max_width = data.iter().map(Vec::len).max().unwrap_or_default();
    let binomials = CachedBinomials::new(max_width);
    #[cfg(debug_assertions)]
    println!("{binomials:?}");
    Ok(data
        .into_iter()
        .map(|line| {
            let mut v = line.clone();
            let mut n = 1;
            loop {
                let mut all_zeros = true;
                v = v
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| b - a)
                    .inspect(|diff| all_zeros &= diff == &0)
                    .collect_vec();
                if all_zeros {
                    break (line, n);
                }
                n += 1;
            }
        })
        .flat_map(|(mut line, n)| {
            if part.one() {
                line.reverse();
            }
            itertools::izip!(&binomials[n][1..], line, [1, -1].iter().cycle())
                .map(|(binom, val, e)| binom * i64::from(val) * e)
        })
        .sum())
}

impl CachedBinomials {
    fn new(width: usize) -> Self {
        // data[n * width + k] == factorial(n) / factorial(n - k) / factorial(k)
        let mut data = vec![0; width * width];
        // First column
        for n in 0..width {
            data[n * width] = 1;
        }
        // Apply recursion formula for the rest of the grid
        for n in 1..width {
            for k in 1..=n {
                data[n * width + k] = data[(n - 1) * width + k - 1] + data[(n - 1) * width + k];
            }
        }
        Self { data, width }
    }
}

impl Index<usize> for CachedBinomials {
    type Output = [i64];
    #[inline]
    fn index(&self, n: usize) -> &[i64] {
        &self.data[n * self.width..][..=n]
    }
}

impl fmt::Debug for CachedBinomials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries((0..self.width).map(|n| &self[n]).collect_vec())
            .finish()
    }
}

pub const INPUTS: [&str; 2] = [
    "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
",
    include_input!(23 09),
];

#[test]
fn solver_23_09() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 114); // 18 + 28 + 68
    assert_eq!(solver(Part1, INPUTS[1])?, 1987402313);
    assert_eq!(solver(Part2, INPUTS[0])?, 2); // -3 + 0 + 5
    assert_eq!(solver(Part2, INPUTS[1])?, 900);
    Ok(())
}
