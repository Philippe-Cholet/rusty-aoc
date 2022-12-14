# [Advent of Code](https://adventofcode.com), in Rust
Year | Stars | When
---- | ----- | ---------
2021 | 50 ð | Nov. 2022
2022 | 50 ð | DÃ©c. 2022
2020 |       | Soon

## About this workspace
My main goal here is to practice and write idiomatic Rust solving "Advent of Code" puzzles.
Any of my solvers _should_ parse the input (nothing by hand) and not panic but return an error.

In this workspace, packages are:

- "common" which mostly defines "Year", "Day", "Part" enums used in other packages and reexport "anyhow".
- "aoc-macro" defines a "function-like" procedural macro "make_aoc" generating a function "aoc" to get
  the solver and inputs for a given year and day (it reads the cargo manifest to detect dependencies).
- "cli-runner" to run my solutions from the command line.
- "utils" lists utilities that my solutions can all use.
- The folder "aoc2021/day01" is for the library "aoc21-01" containing a solver, inputs and tests against those inputs.

"cli-runner" usage: `<year [20]15...> <day 1...25> [<part 1 2> [<input 0...>]]`

When no part is given, the solver runs on both.
When no input index is given, the solver runs on all inputs.

To [reduce compile times](https://endler.dev/2020/rust-compile-times/),
my target directory is on a SSD and packages without changes are not recompiled thanks to this workspace structure.
I first used "clap" to parse command line arguments but the compile times were longer.

### Dependencies
"anyhow" (error handling), "itertools" (iterators are nice), "petgraph" (graph algorithms), "good_lp" (linear programming) at the moment to solve puzzles.

But I might need "regex", "num", "ndarray" at some point, eventually others.

I also use "quote" and "syn" for procedural macros ; and "cargo_toml" to parse dependencies inside a macro.

### Commands I use
- `cargo r 2021 23 2 0` to run my solver of the AoC 2021 day 23 for the part 2 on the input for which I have the answer.
- Testing: some variants of `cargo t -r -p aoc21-01`
- Clippy: `cargo clippy -- -W clippy::all -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used -A clippy::missing-errors-doc`
- Rustfmt: `cargo fmt --check`

I also use a rusty binary to create a project for a new AoC puzzle (and fill `lib.rs`) and update "cli-runner" dependencies
(crates: "inquire" for nice prompts, "time" to know the date and "tinytemplate"/"serde" for basic templates).

### Roadmap
- Expand/Improve my utilities.
- Solve puzzles.
- Maybe make a "wasm-runner".
