use std::collections::HashSet;

use common::{ensure, Context, Error, Part, Part1, Part2, Result};
use utils::OkIterator;

/// Crab Combat
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut ferris_combat: DeckGame = input.parse()?;
    let am_i_winning = match part {
        Part1 => ferris_combat.combat(),
        Part2 => ferris_combat.recursive_combat(),
    };
    Ok(ferris_combat.score(am_i_winning).to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DeckGame {
    deck1: Vec<u8>,
    deck2: Vec<u8>,
}

impl DeckGame {
    fn score(&self, deck1: bool) -> usize {
        if deck1 { &self.deck1 } else { &self.deck2 }
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, card)| (idx + 1) * *card as usize)
            .sum()
    }

    fn combat(&mut self) -> bool {
        while !self.deck1.is_empty() && !self.deck2.is_empty() {
            let c1 = self.deck1.remove(0);
            let c2 = self.deck2.remove(0);
            if c1 > c2 {
                self.deck1.extend([c1, c2]);
            } else {
                self.deck2.extend([c2, c1]);
            }
        }
        !self.deck1.is_empty()
    }

    fn recursive_combat(&mut self) -> bool {
        let mut history = HashSet::new();
        while !self.deck1.is_empty() && !self.deck2.is_empty() {
            if !history.insert(self.clone()) {
                return true;
            }
            let c1 = self.deck1.remove(0);
            let c2 = self.deck2.remove(0);
            let i1 = c1 as usize;
            let i2 = c2 as usize;
            let p1win_round = if i1 <= self.deck1.len() && i2 <= self.deck2.len() {
                Self {
                    deck1: self.deck1[..i1].to_vec(),
                    deck2: self.deck2[..i2].to_vec(),
                }
                .recursive_combat()
            } else {
                c1 > c2
            };
            if p1win_round {
                self.deck1.extend([c1, c2]);
            } else {
                self.deck2.extend([c2, c1]);
            }
        }
        !self.deck1.is_empty()
    }
}

impl std::str::FromStr for DeckGame {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (p1, p2) = s.split_once("\n\n").context("No empty line")?;
        let deck1 = p1
            .strip_prefix("Player 1:\n")
            .context("context")?
            .lines()
            .map(str::parse::<u8>)
            .ok_collect_vec()?;
        let deck2 = p2
            .strip_prefix("Player 2:\n")
            .context("context")?
            .lines()
            .map(str::parse::<u8>)
            .ok_collect_vec()?;
        let mut whole_deck = HashSet::new();
        ensure!(
            deck1
                .iter()
                .chain(deck2.iter())
                .all(|c| whole_deck.insert(c)),
            "Cards are not all unique!"
        );
        ensure!(
            deck1.len() == deck2.len(),
            "Players must start with the same number of cards"
        );
        Ok(Self { deck1, deck2 })
    }
}

pub const INPUTS: [&str; 2] = [
    "Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10\n",
    include_str!("input.txt"),
];

#[test]
fn solver_20_22() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "306");
    assert_eq!(solver(Part1, INPUTS[1])?, "29764");
    assert_eq!(solver(Part2, INPUTS[0])?, "291");
    assert_eq!(solver(Part2, INPUTS[1])?, "32588");
    Ok(())
}
