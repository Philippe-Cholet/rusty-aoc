use common::prelude::*;
use crate::utils::OkIterator;

#[allow(clippy::cast_possible_truncation)] // SAFETY: `idx < numbers.len() <= nb_turns <= 30_000_000 <= u32::MAX`
/// Rambunctious Recitation
pub fn solver(part: Part, input: &str) -> Result<u32> {
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
    Ok(n)
}

test_solver! {
    "0,3,6" => (436, 175594),
    "1,3,2" => (1, 2578),
    "2,1,3" => (10, 3544142),
    "1,2,3" => (27, 261214),
    "2,3,1" => (78, 6895259),
    "3,2,1" => (438, 18),
    "3,1,2" => (1836, 362),
    include_input!(20 15) => (1696, 37385),
}
