use std::cmp::Ordering;

use itertools::iproduct;

use common::prelude::*;

/// Trick Shot
pub fn solver(part: Part, input: &str) -> Result<i32> {
    let (xs, ys) = input
        .trim_end()
        .strip_prefix("target area: x=")
        .context("wrong prefix")?
        .split_once(", y=")
        .context("wrong delimiter")?;
    let (x_min, x_max) = xs.split_once("..").context("wrong range")?;
    let (y_min, y_max) = ys.split_once("..").context("wrong range")?;
    // Would be nicer with the unstable `try_map` method.
    let [x_min, x_max, y_min, y_max] = [x_min, x_max, y_min, y_max].map(str::parse);
    let [x_min, x_max, y_min, y_max] = [x_min?, x_max?, y_min?, y_max?];
    debug_assert!(x_min >= 0 && y_max < 0);
    // There is no reason to limit `vy` other than keep things plausible.
    let shots = iproduct!(0..=x_max, y_min..=1000).filter_map(|(mut vx, mut vy)| {
        let mut x = 0;
        let mut y = 0;
        let mut max_height = 0;
        loop {
            x += vx;
            y += vy;
            if vy > 0 {
                max_height += vy;
            }
            if x_min <= x && x <= x_max && y_min <= y && y <= y_max {
                // Target found!
                break Some(max_height);
            }
            match vx.cmp(&0) {
                Ordering::Greater => vx -= 1,
                Ordering::Equal => {}
                Ordering::Less => vx += 1,
            }
            vy -= 1;
            if (vx == 0 && !(x_min <= x && x <= x_max)) || (vy <= 0 && y < y_min) {
                // Stuck horizontally outside of the target area OR Too deep!
                break None;
            }
        }
    });
    match part {
        Part1 => shots.max().context("Always Off Target!"),
        Part2 => Ok(shots.count().try_into()?),
    }
}

pub const INPUTS: [&str; 2] = [
    "target area: x=20..30, y=-10..-5\n",
    include_str!("input.txt"),
];

#[test]
fn solver_21_17() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 45);
    assert_eq!(solver(Part1, INPUTS[1])?, 10585);
    assert_eq!(solver(Part2, INPUTS[0])?, 112);
    assert_eq!(solver(Part2, INPUTS[1])?, 5247);
    Ok(())
}
