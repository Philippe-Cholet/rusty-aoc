use std::{fmt, iter::Sum, ops::Add, str::FromStr};

use itertools::Itertools;

use common::{Context, Error, Part, Part1, Part2, Result};
use utils::OkIterator;

// I avoided to derive Clone on purpose.
#[derive(Debug)]
enum Snailfish {
    Value(u8),
    Pair { left: Box<Self>, right: Box<Self> },
}

impl FromStr for Snailfish {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(if let Ok(n) = s.parse() {
            Self::Value(n)
        } else {
            let mut bracket_diff = 0;
            let (left, right) = s[1..s.len() - 1]
                .split_once(|ch| {
                    match ch {
                        ',' if bracket_diff == 0 => return true,
                        '[' => bracket_diff += 1,
                        ']' => bracket_diff -= 1,
                        _ => {}
                    }
                    false
                })
                .context("Could not find balance between opening and ending brackets")?;
            Self::new_pair(left.parse()?, right.parse()?)
        })
    }
}

impl fmt::Display for Snailfish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(n) => write!(f, "{n}"),
            Self::Pair { left, right } => write!(f, "[{left},{right}]"),
        }
    }
}

#[derive(Debug)]
struct Explosion {
    left: Option<u8>,
    right: Option<u8>,
}

impl Snailfish {
    fn new_pair(left: Self, right: Self) -> Self {
        Self::Pair {
            left: left.into(),
            right: right.into(),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Value(n) => u32::from(*n),
            Self::Pair { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn increase(&mut self, value: u8, on_left: bool) {
        match self {
            Self::Value(n) => *n += value,
            Self::Pair { left, .. } if on_left => left.increase(value, true),
            Self::Pair { right, .. } => right.increase(value, false),
        }
    }

    fn explode_utility(&mut self, level: u8) -> Option<Explosion> {
        let mut explosion: Option<Explosion> = None;
        let mut reset = false;
        if let Self::Pair { left, right } = self {
            // Found the pair to explode!
            if level >= 4 {
                if let Self::Value(left) = left.as_ref() {
                    if let Self::Value(right) = right.as_ref() {
                        explosion = Some(Explosion {
                            left: Some(*left),
                            right: Some(*right),
                        });
                        reset = true;
                    }
                }
            }
            if explosion.is_none() {
                // Continue exploration.
                if let Some(mut exp) = left.explode_utility(level + 1) {
                    if let Some(r) = exp.right.take() {
                        right.increase(r, true);
                    }
                    explosion = Some(exp);
                } else if let Some(mut exp) = right.explode_utility(level + 1) {
                    if let Some(l) = exp.left.take() {
                        left.increase(l, false);
                    }
                    explosion = Some(exp);
                }
            }
        }
        if reset {
            *self = Self::default();
        }
        explosion
    }

    fn explode(&mut self) -> bool {
        self.explode_utility(0).is_some()
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Value(n @ 10..) => {
                *self = Self::new_pair(Self::Value(*n / 2), Self::Value(*n - *n / 2));
                true
            }
            Self::Value(_) => false,
            Self::Pair { left, right } => left.split() || right.split(),
        }
    }

    fn reduction(&mut self) {
        while self.explode() || self.split() {}
    }
}

impl Default for Snailfish {
    fn default() -> Self {
        Self::Value(0)
    }
}

impl Add for Snailfish {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut res = Self::new_pair(self, other);
        res.reduction();
        res
    }
}

impl Sum<Self> for Snailfish {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(Self::add).unwrap_or_default()
    }
}

impl PartialEq for Snailfish {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(v1), Self::Value(v2)) => v1 == v2,
            (
                Self::Pair {
                    left: l1,
                    right: r1,
                },
                Self::Pair {
                    left: l2,
                    right: r2,
                },
            ) => *l1 == *l2 && *r1 == *r2,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests_18 {
    use super::*;

    #[test]
    fn test_snailfish_explode() -> Result<()> {
        for (s, a) in [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ] {
            let mut f: Snailfish = s.parse()?;
            f.explode();
            assert_eq!(f, a.parse()?);
        }
        Ok(())
    }

    #[test]
    fn test_snailfish_reduction() -> Result<()> {
        let s = [
            "[[[[4,3],4],4],[7,[[8,4],9]]]",
            "[1,1]",
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        ];
        let mut f = Snailfish::new_pair(s[0].parse()?, s[1].parse()?);
        f.explode();
        assert_eq!(f, s[2].parse()?);
        f.explode();
        assert_eq!(f, s[3].parse()?);
        f.split();
        assert_eq!(f, s[4].parse()?);
        f.split();
        assert_eq!(f, s[5].parse()?);
        f.explode();
        assert_eq!(f, s[6].parse()?);
        assert_eq!(s[0].parse::<Snailfish>()? + s[1].parse()?, s[6].parse()?);
        Ok(())
    }

    #[test]
    fn test_snailfish_sum() -> Result<()> {
        for (fs, answer) in [
            (
                vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            ),
            (
                vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            ),
            (
                vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            ),
            (
                vec![
                    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                    "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                    "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                    "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                    "[7,[5,[[3,8],[1,4]]]]",
                    "[[2,[2,2]],[8,[8,1]]]",
                    "[2,9]",
                    "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                    "[[[5,[7,4]],7],1]",
                    "[[[[4,2],2],6],[8,7]]",
                ],
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
        ] {
            assert_eq!(
                fs.into_iter().map(str::parse).ok_sum::<Snailfish>()?,
                answer.parse()?
            );
        }
        Ok(())
    }

    #[test]
    fn test_snailfish_magnitude() -> Result<()> {
        for (s, n) in [
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ] {
            assert_eq!(s.parse::<Snailfish>()?.magnitude(), n);
        }
        Ok(())
    }

    #[test]
    fn test_snailfish_sum2() -> Result<()> {
        let fs = [
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];
        assert_eq!(
            fs.into_iter().map(str::parse).ok_sum::<Snailfish>()?,
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".parse()?,
        );
        Ok(())
    }
}

/// Snailfish
pub fn solver(part: Part, input: &str) -> Result<String> {
    let result = match part {
        Part1 => input
            .lines()
            .map(str::parse)
            .ok_sum::<Snailfish>()?
            .magnitude(),
        Part2 => input
            .lines()
            .tuple_combinations()
            .flat_map(|(a, b)| [(a, b), (b, a)])
            .map(|(a, b)| common::Ok((a.parse::<Snailfish>()? + b.parse()?).magnitude()))
            .ok_max()?
            .context("empty input")?,
    };
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_18() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "4140");
    assert_eq!(solver(Part1, INPUTS[1])?, "4347");
    assert_eq!(solver(Part2, INPUTS[0])?, "3993");
    assert_eq!(solver(Part2, INPUTS[1])?, "4721");
    Ok(())
}
