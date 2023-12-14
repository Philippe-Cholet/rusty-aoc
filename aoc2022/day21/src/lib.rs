use petgraph::{algo::toposort, graph::NodeIndex, Directed, Graph};

use common::prelude::*;
use utils::OkIterator;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl std::str::FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => bail!("Wrong operation: {}", s),
        })
    }
}

impl Operation {
    const fn evaluate(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Sub => left - right,
            Self::Mul => left * right,
            Self::Div => left / right,
        }
    }

    /// `+ <--> -` and `* <--> /`
    const fn inv(&self) -> Self {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Mul => Self::Div,
            Self::Div => Self::Mul,
        }
    }

    /// `... operator v == value`
    const fn solve_left(&self, v: i64, value: i64) -> i64 {
        self.inv().evaluate(value, v)
    }

    /// `v operator ... == value`
    const fn solve_right(&self, v: i64, value: i64) -> i64 {
        match self {
            Self::Add => value - v,
            Self::Sub => v - value,
            Self::Mul => value / v,
            Self::Div => v / value,
        }
    }
}

#[derive(Debug)]
enum Job<'a> {
    Unknown,
    Number(i64),
    Operation {
        left: &'a str,
        op: Operation,
        right: &'a str,
    },
}

impl<'a> Job<'a> {
    fn evaluate(&self, monkey_values: &HashMap<&str, Option<i64>>) -> Option<i64> {
        match self {
            Self::Unknown => None,
            Self::Number(n) => Some(*n),
            Self::Operation { left, op, right } => {
                Some(op.evaluate(monkey_values[left]?, monkey_values[right]?))
            }
        }
    }

    fn solve(
        &'a self,
        monkey_values: &mut HashMap<&'a str, Option<i64>>,
        value: i64,
    ) -> Result<()> {
        let (name, new_value) = match self {
            Self::Unknown => (&"humn", value),
            Self::Number(n) => {
                ensure!(n == &value, "{} != {}", n, value);
                return Ok(());
            }
            Self::Operation { left, op, right } => {
                match (&monkey_values[left], op, &monkey_values[right]) {
                    (Some(_), _, Some(_)) => return Ok(()),
                    (None, op, Some(v)) => (left, op.solve_left(*v, value)),
                    (Some(v), op, None) => (right, op.solve_right(*v, value)),
                    (None, _, None) => bail!("Can not solve with two unknown variables"),
                }
            }
        };
        *monkey_values.get_mut(name).context("Missing name")? = Some(new_value);
        Ok(())
    }
}

fn get_order(data: &[(&str, Job)]) -> Result<Vec<usize>> {
    let name2idx: HashMap<_, _> = data
        .iter()
        .enumerate()
        .map(|(idx, (key, _))| (*key, idx))
        .collect();
    let edges = data
        .iter()
        .flat_map(|(key, job)| match job {
            Job::Unknown | Job::Number(_) => vec![],
            Job::Operation { left, right, .. } => {
                vec![
                    (name2idx[left], name2idx[key]),
                    (name2idx[right], name2idx[key]),
                ]
            }
        })
        // humn first.
        .chain(data.iter().filter_map(|(name, _)| {
            (name != &"humn").then_some((name2idx["humn"], name2idx[name]))
        }));
    let g: Graph<(), (), Directed, usize> = Graph::from_edges(edges);
    Ok(toposort(&g, None)
        .map_err(|_| format_err!("cycle of monkeys detected"))?
        .into_iter()
        .map(NodeIndex::index)
        .collect())
}

/// Monkey Math
pub fn solver(part: Part, input: &str) -> Result<i64> {
    let data = input
        .lines()
        .map(|line| {
            let mut vs: Vec<_> = line.split_whitespace().collect();
            vs[0] = vs[0].split_once(':').context("No :")?.0;
            Ok(match (part, vs.as_slice()) {
                (Part2, &["humn", _]) => ("humn", Job::Unknown),
                (_, &[name, n]) => (name, Job::Number(n.parse()?)),
                (_, &[name, left, op, right]) => (
                    name,
                    Job::Operation {
                        left,
                        op: op.parse()?,
                        right,
                    },
                ),
                _ => bail!("Wrong line: {:?}", vs),
            })
        })
        .ok_collect_vec()?;
    let order = get_order(&data)?;
    // println!(
    //     "Ordered names: {:?}",
    //     order.iter().map(|idx| data[*idx].0).collect::<Vec<_>>()
    // );
    ensure!(
        data[*order.first().context("No data")?].0 == "humn",
        "Human should be first",
    );
    ensure!(
        data[*order.last().context("No data")?].0 == "root",
        "Root should be last",
    );

    let mut monkey_values: HashMap<&str, Option<i64>> = HashMap::new();
    for idx in &order {
        let (name, job) = &data[*idx];
        let v = if name != &"root" {
            job.evaluate(&monkey_values)
        } else if part.one() {
            return job.evaluate(&monkey_values).context("unevaluable root");
        } else {
            None
        };
        monkey_values.insert(name, v);
    }
    ensure!(part.two(), "Root being last, part 1 should be solved!");
    for idx in order.iter().rev() {
        let (name, job) = &data[*idx];
        if name == &"root" {
            let Job::Operation { left, right, .. } = job else {
                bail!("Root is a number");
            };
            match (monkey_values[left], monkey_values[right]) {
                (None, Some(v)) => *monkey_values.get_mut(left).context("Missing name")? = Some(v),
                (Some(v), None) => *monkey_values.get_mut(right).context("Missing name")? = Some(v),
                _ => bail!(""),
            }
        } else {
            let Some(value) = monkey_values[name] else {
                bail!("{} yells no value", name);
            };
            job.solve(&mut monkey_values, value)?;
        }
    }
    #[cfg(debug_assertions)]
    println!("{:?}", monkey_values["humn"]);
    monkey_values["humn"].context("Failed to solve")
}

pub const INPUTS: [&str; 2] = [
    "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
",
    include_input!(22 21),
];

#[test]
fn solver_22_21() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 152);
    assert_eq!(solver(Part1, INPUTS[1])?, 256997859093114);
    assert_eq!(solver(Part2, INPUTS[0])?, 301);
    assert_eq!(solver(Part2, INPUTS[1])?, 3952288690726);
    Ok(())
}
