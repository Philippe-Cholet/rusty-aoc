use common::{bail, Context, Part, Part1, Part2, Result};
use utils::OkIterator;

use Outcome::{Draw, Lose, Win};
use Shape::{Paper, Rock, Scissors};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Shape {
    const fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    const fn fight_outcome(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Lose,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
        }
    }
}

impl Outcome {
    const fn score(&self) -> u32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }

    const fn shape_against(&self, enemy: &Shape) -> Shape {
        match (self, enemy) {
            (Lose, Paper) | (Draw, Rock) | (Win, Scissors) => Rock,
            (Lose, Scissors) | (Draw, Paper) | (Win, Rock) => Paper,
            (Lose, Rock) | (Draw, Scissors) | (Win, Paper) => Scissors,
        }
    }
}

/// Rock Paper Scissors
pub fn solver(part: Part, input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(|line| {
            let (abc, xyz) = line.split_once(' ').context("No space")?;
            let elf = match abc {
                "A" => Rock,
                "B" => Paper,
                "C" => Scissors,
                s => bail!("Not ABC: {}", s),
            };
            let me = match (part, xyz) {
                (Part1, "X") => Rock,
                (Part1, "Y") => Paper,
                (Part1, "Z") => Scissors,
                (Part2, "X") => Lose.shape_against(&elf),
                (Part2, "Y") => Draw.shape_against(&elf),
                (Part2, "Z") => Win.shape_against(&elf),
                (_, s) => bail!("Not XYZ: {}", s),
            };
            Ok(me.fight_outcome(&elf).score() + me.score())
        })
        .ok_sum::<u32>()?
        .to_string())
}

pub const INPUTS: [&str; 2] = ["A Y\nB X\nC Z\n", include_str!("input.txt")];

#[test]
fn solver_22_02() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "15");
    assert_eq!(solver(Part1, INPUTS[1])?, "11767");
    assert_eq!(solver(Part2, INPUTS[0])?, "12");
    assert_eq!(solver(Part2, INPUTS[1])?, "13886");
    Ok(())
}
