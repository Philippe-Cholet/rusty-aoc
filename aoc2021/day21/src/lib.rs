use std::cmp::{max, min};

use itertools::Itertools;

use common::{Context, Part, Part1, Part2, Result};

#[derive(Debug, Clone)]
struct Game {
    turn1: bool,
    player1: Player,
    player2: Player,
}

impl Game {
    fn forward(&mut self, moves: u8) {
        if self.turn1 {
            self.player1.forward(moves);
        } else {
            self.player2.forward(moves);
        }
        self.turn1 = !self.turn1;
    }
}

#[derive(Debug, Clone)]
struct Player {
    position: u8,
    score: usize,
}

impl Player {
    fn forward(&mut self, moves: u8) {
        debug_assert!(self.position > 0 && self.position <= 10);
        self.position = (self.position + moves - 1) % 10 + 1;
        self.score += self.position as usize;
    }
}

#[derive(Debug, Default)]
struct Dice100 {
    value: u8,
    nb_roll: usize,
}

impl Dice100 {
    fn roll(&mut self) -> u8 {
        self.value = (self.value % 100) + 1;
        self.nb_roll += 1;
        self.value % 10
    }

    fn roll3(&mut self) -> u8 {
        (self.roll() + self.roll() + self.roll()) % 10
    }
}

// Rolling a 3-sided dice 3 times have the followed (outputs, number of universes).
const DICE3_OPTIONS: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

/// Dirac Dice
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (line1, line2) = input.lines().collect_tuple().context("Not 2 lines")?;
    let mut game = Game {
        turn1: true,
        player1: Player {
            position: line1
                .strip_prefix("Player 1 starting position: ")
                .context("Wrong prefix")?
                .parse()?,
            score: 0,
        },
        player2: Player {
            position: line2
                .strip_prefix("Player 2 starting position: ")
                .context("Wrong prefix")?
                .parse()?,
            score: 0,
        },
    };
    Ok(match part {
        Part1 => {
            let mut dice = Dice100::default();
            loop {
                game.forward(dice.roll3());
                if max(game.player1.score, game.player2.score) >= 1000 {
                    let low_score = min(game.player1.score, game.player2.score);
                    break low_score * dice.nb_roll;
                }
            }
        }
        Part2 => {
            let (mut player1wins, mut player2wins) = (0, 0);
            let mut stack = vec![(1, game)];
            while let Some((count, game)) = stack.pop() {
                for (moves, nb_universes) in DICE3_OPTIONS {
                    let mut new_game = game.clone();
                    new_game.forward(moves);
                    let new_count = count * nb_universes;
                    if new_game.player1.score >= 21 {
                        player1wins += new_count;
                    } else if new_game.player2.score >= 21 {
                        player2wins += new_count;
                    } else {
                        stack.push((new_count, new_game));
                    }
                }
            }
            max(player1wins, player2wins)
        }
    }
    .to_string())
}

pub const INPUTS: [&str; 2] = [
    "Player 1 starting position: 4
Player 2 starting position: 8
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_21() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "739785");
    assert_eq!(solver(Part1, INPUTS[1])?, "888735");
    assert_eq!(solver(Part2, INPUTS[0])?, "444356092776315");
    assert_eq!(solver(Part2, INPUTS[1])?, "647608359455719");
    Ok(())
}
