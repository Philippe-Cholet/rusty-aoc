use itertools::Itertools;

use common::prelude::*;
use utils::neighbors;

/// Gear Ratios
pub fn solver(part: Part, input: &str) -> Result<u32> {
    // Collect numbers and location of each symbol.
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();
    #[allow(clippy::expect_used)]
    let grid = input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            let mut in_nb = false;
            line.chars()
                .enumerate()
                .map(|(c, ch)| {
                    if let Some(digit) = ch.to_digit(10) {
                        if in_nb {
                            let nb = numbers.last_mut().expect("In number but no number?!");
                            *nb = *nb * 10 + digit;
                        } else {
                            in_nb = true;
                            numbers.push(digit);
                        }
                        // Can't substract with overflow.
                        Some(numbers.len() - 1)
                    } else {
                        if ch != '.' {
                            symbols.insert((r, c), ch);
                        }
                        in_nb = false;
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    Ok(match part {
        // Unique neighboring numbers of all symbols, sum.
        Part1 => symbols
            .iter()
            .flat_map(|(&loc, _)| {
                neighbors(loc, usize::MAX, usize::MAX, true)
                    .into_iter()
                    .filter_map(|(r, c)| *grid.get(r)?.get(c)?)
            })
            .unique()
            .map(|idx| numbers[idx])
            .sum(),
        // Neighboring numbers of stars, multiply & sum.
        Part2 => {
            // A symbol could have up to 6 neighboring numbers.
            let mut idxs = HashSet::with_capacity(6); // more global to limit re-allocations
            symbols
                .iter()
                .filter_map(|(loc, symbol)| (*symbol == '*').then_some(*loc))
                .map(|loc| {
                    idxs.clear();
                    idxs.extend(
                        neighbors(loc, usize::MAX, usize::MAX, true)
                            .into_iter()
                            .filter_map(|(r, c)| *grid.get(r)?.get(c)?),
                    );
                    if idxs.len() > 1 {
                        // debug_assert_eq!(idxs.len(), 2);
                        idxs.iter().map(|&idx| numbers[idx]).product()
                    } else {
                        0
                    }
                })
                .sum()
        }
    })
}

pub const INPUTS: [&str; 2] = [
    "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
    include_str!("input.txt"),
];

#[test]
fn solver_23_03() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 4361);
    assert_eq!(solver(Part1, INPUTS[1])?, 527364);
    assert_eq!(solver(Part2, INPUTS[0])?, 467835);
    assert_eq!(solver(Part2, INPUTS[1])?, 79026871);
    Ok(())
}
