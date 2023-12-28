use common::prelude::*;
use crate::utils::OkIterator;

/// Calorie Counting
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let mut counts = input
        .split("\n\n")
        .map(|snacks| snacks.lines().map(str::parse::<u32>).ok_sum::<u32>())
        .ok_collect_heap()?;
    // No need to pop when peek is enough.
    Ok(match part {
        Part1 => *counts.peek().context("No elf")?,
        Part2 => {
            counts.pop().context("No elf")?
                + counts.pop().context("Only one elf")?
                + *counts.peek().context("Only two elves")?
        }
    })
}

test_solver! {
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
" => (24000, 45000),
    include_input!(22 01) => (71124, 204639), // 259 elves
}
