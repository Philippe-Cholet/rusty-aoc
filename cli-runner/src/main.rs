use std::{env, time::Instant};

#[allow(clippy::wildcard_imports)]
use common::*;

aoc_macro::make_aoc!("cli-runner");

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    // Get the solver for a given puzzle (year, day).
    let year = args
        .next()
        .context("Missing argument: <year [20]15...>")?
        .parse()?;
    let day = args
        .next()
        .context("Missing argument: <day 1...25>")?
        .parse()?;
    let (solver, inputs) = aoc(year, day)?;
    // Given part or both.
    let parts = match args.next() {
        Some(s) => vec![s.parse()?],
        None => vec![Part1, Part2],
    };
    // Given index or all.
    let input_indexes = match args.next() {
        Some(s) => {
            let idx: usize = s.parse()?;
            ensure!(
                idx < inputs.len(),
                "No input #{}, there are only {} inputs!",
                idx,
                inputs.len()
            );
            vec![idx]
        }
        None => (0..inputs.len()).collect(),
    };
    // Run the solver on selected parts and inputs.
    println!("Advent of Code {year:?} {day:?}...");
    for part in parts {
        for input_idx in &input_indexes {
            println!("\n{part:?} input #{input_idx}:");
            let input = inputs[*input_idx];
            ensure!(!input.is_empty(), "Empty input: you forgot to fill it?!");
            let now = Instant::now();
            let result = solver(part, input)?;
            let t = now.elapsed();
            println!("[ Done in {t:?} ]\n{result}");
        }
    }
    Ok(())
}
