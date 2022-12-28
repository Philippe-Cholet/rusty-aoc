use std::{cmp::Ordering, str::FromStr};

use itertools::{EitherOrBoth, Itertools};

use common::{ensure, Context, Error, Part, Part1, Part2, Result};
use utils::OkIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    List(Vec<Self>),
    Int(u8),
}

impl FromStr for Packet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(if s.starts_with('[') {
            ensure!(s.ends_with(']'), "No closing bracket");
            let t = &s[1..s.len() - 1];
            Self::List(if t.is_empty() {
                vec![]
            } else {
                let mut bracket_diff = 0;
                t.split(|ch| {
                    match ch {
                        ',' if bracket_diff == 0 => return true,
                        '[' => bracket_diff += 1,
                        ']' => bracket_diff -= 1,
                        _ => {}
                    }
                    false
                })
                .map(str::parse)
                .ok_collect_vec()?
            })
        } else {
            Self::Int(s.parse()?)
        })
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Int(left), Self::Int(right)) => left.cmp(right),
            (Self::List(_), Self::Int(right)) => self.cmp(&Self::List(vec![Self::Int(*right)])),
            (Self::Int(left), Self::List(_)) => Self::List(vec![Self::Int(*left)]).cmp(other),
            (Self::List(left), Self::List(right)) => left
                .iter()
                .zip_longest(right.iter())
                .find_map(|pair| match pair {
                    EitherOrBoth::Left(_) => Some(Ordering::Greater),
                    EitherOrBoth::Right(_) => Some(Ordering::Less),
                    EitherOrBoth::Both(left, right) => {
                        let cmp = left.cmp(right);
                        cmp.is_ne().then_some(cmp)
                    }
                })
                .unwrap_or(Ordering::Equal),
        }
    }
}

/// Distress Signal
pub fn solver(part: Part, input: &str) -> Result<String> {
    Ok(match part {
        Part1 => input
            .split("\n\n")
            .map(|s| {
                let (left, right) = s.lines().collect_tuple().context("Not a pair")?;
                Ok((left.parse::<Packet>()?, right.parse::<Packet>()?))
            })
            .ok_collect_vec()?
            .into_iter()
            .enumerate()
            .filter_map(|(idx, (left, right))| (left < right).then_some(idx + 1))
            .sum(),
        Part2 => {
            let mut lines = input
                .lines()
                .filter(|line| !line.is_empty())
                .map(str::parse)
                .ok_collect_vec()?;
            let packet2: Packet = "[[2]]".parse()?;
            let packet6: Packet = "[[6]]".parse()?;
            lines.push(packet2.clone());
            lines.push(packet6.clone());
            lines.sort();
            (1 + lines
                .iter()
                .position(|p| p == &packet2)
                .context("Did not find packet2")?)
                * (1 + lines
                    .iter()
                    .position(|p| p == &packet6)
                    .context("Did not find packet6")?)
        }
    }
    .to_string())
}

pub const INPUTS: [&str; 2] = [
    "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_13() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "13");
    assert_eq!(solver(Part1, INPUTS[1])?, "5555");
    assert_eq!(solver(Part2, INPUTS[0])?, "140");
    assert_eq!(solver(Part2, INPUTS[1])?, "22852");
    Ok(())
}
