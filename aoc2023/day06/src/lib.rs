use itertools::Itertools;

use common::prelude::*;

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::suboptimal_flops
)]
fn race_nb_solutions(time: i64, distance: i64) -> i64 {
    let eq = |a, b| i64::from(f64::abs(a - b) < 1e-10);
    let t = time as f64;
    // (t - x) * x > d  so  x² - t * x + d < 0
    let delta = t * t - 4. * distance as f64;
    if delta < 0.0 {
        // No solution
        0
    } else {
        let d = delta.sqrt();
        let (m0, m1) = ((t - d) / 2., (t + d) / 2.);
        let (i0, i1) = (m0.ceil(), m1.floor());
        // min <= x <= max
        let min = i0 as i64 + eq(i0, m0);
        let max = i1 as i64 - eq(i1, m1);
        max - min + 1
    }
}

/// Wait For It
pub fn solver(part: Part, input: &str) -> Result<i64> {
    let (mut time, mut dist) = input.lines().collect_tuple().context("Not 2 lines")?;
    time = time.strip_prefix("Time:").context("Wrong prefix")?;
    dist = dist.strip_prefix("Distance:").context("Wrong prefix")?;
    Ok(match part {
        Part1 => {
            let times: Vec<_> = time.split_whitespace().map(str::parse).try_collect()?;
            let distances: Vec<_> = dist.split_whitespace().map(str::parse).try_collect()?;
            ensure!(times.len() == distances.len(), "Half race data");
            times
                .into_iter()
                .zip(distances)
                .map(|(t, d)| race_nb_solutions(t, d))
                .product()
        }
        Part2 => {
            let t = time.replace(' ', "").parse()?;
            let d = dist.replace(' ', "").parse()?;
            race_nb_solutions(t, d)
        }
    })
}

pub const INPUTS: [&str; 2] = [
    "\
Time:      7  15   30
Distance:  9  40  200
",
    include_str!("input.txt"),
];

#[test]
fn solver_23_06() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 288); // 4 * 8 * 9
    assert_eq!(solver(Part1, INPUTS[1])?, 505494);
    assert_eq!(solver(Part2, INPUTS[0])?, 71503);
    assert_eq!(solver(Part2, INPUTS[1])?, 23632299);
    Ok(())
}
