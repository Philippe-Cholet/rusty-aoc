use itertools::Itertools;

use common::prelude::*;

/// Scratchcards
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let data: Vec<u8> = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (card, cards) = line.split_once(':').context("colon delimiter")?;
            let (card, id) = card.split_whitespace().collect_tuple().context("Card #")?;
            ensure!(card == "Card");
            ensure!(id.parse::<usize>()? == idx + 1);
            let (winning, my) = cards.split_once('|').context("| delimiter")?;
            let winning = winning.split_whitespace().collect_vec();
            Ok(my
                .split_whitespace()
                .filter(|nb| winning.contains(nb))
                .count() // I do not have more than `u8::MAX` winning cards.
                .try_into()?)
        })
        .try_collect()?;
    Ok(match part {
        Part1 => data
            .iter()
            .filter_map(|n| n.checked_sub(1).map(|times_doubled| 1u32 << times_doubled))
            .sum(),
        Part2 => {
            let mut counts = vec![1; data.len()];
            for (idx, new_cards) in data.iter().enumerate().rev() {
                counts[idx] += counts[idx + 1..][..*new_cards as usize]
                    .iter()
                    .copied()
                    .sum::<u32>();
            }
            counts.into_iter().sum()
        }
    })
}

test_solver! {
    "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
" => (13, 30),
    include_input!(23 04) => (17803, 5554894),
}
