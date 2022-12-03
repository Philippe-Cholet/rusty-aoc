use std::{env, time::Instant};

#[allow(clippy::wildcard_imports)]
use common::*;

aoc_macro::make_aoc!(21 25, 22 3);

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
    let (solver, inputs) = aoc(year, day)?;
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
