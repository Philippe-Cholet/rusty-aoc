use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pt(i32, i32);

impl std::ops::AddAssign for Pt {
    fn add_assign(&mut self, pt: Self) {
        self.0 += pt.0;
        self.1 += pt.1;
    }
}

// ...too far ; -2 -> -1 ; -1 -> -1 ; 0 -> 0 ; 1 -> 1 ; 2 -> 1 ; too far...
fn shorten(n: i32) -> Result<i32> {
    Ok(match n.abs() {
        0 | 1 => n,
        2 => n - n.signum(),
        3..=i32::MAX => bail!("impossibly far: {}", n.abs()),
        i32::MIN..=-1 => unreachable!("negative abs"),
    })
}

impl Pt {
    fn follow_knot(&mut self, knot: Self) -> Result<()> {
        let diff = Self(knot.0 - self.0, knot.1 - self.1);
        let short_diff = Self(shorten(diff.0)?, shorten(diff.1)?);
        if short_diff != diff {
            *self += short_diff;
        }
        Ok(())
    }
}

fn get_tail_positions<const NB_KNOTS: usize>(moves: &[(Pt, usize)]) -> Result<Vec<Pt>> {
    let mut rope = [Pt(0, 0); NB_KNOTS];
    let mut tail_pos = vec![Pt(0, 0)];
    for (head_move, nb) in moves {
        for _ in 0..*nb {
            rope[0] += *head_move;
            for i in 1..NB_KNOTS {
                rope[i].follow_knot(rope[i - 1])?;
            }
            tail_pos.push(rope[NB_KNOTS - 1]);
        }
    }
    Ok(tail_pos)
}

/// Rope Bridge
pub fn solver(part: Part, input: &str) -> Result<String> {
    let moves = input
        .lines()
        .map(|line| {
            let (s, nb) = line.split_once(' ').context("No whitespace")?;
            let head_move = match s {
                "D" => Pt(1, 0),
                "U" => Pt(-1, 0),
                "L" => Pt(0, -1),
                "R" => Pt(0, 1),
                _ => bail!("Wrong move: {}", s),
            };
            Ok((head_move, nb.parse()?))
        })
        .ok_collect_vec()?;
    let tail_pos = match part {
        Part1 => get_tail_positions::<2>(&moves)?,
        Part2 => get_tail_positions::<10>(&moves)?,
    };
    let result = tail_pos.into_iter().unique().count();
    Ok(result.to_string())
}

pub const INPUTS: [&str; 3] = [
    "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
",
    "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_09() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "13");
    assert_eq!(solver(Part1, INPUTS[2])?, "6037");
    assert_eq!(solver(Part2, INPUTS[0])?, "1");
    assert_eq!(solver(Part2, INPUTS[1])?, "36");
    assert_eq!(solver(Part2, INPUTS[2])?, "2485");
    Ok(())
}
