use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

fn range_inclusive(a: u32, b: u32) -> impl Iterator<Item = u32> {
    let x: Box<dyn Iterator<Item = u32>> = if b > a {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    };
    x
}

/// Hydrothermal Venture
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let data = input
        .lines()
        .map(|line| {
            line.split(&[',', ' ', '-', '>'])
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .with_context(|| format!("Not 4 integers: {line}"))
        })
        .ok_collect_vec()?;
    let mut counts = HashMap::<(u32, u32), u8>::new();
    let mut increment = |pt| *counts.entry(pt).or_default() += 1;
    for (x1, y1, x2, y2) in data {
        if x1 == x2 {
            range_inclusive(y1, y2).for_each(|y| increment((x1, y)));
        } else if y1 == y2 {
            range_inclusive(x1, x2).for_each(|x| increment((x, y1)));
        } else if part.two() {
            range_inclusive(x1, x2)
                .zip(range_inclusive(y1, y2))
                .for_each(&mut increment);
        }
    }
    Ok(counts.into_values().filter(|&value| value > 1).count())
}

test_solver! {
    "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
" => (5, 12),
    include_input!(21 05) => (5698, 15463),
}
