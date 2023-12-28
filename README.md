# [Advent of Code](https://adventofcode.com), in Rust
Year | Stars | When
---- | ----- | ---------
2021 | 50 🌟 | Nov. 2022
2022 | 50 🌟 | Dec. 2022
2020 | 50 🌟 | Feb. 2023
2015 | 42 🌟 | On a break
2023 | 50 🌟 | Dec. 2023

## About this workspace
My main goal here is to practice and write idiomatic Rust solving "Advent of Code" puzzles.
Any of my solvers _should_ parse the input (nothing by hand) and not panic but return an error.

In this workspace, packages are:

- "common" which defines `Year`, `Day`, `Part` enums used in other packages and reexport "anyhow" and an extended "rustc-hash".
  It also has a prelude for convenience.
- "solvers" is a library for all my solutions and utilities plus a binary to run them from the command line.
- "web" for simple interactions with [adventofcode.com](https://adventofcode.com).

### Performance
[**The Rust Performance Book**](https://nnethercote.github.io/perf-book/) is a must-read.

#### Faster tests
Using "cargo-nextest", run tests in release mode is 70% to 120% faster.
And reports are so much cleaner.

    cargo nextest run -r [--run-ignored [ignored-only|all]]

#### Faster solvers
I use the [flamegraph](https://crates.io/crates/flamegraph) crate to easily detect where my code is not efficient enough
([basic usage](https://ntietz.com/blog/profiling-rust-programs-the-easy-way/)).

Here is what I learnt so far:

- Heap-allocation: prevent it or avoid as much reallocation as possible with:
    - methods like `with_capacity`, `reserve` and insightful values ;
    - shared `vec/hashmap/...` like `fn job(..., &mut shared: HashMap<Data, usize>)`.
- Hash less data. Use a faster hasher? Or avoid hashing entirely when possible (eventually use vectors when possible).
- Know the methods of the data structure you use.
- Use `u32` instead of `usize` when possible on a 64 bits target.
- Otherwise change... the data structure you use OR the entire algorithm, there is fun in starting from scratch again.

### `cli-runner` usage
Arguments: `[<year [20]15...> [<day 1...25> [<part 1 2> [<input 0...>]]]]`

- When no input index is given, the solver runs on all inputs.
- When no part is given, the solver runs on both parts.
- When no day is given, the solver runs on all available days/parts (only big inputs).
- When no year is given, the solver runs on all available years/days/parts (only big inputs).

### `web` usage
The session cookie can be hold in an environment variable (`AOC_TOKEN` by default), in a text file or be given manually.

    cargo web [--token <TOKEN>] <YEAR> <DAY> open [--calendar] [--description] [--input]
    cargo web [--token <TOKEN>] <YEAR> <DAY> download [--calendar <FILEPATH>] [--description <FILEPATH>] [--input <FILEPATH>]
    cargo web [--token <TOKEN>] <YEAR> <DAY> submit <PART> <ANSWER>

## Roadmap
- Expand/Improve my utilities.
- Solve puzzles.
- Solve puzzles faster.
- Maybe make a "wasm-runner".
- Maybe make visualizations.
