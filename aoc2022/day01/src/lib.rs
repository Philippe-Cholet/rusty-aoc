use std::collections::BinaryHeap; // max-heap

use common::{Context, Part, Part1, Part2, Result};

/// Calorie Counting
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut counts: BinaryHeap<u32> = input
        .split("\n\n")
        .map(|snacks| snacks.lines().map(str::parse::<u32>).sum())
        .collect::<Result<_, _>>()?;
    // No need to pop when peek is enough.
    Ok(match part {
        Part1 => *counts.peek().context("No elf")?,
        Part2 => {
            counts.pop().context("No elf")?
                + counts.pop().context("Only one elf")?
                + *counts.peek().context("Only two elves")?
        }
    }
    .to_string())
}

pub const INPUTS: [&str; 2] = [
    "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
",
    include_str!("input.txt"), // 259 elves
];

#[test]
fn solver_22_01() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "24000");
    assert_eq!(solver(Part1, INPUTS[1])?, "71124");
    assert_eq!(solver(Part2, INPUTS[0])?, "45000");
    assert_eq!(solver(Part2, INPUTS[1])?, "204639");
    Ok(())
}
