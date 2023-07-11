use std::str::FromStr;

use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

type Worry = u64;

#[derive(Debug)]
struct Monkey {
    index: usize,
    items: Vec<Worry>,
    inspection: MonkeyInspection,
    divisor: Worry,
    then_throw_to: [usize; 2], // false 0 ; true 1
    inspected_items: usize,
}

#[derive(Debug)]
enum MonkeyInspection {
    Add(Option<Worry>),
    Mul(Option<Worry>),
}

#[derive(Debug)]
enum WorryManagement {
    DividedBy3,
    Explode(Worry),
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (index, items, inspection, divisor, if_true, if_false) =
            s.lines().collect_tuple().context("Not 6 lines")?;
        Ok(Self {
            index: index
                .strip_prefix("Monkey ")
                .context("Monkey")?
                .strip_suffix(':')
                .context(":")?
                .parse()?,
            items: items
                .strip_prefix("  Starting items: ")
                .context("Starting")?
                .split(", ")
                .map(str::parse)
                .ok_collect_vec()?,
            inspection: inspection
                .strip_prefix("  Operation: new = old ")
                .context("Operation")?
                .parse()?,
            divisor: divisor
                .strip_prefix("  Test: divisible by ")
                .context("divisible")?
                .parse()?,
            then_throw_to: [
                if_false
                    .strip_prefix("    If false: throw to monkey ")
                    .context("if false")?
                    .parse()?,
                if_true
                    .strip_prefix("    If true: throw to monkey ")
                    .context("if true")?
                    .parse()?,
            ],
            inspected_items: 0,
        })
    }
}

impl FromStr for MonkeyInspection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (op, s) = s.split_once(' ').context("No whitespace")?;
        let value = match s {
            "old" => None,
            s => Some(s.parse()?),
        };
        Ok(match op {
            "+" => Self::Add(value),
            "*" => Self::Mul(value),
            _ => bail!("Wrong operation: {}", op),
        })
    }
}

impl Monkey {
    fn throw_items(&mut self, manager: &WorryManagement) -> Vec<(usize, Worry)> {
        self.items
            .drain(..)
            .map(|mut item| {
                self.inspection.handle(&mut item);
                self.inspected_items += 1;
                manager.handle(&mut item);
                let idx: usize = (item % self.divisor == 0).into();
                (self.then_throw_to[idx], item)
            })
            .collect()
    }

    fn catch_item(&mut self, item: Worry) {
        self.items.push(item);
    }
}

impl MonkeyInspection {
    fn handle(&self, worry: &mut Worry) {
        match self {
            Self::Add(None) => *worry += *worry,
            Self::Add(Some(value)) => *worry += value,
            Self::Mul(None) => *worry *= *worry,
            Self::Mul(Some(value)) => *worry *= value,
        }
    }
}

impl WorryManagement {
    fn handle(&self, worry: &mut Worry) {
        match self {
            Self::DividedBy3 => *worry /= 3,
            Self::Explode(lcm) => *worry %= lcm,
        }
    }
}

/// Monkey in the Middle
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(str::parse).ok_collect()?;
    ensure!(
        monkeys.iter().enumerate().all(|(i, m)| i == m.index),
        "Unordered monkey indexes",
    );
    let nb_monkeys = monkeys.len();
    let (nb_rounds, worry_management) = match part {
        Part1 => (20, WorryManagement::DividedBy3),
        Part2 => {
            // (Least) Common Multiple  (well, here it is the product not the least,
            // but they are different primes numbers, so yeah it's the least in tests).
            let lcm = monkeys.iter().map(|m| m.divisor).product();
            (10000, WorryManagement::Explode(lcm))
        }
    };
    for _round in 1..=nb_rounds {
        for monkey_id in 0..nb_monkeys {
            for (index, worry) in monkeys[monkey_id].throw_items(&worry_management) {
                monkeys[index].catch_item(worry);
            }
        }
        /*
        match part {
            Part1 => {
                println!("Round {:?}", round);
                for m in &monkeys {
                    println!("    {:?}", m.items);
                }
            }
            Part2 => {
                if round == 1 || round == 20 || round % 1000 == 0 {
                    println!(
                        "Round {}: {:?}",
                        round,
                        monkeys
                            .iter()
                            .map(|m| m.inspected_items)
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }
        */
    }
    let result: usize = monkeys
        .into_iter()
        .map(|m| m.inspected_items)
        // https://github.com/rust-itertools/itertools/issues/586
        // there is not a `k_largest` so...
        .map(std::cmp::Reverse) // reverse comparisons
        .k_smallest(2) // get smallests
        .map(|rev| rev.0) // unpack reversed
        .product();
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_11() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "10605");
    assert_eq!(solver(Part1, INPUTS[1])?, "99852");
    assert_eq!(solver(Part2, INPUTS[0])?, "2713310158");
    assert_eq!(solver(Part2, INPUTS[1])?, "25935263541");
    Ok(())
}
