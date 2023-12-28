use common::prelude::*;
use crate::utils::OkIterator;

/// Not Quite Lisp
pub fn solver(part: Part, input: &str) -> Result<i32> {
    let mut ns = input.trim_end().chars().map(|ch| match ch {
        '(' => Ok(1),
        ')' => Ok(-1),
        _ => bail!("Expected ( or ) but got {:?}.", ch),
    });
    match part {
        Part1 => ns.sum(),
        Part2 => {
            let mut floor = 0;
            let pos = ns
                .ok_position(|n| {
                    floor += n;
                    floor == -1
                })?
                .context("Did not reach the basement.")?;
            Ok(i32::try_from(1 + pos)?)
        }
    }
}

test_solver!(include_input!(15 01) => (280, 1797));
