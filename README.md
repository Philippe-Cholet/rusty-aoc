# [Advent of Code](https://adventofcode.com), in Rust

I first learned python (pratice [here](https://py.checkio.org/user/Phil15/))
and wanted to get to know a low level language.
Rust seemed to be a promising option compared to C or C++, and after a
necessary adaptation phase and solving the Advent of Code 2021 (in november 2022),
I must say that I don't have the feeling of going slower than in python
while having the advantages of a low level language and cargo tools.
The compiler is my buddy now.

My main goal here is to practice and write idiomatic Rust.
Any of my solvers should not panic but return an error.

I use clippy (nursery, pedantic, unwrap uses and most expect uses)
to enhance my solutions formatted with rustfmt.
The external dependencies are "anyhow" and "itertools".

In this workspace, packages are:

- "common" which mostly defines "Year", "Day", "Part" enums used in other packages.
- "cli-runner" to run my solutions from the command line.
- "utils" lists utilities that my solutions can all use.
- The folder "aoc2021/day01" is for the library "aoc21-01"
  containing a solver, inputs and tests against those inputs.

Usage: `<year [20]15...> <day 1...25> <part 1 2> [<input 0...>]`

When no input is given to the command line, the big (and last) one is chosen.

Feel free to open an issue about anything! I'm here to learn and get better.
