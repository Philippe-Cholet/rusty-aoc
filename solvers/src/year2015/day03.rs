use common::prelude::*;
use crate::utils::OkIterator;

/// Perfectly Spherical Houses in a Vacuum
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let data = input
        .trim_end()
        .chars()
        .map(|ch| {
            Ok(match ch {
                '^' => (-1, 0),
                'v' => (1, 0),
                '>' => (0, 1),
                '<' => (0, -1),
                _ => bail!("Expected ^ v > or < but got {:?}.", ch),
            })
        })
        .ok_collect_vec()?;
    let mut houses: HashSet<_> = match part {
        Part1 => {
            let mut loc = (0, 0);
            data.into_iter()
                .map(|(dr, dc)| {
                    loc.0 += dr;
                    loc.1 += dc;
                    loc
                })
                .collect()
        }
        Part2 => {
            let mut locs = [(0, 0); 2];
            data.into_iter()
                .enumerate()
                .map(|(idx, (dr, dc))| {
                    let loc = &mut locs[idx % 2];
                    loc.0 += dr;
                    loc.1 += dc;
                    *loc
                })
                .collect()
        }
    };
    houses.insert((0, 0));
    Ok(houses.len())
}

pub const INPUTS: [&str; 1] = [include_input!(15 03)];

#[test]
fn solver_15_03() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 2572);
    assert_eq!(solver(Part2, INPUTS[0])?, 2631);
    Ok(())
}
