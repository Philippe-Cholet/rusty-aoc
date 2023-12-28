use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

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

test_solver! {
    "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
" => (2, 1),
    include_input!(20 02) => (564, 325),
}
