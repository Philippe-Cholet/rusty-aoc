use common::prelude::*;
use utils::OkIterator;

use Instruction::{Mask, Mem};

const U36_MAX: u64 = (1 << 36) - 1;

enum Instruction {
    Mask(u64, u64), // X bits, other bits
    Mem { addr: u64, value: u64 },
}

enum DecoderChip {
    V1(HashMap<u64, u64>),
    V2(Vec<(Vec<u64>, u64)>),
}

/// Docking Data
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let initialization: Vec<Instruction> = input.lines().map(str::parse).ok_collect()?;
    #[cfg(debug_assertions)]
    for instruction in &initialization {
        println!("{instruction:b}");
    }
    let mut decoder = match part {
        Part1 => DecoderChip::V1(HashMap::new()),
        Part2 => DecoderChip::V2(vec![]),
    };
    let mut mask = (U36_MAX, 0); // 36X
    for instruction in initialization {
        match instruction {
            Mask(xs, bits) => mask = (xs, bits),
            Mem { addr, value } => decoder.access_memory(addr, value, mask),
        }
    }
    Ok(decoder.sum())
}

impl DecoderChip {
    fn access_memory(&mut self, addr: u64, value: u64, (xs, bits): (u64, u64)) {
        match self {
            Self::V1(hm) => {
                hm.insert(addr, (value & xs) | bits);
            }
            Self::V2(items) => {
                let mut addrs = Vec::with_capacity(1 << xs.count_ones());
                addrs.push((addr & (bits ^ U36_MAX)) | bits);
                for k in 0..36 {
                    let bit = 1 << k;
                    if xs & bit != 0 {
                        let swapped_addrs: Vec<_> = addrs.iter().map(|addr| addr ^ bit).collect();
                        addrs.extend(swapped_addrs);
                    }
                }
                // On my test, `addrs` is max 512 long (9X). A larger number of X could be bad!
                items.push((addrs, value));
            }
        }
    }

    fn sum(self) -> u64 {
        match self {
            Self::V1(hm) => hm.into_values().sum(),
            Self::V2(items) => {
                // Reserve enough memory is faster than do multiple reallocations later.
                let max_count_addrs = items.iter().map(|(addrs, _)| addrs.len()).sum();
                let mut addrs_in_memory = HashSet::with_capacity(max_count_addrs);
                items
                    .into_iter()
                    .rev()
                    .map(|(mut addrs, value)| {
                        addrs.retain(|addr| !addrs_in_memory.contains(addr));
                        let new_addrs = addrs.len() as u64;
                        addrs_in_memory.extend(addrs);
                        new_addrs * value
                    })
                    .sum()
            }
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (assign, nb) = s.split_once(" = ").context("No equal?!")?;
        if assign == "mask" {
            ensure!(nb.chars().count() == 36, "Not 36 long");
            nb.chars()
                .map(|ch| {
                    Ok(match ch {
                        'X' => None,
                        '0' => Some(false),
                        '1' => Some(true),
                        _ => bail!("Wrong bit: {}", ch),
                    })
                })
                .ok_fold((0, 0), |(xs, bits), opt_x| {
                    let (x, b) = opt_x.map_or((1, false), |b| (0, b));
                    ((xs << 1) | x, (bits << 1) | u64::from(b))
                })
                .map(|(xs, bits)| Mask(xs, bits))
        } else {
            let value = nb.parse()?;
            ensure!(
                assign.starts_with("mem[") && assign.ends_with(']'),
                "Not mem[...]",
            );
            let addr = assign[4..assign.len() - 1].parse()?;
            Ok(Mem { addr, value })
        }
    }
}

impl std::fmt::Binary for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mask(xs, bits) => write!(f, "Mask({xs:036b}, {bits:036b})"),
            Mem { addr, value } => write!(f, "Mem({addr}, {value})"),
        }
    }
}

pub const INPUTS: [&str; 3] = [
    "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
",
    "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
",
    include_str!("input.txt"),
];

#[test]
fn solver_20_14() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 165);
    assert_eq!(solver(Part1, INPUTS[2])?, 9615006043476);
    assert_eq!(solver(Part2, INPUTS[1])?, 208);
    assert_eq!(solver(Part2, INPUTS[2])?, 4275496544925);
    Ok(())
}
