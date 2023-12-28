use common::prelude::*;

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
pub fn solver(part: Part, input: &str) -> Result<u32> {
    input
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
        .sum()
}

test_solver! {
    "A Y\nB X\nC Z\n" => (15, 12),
    include_input!(22 02) => (11767, 13886),
}
