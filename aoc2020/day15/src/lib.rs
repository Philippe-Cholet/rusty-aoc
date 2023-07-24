use common::prelude::*;
use utils::OkIterator;

#[allow(clippy::cast_possible_truncation)] // SAFETY: `idx < numbers.len() <= nb_turns <= 30_000_000 <= u32::MAX`
/// Rambunctious Recitation
pub fn solver(part: Part, input: &str) -> Result<String> {
    let numbers: Vec<u32> = input.trim_end().split(',').map(str::parse).ok_collect()?;
    ensure!(!numbers.is_empty(), "No number provided");
    let nb_turns = part.value(2020, 30_000_000);
    ensure!(
        numbers.len() <= nb_turns as usize,
        "I assumed there were less numbers than turns",
    );
    // Bruteforce to reserve that much memory but the alternative is to use even slower hashmaps.
    let mut spoken = vec![u32::MAX; nb_turns as usize];
    let mut n = 0;
    for (idx, &nb) in numbers.iter().enumerate() {
        n = nb;
        spoken[n as usize] = idx as u32 + 1;
    }
    for turn in numbers.len() as u32..nb_turns {
        let last = std::mem::replace(&mut spoken[n as usize], turn);
        // let last = spoken[n as usize];
        // spoken[n as usize] = turn;
        n = turn.saturating_sub(last);
    }
    Ok(n.to_string())
}

pub const INPUTS: [&str; 8] = [
    "0,3,6",
    "1,3,2",
    "2,1,3",
    "1,2,3",
    "2,3,1",
    "3,2,1",
    "3,1,2",
    include_str!("input.txt"),
];

#[test]
fn solver_20_15() -> Result<()> {
    let answers1 = ["436", "1", "10", "27", "78", "438", "1836", "1696"];
    for (input, answer) in INPUTS.iter().zip(answers1) {
        assert_eq!(solver(Part1, input)?, answer);
    }
    let answers2 = [
        "175594", "2578", "3544142", "261214", "6895259", "18", "362", "37385",
    ];
    for (input, answer) in INPUTS.iter().zip(answers2) {
        assert_eq!(solver(Part2, input)?, answer);
    }
    Ok(())
}
