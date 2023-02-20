use common::{bail, Context, Part, Part1, Part2, Result};
use utils::OkIterator;

/// Not Quite Lisp
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut ns = input.trim_end().chars().map(|ch| match ch {
        '(' => Ok(1),
        ')' => Ok(-1),
        _ => bail!("Expected ( or ) but got {:?}.", ch),
    });
    Ok(match part {
        Part1 => ns.ok_sum::<i32>()?.to_string(),
        Part2 => {
            let mut floor = 0;
            (1 + ns
                .ok_position(|n| {
                    floor += n;
                    floor == -1
                })?
                .context("Did not reach the basement.")?)
            .to_string()
        }
    })
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_15_01() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "280");
    assert_eq!(solver(Part2, INPUTS[0])?, "1797");
    Ok(())
}
