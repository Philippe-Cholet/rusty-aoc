use common::prelude::*;
use utils::OkIterator;

#[derive(Debug, Clone)]
enum Value<'a> {
    Int(u16),
    Variable(&'a str),
}

#[derive(Debug, Clone)]
enum Operation<'a> {
    Value(Value<'a>),
    Not(Value<'a>),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    Lshift(Value<'a>, Value<'a>),
    Rshift(Value<'a>, Value<'a>),
}

/// Some Assembly Required
pub fn solver(part: Part, input: &str) -> Result<u16> {
    let mut data = input
        .lines()
        .map(|line| {
            let (s, dst) = line.split_once(" -> ").context("No arrow")?;
            s.try_into().map(|op: Operation| (op, dst))
        })
        .ok_collect_vec()?;
    let mut a = Operation::find_a(&data)?;
    if part.two() {
        let b_op = data
            .iter_mut()
            .find_map(|(op, dst)| (dst == &"b").then_some(op))
            .context("No value \"b\"")?;
        *b_op = Operation::Value(Value::Int(a));
        a = Operation::find_a(&data)?;
    }
    Ok(a)
}

impl<'a> Value<'a> {
    fn get(&self, values: &HashMap<&'a str, u16>) -> Option<u16> {
        match self {
            Self::Int(n) => Some(*n),
            Self::Variable(s) => values.get(s).copied(),
        }
    }
}

impl<'a> Operation<'a> {
    fn eval(&self, values: &mut HashMap<&'a str, u16>, dst: &'a str) {
        if let Some(res) = match self {
            Self::Value(v) => v.get(values),
            Self::Not(v) => v.get(values).map(|n| u16::MAX ^ n),
            Self::And(u, v) => u.get(values).zip(v.get(values)).map(|(u, v)| u & v),
            Self::Or(u, v) => u.get(values).zip(v.get(values)).map(|(u, v)| u | v),
            Self::Lshift(u, v) => u.get(values).zip(v.get(values)).map(|(u, v)| u << v),
            Self::Rshift(u, v) => u.get(values).zip(v.get(values)).map(|(u, v)| u >> v),
        } {
            values.insert(dst, res);
        }
    }

    fn find_a(data: &[(Self, &str)]) -> Result<u16> {
        let mut values = HashMap::with_capacity(data.len());
        while !values.contains_key("a") {
            for (op, dst) in data {
                op.eval(&mut values, dst);
            }
        }
        values.remove("a").context("No value \"a\"")
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        value.parse().map_or(Self::Variable(value), Self::Int)
    }
}

impl<'a> TryFrom<&'a str> for Operation<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        Ok(if let Some(s) = value.strip_prefix("NOT ") {
            Self::Not(s.into())
        } else if value.contains(' ') {
            let values: Vec<_> = value.split_whitespace().collect();
            ensure!(values.len() == 3, "Not 2 spaces");
            let (u, op, v) = (values[0].into(), values[1], values[2].into());
            match op {
                "AND" => Self::And(u, v),
                "OR" => Self::Or(u, v),
                "LSHIFT" => Self::Lshift(u, v),
                "RSHIFT" => Self::Rshift(u, v),
                _ => bail!("Not AND/OR/LSHIFT/RSHIFT but {:?}", op),
            }
        } else {
            Self::Value(value.into())
        })
    }
}

pub const INPUTS: [&str; 1] = [include_input!(15 07)];

#[test]
fn solver_15_07() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 46065);
    assert_eq!(solver(Part2, INPUTS[0])?, 14134);
    Ok(())
}
