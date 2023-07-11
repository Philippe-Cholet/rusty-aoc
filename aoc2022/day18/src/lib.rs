use std::collections::HashSet;

use itertools::Itertools;

use common::{prelude::*, Ok};
use utils::OkIterator;

const fn neighbors((x, y, z): (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

/// Boiling Boulders
pub fn solver(part: Part, input: &str) -> Result<String> {
    let lava = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(str::parse::<i32>)
                .collect_tuple()
                .context("Not 3 elements")?;
            Ok((x?, y?, z?))
        })
        .ok_collect_hset()?;
    let result = match part {
        Part1 => lava
            .iter()
            .flat_map(|&point| neighbors(point))
            .filter(|point| !lava.contains(point))
            .count(),
        Part2 => {
            let (x0, x1) = lava
                .iter()
                .map(|point| point.0)
                .minmax()
                .into_option()
                .context("no lava")?;
            let (y0, y1) = lava
                .iter()
                .map(|point| point.1)
                .minmax()
                .into_option()
                .context("no lava")?;
            let (z0, z1) = lava
                .iter()
                .map(|point| point.2)
                .minmax()
                .into_option()
                .context("no lava")?;
            // Extend a bit to strictly contain all lava in large enough cuboid.
            let (x0, x1, y0, y1, z0, z1) = (x0 - 1, x1 + 1, y0 - 1, y1 + 1, z0 - 1, z1 + 1);
            let mut stack = vec![(x0, y0, z0)]; // start outside
            let mut outside = HashSet::new(); // visited outside air
            let mut lava_surface = HashSet::new(); // (outside, lava) connections
            while let Some(old) = stack.pop() {
                if outside.contains(&old) {
                    continue;
                }
                outside.insert(old);
                for new in neighbors(old) {
                    if x0 <= new.0
                        && y0 <= new.1
                        && z0 <= new.2
                        && new.0 <= x1
                        && new.1 <= y1
                        && new.2 <= z1
                    {
                        if lava.contains(&new) {
                            lava_surface.insert((old, new));
                        } else if !outside.contains(&new) {
                            stack.push(new);
                        }
                    }
                }
            }
            lava_surface.len()
        }
    };
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_18() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "64");
    assert_eq!(solver(Part1, INPUTS[1])?, "3494");
    assert_eq!(solver(Part2, INPUTS[0])?, "58");
    assert_eq!(solver(Part2, INPUTS[1])?, "2062");
    Ok(())
}
