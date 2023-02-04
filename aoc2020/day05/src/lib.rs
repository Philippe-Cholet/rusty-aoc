use itertools::Itertools;

use common::{bail, ensure, Context, Part, Part1, Part2, Result};
use utils::OkIterator;

macro_rules! seat_position {
    ($s:expr, $lower:literal, $upper:literal) => {
        $s.chars()
            .map(|ch| match ch {
                $lower => Ok(false),
                $upper => Ok(true),
                _ => bail!("Not {}/{} but {}", $lower, $upper, ch),
            })
            .ok_fold(0, |res, b| (res << 1) | (b as u16))
    };
}

/// Binary Boarding
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut seat_ids = input
        .lines()
        .map(|line| {
            ensure!(line.len() == 10, "Not 10 long");
            let row = seat_position!(line[..7], 'F', 'B')?;
            let col = seat_position!(line[7..], 'L', 'R')?;
            // println!("{:?}", (row, col));
            Ok((row << 3) | col)
        })
        .ok_collect_vec()?;
    match part {
        Part1 => seat_ids.into_iter().max().context("No seat?!"),
        Part2 => {
            seat_ids.sort_unstable();
            seat_ids
                .into_iter()
                .tuple_windows()
                .find_map(|(prev, next)| (prev + 1 != next).then_some(prev + 1))
                .context("No missing seat ID?!")
        }
    }
    .map(|seat_id| seat_id.to_string())
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_20_05() -> Result<()> {
    assert_eq!(solver(Part1, "BFFFBBFRRR")?, "567");
    assert_eq!(solver(Part1, "FFFBBBFRRR")?, "119");
    assert_eq!(solver(Part1, "BBFFBBFRLL")?, "820");
    assert_eq!(solver(Part1, INPUTS[0])?, "989");
    assert_eq!(solver(Part2, INPUTS[0])?, "548");
    Ok(())
}
