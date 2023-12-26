use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

/// Password Philosophy
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let data = input
        .lines()
        .map(|line| {
            let (policy, pwd) = line.split_once(": ").context("no colon")?;
            let (ns, letter) = policy.split_once(' ').context("no space")?;
            let (n1, n2) = ns.split_once('-').context("no dash")?;
            let (n1, n2) = (n1.parse::<usize>()?, n2.parse::<usize>()?);
            ensure!(n1 >= 1 && n2 >= 1, "non-zero positions in Part2");
            letter
                .chars()
                .exactly_one()
                .ok()
                .context("The letter is not exactly one char")
                .map(|ch| (n1, n2, ch, pwd))
        })
        .ok_collect_vec()?;
    Ok(data
        .into_iter()
        .filter(|(n1, n2, ch, pwd)| match part {
            Part1 => {
                let count = pwd.chars().filter(|c| c == ch).count();
                n1 <= &count && &count <= n2
            }
            Part2 => {
                (pwd.chars().nth(*n1 - 1) == Some(*ch)) ^ (pwd.chars().nth(*n2 - 1) == Some(*ch))
            }
        })
        .count())
}

pub const INPUTS: [&str; 2] = [
    "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
",
    include_input!(20 02),
];

#[test]
fn solver_20_02() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 2);
    assert_eq!(solver(Part1, INPUTS[1])?, 564);
    assert_eq!(solver(Part2, INPUTS[0])?, 1);
    assert_eq!(solver(Part2, INPUTS[1])?, 325);
    Ok(())
}
