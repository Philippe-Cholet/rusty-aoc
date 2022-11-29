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
        // Year2022 => aoc22(day)?,
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

fn aoc21(day: Day) -> (AocSolver, Vec<&'static str>) {
    match day {
        Day1 => (aoc21_01::solver, aoc21_01::INPUTS.to_vec()),
        Day2 => (aoc21_02::solver, aoc21_02::INPUTS.to_vec()),
        Day3 => (aoc21_03::solver, aoc21_03::INPUTS.to_vec()),
        Day4 => (aoc21_04::solver, aoc21_04::INPUTS.to_vec()),
        Day5 => (aoc21_05::solver, aoc21_05::INPUTS.to_vec()),
        Day6 => (aoc21_06::solver, aoc21_06::INPUTS.to_vec()),
        Day7 => (aoc21_07::solver, aoc21_07::INPUTS.to_vec()),
        Day8 => (aoc21_08::solver, aoc21_08::INPUTS.to_vec()),
        Day9 => (aoc21_09::solver, aoc21_09::INPUTS.to_vec()),
        Day10 => (aoc21_10::solver, aoc21_10::INPUTS.to_vec()),
        Day11 => (aoc21_11::solver, aoc21_11::INPUTS.to_vec()),
        Day12 => (aoc21_12::solver, aoc21_12::INPUTS.to_vec()),
        Day13 => (aoc21_13::solver, aoc21_13::INPUTS.to_vec()),
        Day14 => (aoc21_14::solver, aoc21_14::INPUTS.to_vec()),
        Day15 => (aoc21_15::solver, aoc21_15::INPUTS.to_vec()),
        Day16 => (aoc21_16::solver, aoc21_16::INPUTS.to_vec()),
        Day17 => (aoc21_17::solver, aoc21_17::INPUTS.to_vec()),
        Day18 => (aoc21_18::solver, aoc21_18::INPUTS.to_vec()),
        Day19 => (aoc21_19::solver, aoc21_19::INPUTS.to_vec()),
        Day20 => (aoc21_20::solver, aoc21_20::INPUTS.to_vec()),
        Day21 => (aoc21_21::solver, aoc21_21::INPUTS.to_vec()),
        Day22 => (aoc21_22::solver, aoc21_22::INPUTS.to_vec()),
        Day23 => (aoc21_23::solver, aoc21_23::INPUTS.to_vec()),
        Day24 => (aoc21_24::solver, aoc21_24::INPUTS.to_vec()),
        Day25 => (aoc21_25::solver, aoc21_25::INPUTS.to_vec()),
    }
}
