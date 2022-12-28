use common::{ensure, Context, Part, Part1, Result};
use utils::{char10, OkIterator};

/// Supply Stacks
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (cargo, moves) = input.split_once("\n\n").context("No empty line")?;
    let mut grid: Vec<Vec<_>> = cargo
        .lines()
        .map(|line| line.chars().skip(1).step_by(4).collect())
        .collect();
    let headers = grid.pop().context("No headers")?;
    let nb_crates = headers.len();
    ensure!(
        (1..=nb_crates).collect::<Vec<_>>()
            == headers
                .iter()
                .copied()
                .map(char10::<usize>)
                .ok_collect_vec()?,
        "headers are not 1..=headers.len()",
    );
    let mut stacks: Vec<Vec<_>> = (0..nb_crates)
        .map(|r| {
            grid.iter()
                .rev()
                .map_while(|row| match row.get(r) {
                    Some(' ') | None => None,
                    Some(ch) => Some(ch),
                })
                .collect()
        })
        .collect();
    for line in moves.lines() {
        let (nb, from, to) = {
            let ns: Vec<usize> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            ensure!(ns.len() == 3, "wrong move");
            (ns[0], ns[1] - 1, ns[2] - 1)
        };
        let idx = stacks[from]
            .len()
            .checked_sub(nb)
            .context("Not enough stuff in this stack")?;
        let mut changes = stacks[from].split_off(idx);
        if part == Part1 {
            changes.reverse();
        }
        stacks[to].extend(changes);
    }
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().context("empty stack"))
        .ok_collect()
}

pub const INPUTS: [&str; 2] = [
    "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_05() -> Result<()> {
    use common::Part2;
    assert_eq!(solver(Part1, INPUTS[0])?, "CMZ");
    assert_eq!(solver(Part1, INPUTS[1])?, "LJSVLTWQM");
    assert_eq!(solver(Part2, INPUTS[0])?, "MCD");
    assert_eq!(solver(Part2, INPUTS[1])?, "BRQWDBBJM");
    Ok(())
}
