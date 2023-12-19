use itertools::{Either, Itertools};

use common::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Operation {
    GreaterThan,
    LowerThan,
}

#[derive(Debug, Clone)]
struct Condition {
    xmas_idx: u8, // 0 1 2 or 3
    op: Operation,
    value: u32,
}

#[derive(Debug)]
struct Item<T> {
    cond: Option<Condition>,
    // Name OR index VS Accepted/Rejected (true/false)
    result: Either<T, bool>,
}

type NamedWorkflows<'a> = Vec<(&'a str, Vec<Item<&'a str>>)>;
type IndexedWorkflows = Vec<Vec<Item<usize>>>;

/// Aplenty
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let (workflows, ratings) = input.split_once("\n\n").context("No empty line")?;
    let workflows: NamedWorkflows = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line
                .strip_suffix('}')
                .context("Not }")?
                .split_once('{')
                .context("Not {")?;
            rest.split(',')
                .map(Item::try_from)
                .try_collect()
                .map(|items| (name, items))
        })
        .try_collect()?;
    let names = workflows.iter().map(|(name, _)| *name).collect_vec();
    let workflows: IndexedWorkflows = workflows
        .iter()
        .map(|(_, items)| {
            items
                .iter()
                .map(|item| item.indexes_from(&names))
                .try_collect()
        })
        .try_collect()?;
    let start_idx = names
        .into_iter()
        .position(|s| s == "in")
        .context("No workflow named \"in\"")?;
    Ok(match part {
        Part1 => ratings
            .lines()
            .map(|line| {
                ensure!(line.starts_with('{') && line.ends_with('}'));
                let xmas = line[1..line.len() - 1]
                    .split(',')
                    .collect_tuple::<(_, _, _, _)>()
                    .context("Not 4")?;
                Ok([
                    xmas.0.strip_prefix("x=").context("Not x=")?.parse()?,
                    xmas.1.strip_prefix("m=").context("Not m=")?.parse()?,
                    xmas.2.strip_prefix("a=").context("Not a=")?.parse()?,
                    xmas.3.strip_prefix("s=").context("Not s=")?.parse()?,
                ])
            })
            .process_results(|it| {
                it.filter(|xmas: &[u32; 4]| workflows.run_process(start_idx, *xmas))
                    .map(|xmas| xmas[0] + xmas[1] + xmas[2] + xmas[3])
                    .map(u64::from)
                    .sum()
            })?,
        Part2 => {
            let valid_ranges = workflows.run_process(start_idx, [(1, 4000); 4]);
            // println!("{:?} ranges: {:?}", valid_ranges.len(), valid_ranges);
            valid_ranges
                .iter()
                .map(|part| {
                    part.iter()
                        .map(|(start, end)| end - start + 1)
                        .map(u64::from)
                        .product::<u64>()
                })
                .sum()
        }
    })
}

trait WorkflowProcess<T> {
    type Output;
    fn run_process(&self, start: usize, xmas: [T; 4]) -> Self::Output;
}

impl WorkflowProcess<u32> for IndexedWorkflows {
    type Output = bool;

    fn run_process(&self, start: usize, xmas: [u32; 4]) -> Self::Output {
        let mut idx = start;
        loop {
            for item in &self[idx] {
                let success = match item.cond {
                    None => true,
                    Some(Condition {
                        xmas_idx,
                        op,
                        value,
                    }) => op.eval(xmas[xmas_idx as usize], value),
                };
                if success {
                    match item.result {
                        Either::Left(i) => {
                            idx = i;
                            break;
                        }
                        Either::Right(accept) => return accept,
                    }
                }
            }
        }
    }
}

impl WorkflowProcess<(u32, u32)> for IndexedWorkflows {
    type Output = Vec<[(u32, u32); 4]>;

    fn run_process(&self, start: usize, xmas_ranges: [(u32, u32); 4]) -> Self::Output {
        let mut jobs = vec![(start, xmas_ranges)];
        let mut valid_ranges = vec![];
        while let Some((idx, mut ranges)) = jobs.pop() {
            for item in &self[idx] {
                let [success, failure] = match item.cond {
                    None => [Some(ranges), None],
                    Some(Condition {
                        xmas_idx,
                        op,
                        value,
                    }) => op
                        .split_range(ranges[xmas_idx as usize], value)
                        .map(|opt_range| {
                            opt_range.map(|range| {
                                let mut new = ranges;
                                new[xmas_idx as usize] = range;
                                new
                            })
                        }),
                };
                if let Some(part) = success {
                    match item.result {
                        Either::Left(idx) => jobs.push((idx, part)),
                        Either::Right(false) => {}
                        Either::Right(true) => valid_ranges.push(part),
                    }
                }
                match failure {
                    Some(part) => ranges = part,
                    None => break,
                }
            }
        }
        valid_ranges
    }
}

impl Operation {
    #[inline]
    const fn eval(self, n: u32, value: u32) -> bool {
        match self {
            Self::GreaterThan => n > value,
            Self::LowerThan => n < value,
        }
    }

    // [Good range?, Bad range?]
    const fn split_range(self, range: (u32, u32), value: u32) -> [Option<(u32, u32)>; 2] {
        match self {
            Self::GreaterThan => {
                if range.1 <= value {
                    [None, Some(range)]
                } else if value < range.0 {
                    [Some(range), None]
                } else {
                    [Some((value + 1, range.1)), Some((range.0, value))]
                }
            }
            Self::LowerThan => {
                if value <= range.0 {
                    [None, Some(range)]
                } else if range.1 < value {
                    [Some(range), None]
                } else {
                    [Some((range.0, value - 1)), Some((value, range.1))]
                }
            }
        }
    }
}

impl Item<&str> {
    fn indexes_from(&self, names: &[&str]) -> Result<Item<usize>> {
        let Self { cond, result } = self;
        Ok(Item {
            cond: cond.clone(),
            result: match result {
                Either::Left(name) => {
                    let idx = names.iter().position(|s| s == name);
                    Either::Left(idx.context("Name not found")?)
                }
                Either::Right(accept) => Either::Right(*accept),
            },
        })
    }
}

impl std::str::FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            ">" => Self::GreaterThan,
            "<" => Self::LowerThan,
            _ => bail!("Not < nor >"),
        })
    }
}

impl std::str::FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            xmas_idx: match &s[..1] {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => bail!("Not xmas indexed"),
            },
            op: s[1..2].parse()?,
            value: s[2..].parse()?,
        })
    }
}

impl<'a> TryFrom<&'a str> for Item<&'a str> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let (cond, res) = match s.split_once(':') {
            Some((condition, then)) => (Some(condition.parse()?), then),
            None => (None, s),
        };
        Ok(Self {
            cond,
            result: match res {
                "A" => Either::Right(true),
                "R" => Either::Right(false),
                _ => Either::Left(res),
            },
        })
    }
}

pub const INPUTS: [&str; 2] = [
    "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
",
    include_input!(23 19),
];

#[test]
fn solver_23_19() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 19114);
    assert_eq!(solver(Part1, INPUTS[1])?, 352052);
    assert_eq!(solver(Part2, INPUTS[0])?, 167409079868000);
    assert_eq!(solver(Part2, INPUTS[1])?, 116606738659695);
    Ok(())
}
