use itertools::{iproduct, Itertools};

use common::prelude::*;

/// Dirac Dice
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (line1, line2) = input.lines().collect_tuple().context("Not 2 lines")?;
    let pos1: usize = line1
        .strip_prefix("Player 1 starting position: ")
        .context("Wrong prefix")?
        .parse()?;
    let pos2: usize = line2
        .strip_prefix("Player 2 starting position: ")
        .context("Wrong prefix")?
        .parse()?;
    match part {
        Part1 => {
            let (mut positions, mut scores) = ([pos1, pos2], [0, 0]);
            let mut deterministic_dice = (1..=100).cycle().enumerate();
            for idx in [0, 1].into_iter().cycle() {
                let moves: usize = deterministic_dice
                    .by_ref()
                    .take(3)
                    .map(|(_step, mov)| mov)
                    .sum();
                let new_position = (positions[idx] + moves - 1) % 10 + 1;
                positions[idx] = new_position;
                scores[idx] += new_position;
                if scores[idx] >= 1000 {
                    let losing_score = scores[1 - idx];
                    let nb_rolls = deterministic_dice.next().context("Broken dice?!")?.0;
                    return Ok((losing_score * nb_rolls).to_string());
                }
            }
            unreachable!("Endless loop...");
        }
        Part2 => {
            let (player1, mut wins1) = (player_universes(pos1), 0);
            let (player2, mut wins2) = (player_universes(pos2), 0);
            for turn in 1..=MAX_TURN {
                // player2 still loses at the previous turn AND player1 finally wins
                wins1 += player2[turn - 1].0 * player1[turn].1;
                // player1 still loses AND player2 finally wins
                wins2 += player1[turn].0 * player2[turn].1;
            }
            let most_wins = wins1.max(wins2);
            Ok(most_wins.to_string())
            // The only times player 2 wins in more universes than player 1 is when player 2
            // starts at position 1 and player 1 starts in a positions 3..=8 (so 6 cases on 100).
        }
    }
}

// Rolling a 3-sided dice 3 times have the followed (outputs, number of universes).
const DICE3_OPTIONS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
// No matter what position we start from, we reach a score of 21 at most at turn 10.
const MAX_SCORE: usize = 21;
const MAX_TURN: usize = 10;

/// `[turn: (nb universes where score < MAX_SCORE, nb universes where player wins)]`
fn player_universes(start: usize) -> [(usize, usize); MAX_TURN + 1] {
    // nb_universes = all_universes[turn][score][position - 1]
    let mut all_universes = [[[0; 10]; MAX_SCORE + 1]; MAX_TURN + 1];
    // Turn 0: Only 1 universe, with position `start` and score 0.
    all_universes[0][0][start - 1] = 1;
    // Get a turn based on the previous one.
    for (turn, losing_score, pos, (moves, nb_universes)) in
        iproduct!(1..=MAX_TURN, 0..MAX_SCORE, 1..=10, DICE3_OPTIONS)
    {
        let new_pos = (pos + moves - 1) % 10 + 1;
        let new_score = MAX_SCORE.min(losing_score + new_pos);
        all_universes[turn][new_score][new_pos - 1] +=
            all_universes[turn - 1][losing_score][pos - 1] * nb_universes;
    }
    all_universes.map(|at_turn| {
        (
            at_turn[..MAX_SCORE].iter().flatten().sum(),
            at_turn[MAX_SCORE].into_iter().sum(),
        )
    })
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
