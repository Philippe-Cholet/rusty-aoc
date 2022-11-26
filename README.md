# [Advent of Code](https://adventofcode.com), in Rust

## About me
I'm French, I studied mathematics, became a professor two years,
I learnt Python and solved "[CheckiO](https://py.checkio.org/user/Phil15/) missions"
(I created some, most based on [Simon Tatham Puzzles](https://www.chiark.greenend.org.uk/~sgtatham/puzzles/)).

Meanwhile I looked at C/C++ with envy but horror and I kept using Python.
Then I heard of Rust only a few months ago, so I tried on "ugly numbers" (with the crate "num-bigint")
and [Simon Tatham's Inertia](https://www.chiark.greenend.org.uk/~sgtatham/puzzles/js/inertia.html) (with the crate "petgraph").

I like being able to use a low level language without the headaches I expected with C.
I certainly like Rust's "cargo" tools way more than Python's "pip" (+ mypy + black + virtual environments)
and prefer an annoying compiler/buddy saying what I'm doing wrong while I'm writing it.
Plus the rusty "itertools" is way better than the Python one.

I heard about Advent of Code a year ago, solved "AoC 2021" in november 2022 (roughly two weeks).
I will surely solve some previous advents after AoC 2022.

## About this workspace
My main goal here is to practice and write idiomatic Rust solving "Advent of Code" puzzles.
Any of my solvers _should_ parse the input (nothing by hand) and not panic but return an error.

In this workspace, packages are:

- "common" which mostly defines "Year", "Day", "Part" enums used in other packages and reexport "anyhow".
- "cli-runner" to run my solutions from the command line.
- "utils" lists utilities that my solutions can all use.
- The folder "aoc2021/day01" is for the library "aoc21-01" containing a solver, inputs and tests against those inputs.

"cli-runner" usage: `<year [20]15...> <day 1...25> <part 1 2> [<input 0...>]`

When no input is given to the command line, the main/last one is chosen.

I chose this structure to reduce compile times (a package without changes is not recompiled).
I first used "clap" to parse command line arguments but the compile times were longer.

### Dependencies
Just "anyhow" and "itertools" at the moment.

But I might need "regex", "num", "petgraph", "ndarray" at some point, eventually others.

### Commands I use
- `cargo r 2021 23 2 0` to run my solver of the AoC 2021 day 23 for the part 2 on the input for which I have the answer.
- Testing: some variants of `cargo t -r -p aoc21-01`
- Clippy: `cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used -A clippy::missing-errors-doc`
- Rustfmt: `cargo fmt --check`

I also use a rusty binary to create a project for a new AoC puzzle (and fill `lib.rs`) and fully update boilerplate in "cli-runner" files
(crates: "inquire" for nice prompts, "time" to know the date and "tinytemplate"/"serde" for basic templates).

### Roadmap
- There is some boilerplate in "cli-runner" (to choose the solver and inputs) that I would like to replace with a "function-like" procedural macro, maybe.
- Expand/Improve my utilities.
- Solve puzzles.
- Maybe make a "wasm-runner".
