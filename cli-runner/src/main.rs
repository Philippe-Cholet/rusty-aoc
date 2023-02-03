use std::{
    env,
    time::{Duration, Instant},
};

use common::{ensure, AocSolver, Context, Day, Part, Part1, Part2, Result, Year};

aoc_macro::make_aoc!();

fn timed_solve(solver: AocSolver, part: Part, input: &str) -> Result<(String, Duration)> {
    let now = Instant::now();
    let result = solver(part, input)?;
    let t = now.elapsed();
    Ok((result, t))
}

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    // Get the solver for a given puzzle (year, day).
    let year = args
        .next()
        .context("Missing argument: <year [20]15...>")?
        .parse()?;
    let day = match args.next() {
        Some(s) => s.parse()?,
        None => return run_big_inputs(year),
    };
    let (solver, inputs) = aoc(year, day)?;
    // Given part or both.
    let parts = match args.next() {
        Some(s) => vec![s.parse()?],
        None => Part::ALL.to_vec(),
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
            let (result, t) = timed_solve(solver, part, input)?;
            println!("[ Done in {t:?} ]\n{result}");
        }
    }
    Ok(())
}

fn run_big_inputs(year: Year) -> Result<()> {
    let mut results = vec![];
    for day in Day::ALL {
        if let Ok((solver, [.., big_input])) = aoc(year, day) {
            let t1 = timed_solve(solver, Part1, big_input)?.1;
            let t2 = timed_solve(solver, Part2, big_input)?.1;
            results.push((day, t1, t2));
        }
    }
    results.sort_by_key(|(_, t0, t1)| *t0 + *t1);
    println!("========== {year:?} ==========");
    for (day, t0, t1) in &results {
        println!("{day:?}: {t0:?} + {t1:?}");
    }
    let t1s: Duration = results.iter().map(|(_, t1, _)| t1).sum();
    let t2s: Duration = results.iter().map(|(_, _, t2)| t2).sum();
    println!(
        "{} days: {t1s:?} + {t2s:?} == {:?}",
        results.len(),
        t1s + t2s
    );
    Ok(())
}
