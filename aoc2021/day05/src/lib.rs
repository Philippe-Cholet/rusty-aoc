use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

fn range_inclusive(a: u32, b: u32) -> impl Iterator<Item = u32> {
    let x: Box<dyn Iterator<Item = u32>> = if b > a {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    };
    x
}

/// Hydrothermal Venture
pub fn solver(part: Part, input: &str) -> Result<String> {
    let data = input
        .lines()
        .map(|line| {
            line.split(&[',', ' ', '-', '>'])
                .filter_map(|s| s.parse().ok())
                .collect_tuple()
                .with_context(|| format!("Not 4 integers: {line}"))
        })
        .ok_collect_vec()?;
    let counts = data
        .into_iter()
        .flat_map(|(x1, y1, x2, y2)| {
            if x1 == x2 {
                range_inclusive(y1, y2).map(|y| (x1, y)).collect()
            } else if y1 == y2 {
                range_inclusive(x1, x2).map(|x| (x, y1)).collect()
            } else {
                match part {
                    Part1 => vec![],
                    Part2 => range_inclusive(x1, x2)
                        .zip(range_inclusive(y1, y2))
                        .collect(),
                }
            }
        })
        .counts();
    let result = counts.into_values().filter(|&value| value > 1).count();
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_05() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "5");
    assert_eq!(solver(Part1, INPUTS[1])?, "5698");
    assert_eq!(solver(Part2, INPUTS[0])?, "12");
    assert_eq!(solver(Part2, INPUTS[1])?, "15463");
    Ok(())
}
