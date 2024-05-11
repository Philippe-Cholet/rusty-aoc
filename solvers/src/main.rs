use std::env;
use std::time::Duration;

use common::{ensure, Day, Part, Part1, Part2, Result, Year};
use solvers::aoc;

fn run_some_inputs(year: Year, day: Day, parts: &[Part], index: Option<usize>) -> Result<()> {
    let (solver, inputs) = aoc(year, day)?;
    // Given index or all.
    let input_indexes = match index {
        Some(idx) => {
            let nb = inputs.len();
            ensure!(idx < nb, "No input #{idx}, there are only {nb} inputs!");
            vec![idx]
        }
        None => (0..inputs.len()).collect(),
    };
    // Run the solver on selected parts and inputs.
    println!("Advent of Code {year:?} {day:?}...");
    for &part in parts {
        for input_idx in &input_indexes {
            println!("\n{part:?} input #{input_idx}:");
            let input = inputs[*input_idx];
            ensure!(!input.is_empty(), "Empty input: you forgot to fill it?!");
            #[cfg(not(feature = "trace_alloc"))]
            let (result, t) = solver.timed_solve(part, input)?;
            #[cfg(feature = "trace_alloc")]
            let (result, t) = solver.alloc_solve(part, input)?;
            println!("[ Done in {t:?} ]\n{result}");
        }
    }
    Ok(())
}

fn run_big_inputs(year: Year) -> Result<()> {
    let mut results = Vec::with_capacity(50);
    for day in Day::ALL {
        if let Ok((solver, [.., big_input])) = aoc(year, day) {
            let t1 = solver.timed_solve(Part1, big_input)?.1;
            let t2 = solver.timed_solve(Part2, big_input)?.1;
            results.push((day, t1, t2));
        }
    }
    results.sort_by_key(|(_, t1, t2)| *t1 + *t2);
    if !results.is_empty() {
        println!("========== {year:?} ==========");
        for (day, t1, t2) in &results {
            println!("{day:?}: {t1:?} + {t2:?} == {:?}", *t1 + *t2);
        }
        let t1s: Duration = results.iter().map(|(_, t1, _)| t1).sum();
        let t2s: Duration = results.iter().map(|(_, _, t2)| t2).sum();
        println!(
            "{} days: {t1s:?} + {t2s:?} == {:?}\n",
            results.len(),
            t1s + t2s
        );
    }
    Ok(())
}

const HELP: &str = "\
USAGE:
  cargo run [YEAR [DAY [PART [INDEX]]]]

FLAGS:
  -h, --help         Prints help information

ARGS:
  YEAR    [20]15..   Or run all years only on your big inputs   timings only
  DAY     1..=25     Or run all days only on your big inputs    timings only
  PART    1 | 2      Or run all parts on all inputs             answers & timings
  INDEX   0..        Or run all inputs                          answers & timings
";

#[derive(Debug)]
struct Args {
    year: Option<Year>,
    day: Option<Day>,
    part: Option<Part>,
    index: Option<usize>,
}

impl Args {
    fn from_env() -> Result<Self> {
        let args: Vec<_> = env::args().skip(1).collect();
        if args.iter().any(|s| ["-h", "--help"].contains(&s.as_str())) {
            print!("{HELP}");
            std::process::exit(0);
        }
        ensure!(args.len() <= 4, "Up to four arguments expected");
        Ok(Self {
            year: args.first().map(|s| s.parse()).transpose()?,
            day: args.get(1).map(|s| s.parse()).transpose()?,
            part: args.get(2).map(|s| s.parse()).transpose()?,
            index: args.get(3).map(|s| s.parse()).transpose()?,
        })
    }

    fn run(&self) -> Result<()> {
        // All solvers on big inputs.
        let Some(year) = self.year else {
            for year in Year::ALL {
                run_big_inputs(year)?;
            }
            return Ok(());
        };
        // All solvers of the given year on big inputs.
        let Some(day) = self.day else {
            return run_big_inputs(year);
        };
        // Given part or both.
        let parts = self.part.map_or_else(|| Part::ALL.to_vec(), |p| vec![p]);
        // Some part(s) and some inputs for a given puzzle.
        run_some_inputs(year, day, &parts, self.index)
    }
}

fn main() -> Result<()> {
    Args::from_env()?.run()
}
