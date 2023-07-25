# [Advent of Code](https://adventofcode.com), in Rust
Year | Stars | When
---- | ----- | ---------
2021 | 50 🌟 | Nov. 2022
2022 | 50 🌟 | Dec. 2022
2020 | 50 🌟 | Feb. 2023
2015 | 42 🌟 | On a break

## About this workspace
My main goal here is to practice and write idiomatic Rust solving "Advent of Code" puzzles.
Any of my solvers _should_ parse the input (nothing by hand) and not panic but return an error.

In this workspace, packages are:

- "common" which defines `Year`, `Day`, `Part` enums used in other packages and reexport "anyhow" and an extended "rustc-hash".
  It also has a prelude for convenience.
- "aoc-macro" defines two "function-like" procedural macros:
    - `make_aoc` generates a function "aoc" to get the solver and inputs
      for a given year and day (it reads the cargo manifest to detect dependencies).
    - `make_aoc_tests` generates multiple test functions from a collection of tests
      (tests kept private because we should not share inputs, I keep what I found though).
- "cli-runner" to run my solutions from the command line.
- "utils" lists utilities that my solutions can all use.
- E.g. the folder "aoc2021/day01" is for the library "aoc21-01" containing a solver, inputs and tests against those inputs.

### Performance
#### [Faster compile times](https://endler.dev/2020/rust-compile-times/)
My target directory is on a SSD and packages without changes are not recompiled thanks to this workspace structure.
I first used "clap" to parse command line arguments but the compile times were longer.

#### Faster tests
Using "cargo-nextest", run tests in release mode is 70% to 120% faster.
And reports are so much cleaner.

    cargo nextest run -r [--run-ignored [ignored-only|all]]

#### Faster solvers
I use the [flamegraph](https://crates.io/crates/flamegraph) crate to easily detect where my code is not efficient enough.

Here is what I learnt so far:

- Heap-allocation: prevent it or avoid as much reallocation as possible with methods like `with_capacity`.
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

### Current dependencies
- **anyhow:** error handling
- **rustc-hash:** non-cryptographic but faster hasher
- **itertools:** iterators are nice
- **petgraph:** graph algorithms
- **ndarray:** n-dimensional arrays
- **pest:** Parsing Expression Grammar
- **good_lp:** linear programming
- **md5:** hash function
- **serde_json:** handle JSON

but I might need "regex", "num" at some point, eventually others.

I also thought about "smallvec" and "rayon" but it does not seem to really fasten my code.

#### For procedural macros
- **quote** and **syn**
- **cargo_toml** to parse the dependencies automatically

## Roadmap
- Expand/Improve my utilities.
- Solve puzzles.
- Solve puzzles faster.
- Maybe make a "wasm-runner".
- Maybe make visualizations.
