use std::{env, time::Instant};

#[allow(clippy::wildcard_imports)]
use common::*;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let year = args
        .next()
        .context("Missing argument: <year 15...>")?
        .parse()?;
    let day = args
        .next()
        .context("Missing argument: <day 1...25>")?
        .parse()?;
    let part = args
        .next()
        .context("Missing argument: <part 1 2>")?
        .parse()?;
    let (input_idx, input_nb): (Option<usize>, _) = match args.next() {
        None => (None, String::new()),
        Some(s) => (Some(s.parse()?), format!(" input #{s}")),
    };

    println!("Advent of Code {year:?} {day:?} {part:?}{input_nb}...");
    let (solver, inputs) = match year {
        // Year2015 => aoc15(day)?,
        // Year2016 => aoc16(day)?,
        // Year2017 => aoc17(day)?,
        // Year2018 => aoc18(day)?,
        // Year2019 => aoc19(day)?,
        // Year2020 => aoc20(day)?,
        Year2021 => aoc21(day),
        Year2022 => aoc22(day)?,
        _ => bail!(
            "You have not solved the advent of code of \
            this year ({:?}), what are you waiting for?!",
            year,
        ),
    };
    let input = input_idx
        .map_or_else(|| inputs.last(), |idx| inputs.get(idx))
        .ok_or_else(|| {
            input_idx.map_or_else(
                || format_err!("No input!"),
                |idx| format_err!("No input #{}, there are only {} inputs!", idx, inputs.len()),
            )
        })?;

    let now = Instant::now();
    let result = solver(part, input)?;
    let t = now.elapsed();
    println!("My result [ {t:?} ]\n{result}");

    Ok(())
}

fn aoc21(day: Day) -> (AocSolver, &'static [&'static str]) {
    match day {
        Day1 => (aoc21_01::solver, &aoc21_01::INPUTS),
        Day2 => (aoc21_02::solver, &aoc21_02::INPUTS),
        Day3 => (aoc21_03::solver, &aoc21_03::INPUTS),
        Day4 => (aoc21_04::solver, &aoc21_04::INPUTS),
        Day5 => (aoc21_05::solver, &aoc21_05::INPUTS),
        Day6 => (aoc21_06::solver, &aoc21_06::INPUTS),
        Day7 => (aoc21_07::solver, &aoc21_07::INPUTS),
        Day8 => (aoc21_08::solver, &aoc21_08::INPUTS),
        Day9 => (aoc21_09::solver, &aoc21_09::INPUTS),
        Day10 => (aoc21_10::solver, &aoc21_10::INPUTS),
        Day11 => (aoc21_11::solver, &aoc21_11::INPUTS),
        Day12 => (aoc21_12::solver, &aoc21_12::INPUTS),
        Day13 => (aoc21_13::solver, &aoc21_13::INPUTS),
        Day14 => (aoc21_14::solver, &aoc21_14::INPUTS),
        Day15 => (aoc21_15::solver, &aoc21_15::INPUTS),
        Day16 => (aoc21_16::solver, &aoc21_16::INPUTS),
        Day17 => (aoc21_17::solver, &aoc21_17::INPUTS),
        Day18 => (aoc21_18::solver, &aoc21_18::INPUTS),
        Day19 => (aoc21_19::solver, &aoc21_19::INPUTS),
        Day20 => (aoc21_20::solver, &aoc21_20::INPUTS),
        Day21 => (aoc21_21::solver, &aoc21_21::INPUTS),
        Day22 => (aoc21_22::solver, &aoc21_22::INPUTS),
        Day23 => (aoc21_23::solver, &aoc21_23::INPUTS),
        Day24 => (aoc21_24::solver, &aoc21_24::INPUTS),
        Day25 => (aoc21_25::solver, &aoc21_25::INPUTS),
    }
}

// TODO: Once all days are implemented, remove Result...
fn aoc22(day: Day) -> Result<(AocSolver, &'static [&'static str])> {
    Ok(match day {
        Day1 => (aoc22_01::solver, &aoc22_01::INPUTS),
        Day2 => (aoc22_02::solver, &aoc22_02::INPUTS),
        // Day3 => (aoc22_03::solver, &aoc22_03::INPUTS),
        // Day4 => (aoc22_04::solver, &aoc22_04::INPUTS),
        // Day5 => (aoc22_05::solver, &aoc22_05::INPUTS),
        // Day6 => (aoc22_06::solver, &aoc22_06::INPUTS),
        // Day7 => (aoc22_07::solver, &aoc22_07::INPUTS),
        // Day8 => (aoc22_08::solver, &aoc22_08::INPUTS),
        // Day9 => (aoc22_09::solver, &aoc22_09::INPUTS),
        // Day10 => (aoc22_10::solver, &aoc22_10::INPUTS),
        // Day11 => (aoc22_11::solver, &aoc22_11::INPUTS),
        // Day12 => (aoc22_12::solver, &aoc22_12::INPUTS),
        // Day13 => (aoc22_13::solver, &aoc22_13::INPUTS),
        // Day14 => (aoc22_14::solver, &aoc22_14::INPUTS),
        // Day15 => (aoc22_15::solver, &aoc22_15::INPUTS),
        // Day16 => (aoc22_16::solver, &aoc22_16::INPUTS),
        // Day17 => (aoc22_17::solver, &aoc22_17::INPUTS),
        // Day18 => (aoc22_18::solver, &aoc22_18::INPUTS),
        // Day19 => (aoc22_19::solver, &aoc22_19::INPUTS),
        // Day20 => (aoc22_20::solver, &aoc22_20::INPUTS),
        // Day21 => (aoc22_21::solver, &aoc22_21::INPUTS),
        // Day22 => (aoc22_22::solver, &aoc22_22::INPUTS),
        // Day23 => (aoc22_23::solver, &aoc22_23::INPUTS),
        // Day24 => (aoc22_24::solver, &aoc22_24::INPUTS),
        // Day25 => (aoc22_25::solver, &aoc22_25::INPUTS),
        _ => bail!("AoC 2022 {:?}... What are you waiting for?!", day),
    })
}
