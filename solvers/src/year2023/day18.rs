use itertools::Itertools;

use common::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

/// Lavaduct Lagoon
pub fn solver(part: Part, input: &str) -> Result<u64> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let udlr = parts.next().context("Empty line")?;
            let n = parts.next().context("No number in the line")?;
            let last = parts.next().context("No (#......) in the line")?;
            ensure!(parts.next().is_none(), "Too many spaces in the line");
            Ok(match part {
                Part1 => {
                    let dir = match udlr {
                        "R" => Direction::E,
                        "D" => Direction::S,
                        "L" => Direction::W,
                        "U" => Direction::N,
                        _ => bail!("Not R D L U but: {}", udlr),
                    };
                    (dir, n.parse()?)
                }
                Part2 => {
                    let last = last
                        .strip_prefix("(#")
                        .context("Wrong last prefix")?
                        .strip_suffix(')')
                        .context("Wrong last suffix")?;
                    ensure!(last.len() == 6, "Not 6-long");
                    let (n, udlr) = last.split_at(5);
                    let dir = match udlr {
                        "0" => Direction::E,
                        "1" => Direction::S,
                        "2" => Direction::W,
                        "3" => Direction::N,
                        _ => bail!("Wrong 0-3 direction: {}", udlr),
                    };
                    (dir, i64::from_str_radix(n, 16)?)
                }
            })
        })
        .process_results(|it| {
            let mut loc = (0, 0);
            let mut double_area = 0;
            let mut perimeter = 0;
            // Ideally it would be `it.map(...).circular_tuple_windows().for_each(...)`
            std::iter::once(loc)
                .chain(it.map(|(dir, n)| {
                    match dir {
                        Direction::N => loc.0 -= n,
                        Direction::S => loc.0 += n,
                        Direction::W => loc.1 -= n,
                        Direction::E => loc.1 += n,
                    };
                    loc
                }))
                .tuple_windows()
                .for_each(|((a, b), (c, d))| {
                    double_area += a * d - b * c;
                    perimeter += a.abs_diff(c).max(b.abs_diff(d));
                });
            (loc == (0, 0))
                .then(|| double_area.abs_diff(0) / 2 + perimeter / 2 + 1)
                .context("The polygon does not end where it started!")
        })?
}

pub const INPUTS: [&str; 2] = [
    "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
",
    include_input!(23 18),
];

#[test]
fn solver_23_18() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 62);
    assert_eq!(solver(Part1, INPUTS[1])?, 50465);
    assert_eq!(solver(Part2, INPUTS[0])?, 952408144115);
    assert_eq!(solver(Part2, INPUTS[1])?, 82712746433310);
    Ok(())
}
