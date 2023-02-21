use itertools::Itertools;

use common::{ensure, Part, Part1, Part2, Result};
use utils::{char10, OkIterator};

#[allow(clippy::expect_used)]
/// Elves Look, Elves Say
pub fn solver(part: Part, input: &str) -> Result<String> {
    let nb_steps = match part {
        Part1 => 40,
        Part2 => 50,
    };
    let mut ns = input
        .trim_end()
        .chars()
        .map(char10::<u8>)
        .ok_collect_vec()?;
    ensure!(
        ns.iter().all(|n| matches!(n, 1..=3)),
        "Look-and-say sequence consist of only 1s 2s and 3s.",
    );
    for _ in 0..nb_steps {
        ns = ns
            .into_iter()
            .group_by(|n| *n)
            .into_iter()
            .flat_map(|(n, g)| [g.count().try_into().expect("1, 2 or 3"), n])
            .collect();
    }
    Ok(ns.len().to_string())
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_15_10() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "252594");
    assert_eq!(solver(Part2, INPUTS[0])?, "3579328");
    Ok(())
}
