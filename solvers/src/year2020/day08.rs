use common::prelude::*;
use crate::utils::OkIterator;

/// Handheld Halting
pub fn solver(part: Part, input: &str) -> Result<i32> {
    let mut sequence = BootSequence(input.lines().map(str::parse).ok_collect()?);
    match part {
        Part1 => sequence.run().map(|(accum, _)| accum),
        Part2 => sequence.switch_one_to_boot(),
    }
}

struct BootSequence(Vec<Operation>);

#[derive(Debug)]
enum Operation {
    Accumulator(i32),
    Jump(i32),
    Nothing(i32),
}

impl std::str::FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (op, n) = s.split_once(' ').context("No space")?;
        let n = n.parse()?;
        Ok(match op {
            "acc" => Self::Accumulator(n),
            "jmp" => Self::Jump(n),
            "nop" => Self::Nothing(n),
            _ => bail!("Wrong operation: {}", op),
        })
    }
}

impl BootSequence {
    fn run(&self) -> Result<(i32, bool)> {
        let last_idx = self.0.len().checked_sub(1).context("Empty sequence")?;
        let mut acc = 0;
        let mut idx = 0;
        let mut used = vec![false; self.0.len()];
        let success = loop {
            let Some(op) = self.0.get(idx) else {
                bail!("Index out of range");
            };
            if used[idx] {
                break false;
            }
            used[idx] = true;
            match op {
                Operation::Accumulator(n) => {
                    acc += n;
                    idx += 1;
                }
                Operation::Jump(n) => {
                    idx = idx
                        .checked_add_signed(*n as isize)
                        .context("Index overflow")?;
                }
                Operation::Nothing(_) => idx += 1,
            }
            if used[last_idx] {
                break true;
            }
        };
        Ok((acc, success))
    }

    fn switch_one_to_boot(&mut self) -> Result<i32> {
        for idx in 0..self.0.len() {
            if self.0[idx].switch() {
                let (value, fixed) = self.run()?;
                if fixed {
                    return Ok(value);
                }
                let back = self.0[idx].switch();
                debug_assert!(back, "Not switching back");
            }
        }
        bail!("Switch one operation does not fix the boot sequence.");
    }
}

impl Operation {
    fn switch(&mut self) -> bool {
        match self {
            Self::Accumulator(_) => false,
            Self::Jump(n) => {
                *self = Self::Nothing(*n);
                true
            }
            Self::Nothing(n) => {
                *self = Self::Jump(*n);
                true
            }
        }
    }
}

test_solver! {
    "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
" => (5, 8),
    include_input!(20 08) => (1654, 833),
}
