use common::{prelude::*, Ok};
use crate::utils::OkIterator;

/// Dive!
pub fn solver(part: Part, input: &str) -> Result<i32> {
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    let commands = input
        .lines()
        .map(|line| {
            let (s, n) = line.split_once(' ').context("no whitespace")?;
            Ok((s, n.parse::<i32>()?))
        })
        .ok_collect_vec()?;
    for command in commands {
        match (part, command) {
            (Part1, ("forward", n)) => position += n,
            (Part1, ("down", n)) => depth += n,
            (Part1, ("up", n)) => depth -= n,
            (Part2, ("down", n)) => aim += n,
            (Part2, ("up", n)) => aim -= n,
            (Part2, ("forward", n)) => {
                position += n;
                depth += n * aim;
            }
            _ => bail!("Invalid submarine command"),
        }
    }
    Ok(position * depth)
}

test_solver! {
    "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
" => (150, 900),
    include_input!(21 02) => (1507611, 1880593125),
}
