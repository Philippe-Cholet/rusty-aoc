use itertools::Itertools;

use common::{prelude::*, Ok};
use crate::utils::OkIterator;

/// Reindeer Olympics
pub fn solver(part: Part, input: &str) -> Result<u16> {
    let data: Vec<[u16; 3]> = input
        .lines()
        .map(|line| {
            let (s, time_resting) = line
                .strip_suffix(" seconds.")
                .context("Wrong suffix")?
                .split_once(" seconds, but then must rest for ")
                .context("Wrong delimiter")?;
            let (s, time_active) = s.split_once(" km/s for ").context("Not km/s for")?;
            let (_name, speed) = s.split_once(" can fly ").context("Not can fly")?;
            Ok([speed.parse()?, time_active.parse()?, time_resting.parse()?])
        })
        .ok_collect()?;
    let nb_reindeers = data.len();
    let race_secs = if nb_reindeers == 2 { 1000 } else { 2503 };
    match part {
        Part1 => data
            .into_iter()
            .map(|[speed, time_active, time_resting]| {
                let nb_cycles = race_secs / (time_active + time_resting);
                let end_active = time_active.min(race_secs % (time_active + time_resting));
                (nb_cycles * time_active + end_active) * speed
            })
            .max(),
        Part2 => {
            let reindeer_positions = data
                .into_iter()
                .map(|[speed, time_active, time_resting]| {
                    let nb_cycles = race_secs / (time_active + time_resting);
                    let end_active = time_active.min(race_secs % (time_active + time_resting));
                    let mut pos = 0;
                    let mut positions = vec![];
                    for _ in 0..nb_cycles {
                        for _ in 0..time_active {
                            pos += speed;
                            positions.push(pos);
                        }
                        for _ in 0..time_resting {
                            positions.push(pos);
                        }
                    }
                    for _ in 0..end_active {
                        pos += speed;
                        positions.push(pos);
                    }
                    for _ in 0..race_secs as usize - positions.len() {
                        positions.push(pos);
                    }
                    positions
                })
                .collect_vec();
            let mut pts = vec![0; nb_reindeers];
            for second in 0..race_secs as usize {
                let idx_winners = reindeer_positions
                    .iter()
                    .map(|positions| positions[second])
                    .enumerate()
                    .max_set_by_key(|(_, pos)| *pos);
                for (idx, _) in idx_winners {
                    pts[idx] += 1;
                }
            }
            pts.into_iter().max()
        }
    }
    .context("No reindeer racing")
}

pub const INPUTS: [&str; 2] = [
    "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
",
    include_input!(15 14),
];

#[test]
fn solver_15_14() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 1120);
    assert_eq!(solver(Part1, INPUTS[1])?, 2640);
    assert_eq!(solver(Part2, INPUTS[0])?, 689);
    assert_eq!(solver(Part2, INPUTS[1])?, 1102);
    Ok(())
}
