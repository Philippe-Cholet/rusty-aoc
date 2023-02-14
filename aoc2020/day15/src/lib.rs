use common::{ensure, Part, Part1, Part2, Result};
use utils::OkIterator;

/// Rambunctious Recitation
pub fn solver(part: Part, input: &str) -> Result<String> {
    let numbers: Vec<u32> = input.trim_end().split(',').map(str::parse).ok_collect()?;
    ensure!(!numbers.is_empty(), "No number provided");
    let nb_turns = match part {
        Part1 => 2020,
        Part2 => 30_000_000,
    };
    // Bruteforce to reserve that much memory but the alternative is to use even slower hashmaps.
    let mut spoken = vec![[0, 0]; nb_turns as usize];
    let mut n = 0;
    let len = u32::try_from(numbers.len())?;
    for turn in 1..=nb_turns {
        n = if turn <= len {
            numbers[turn as usize - 1]
        } else {
            let [penul, last] = spoken[n as usize];
            last - penul
        };
        let last = spoken[n as usize][1];
        spoken[n as usize] = [if last == 0 { turn } else { last }, turn];
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
