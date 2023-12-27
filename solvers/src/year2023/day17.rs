use std::collections::BinaryHeap;

use common::prelude::*;
use crate::utils::{char10, parse_to_grid, HeuristicItem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    W,
    E,
}

/// Clumsy Crucible
#[allow(clippy::expect_used)]
pub fn solver(part: Part, input: &str) -> Result<u16> {
    let grid = parse_to_grid(input.lines(), char10::<u16>)?;
    // TODO: ensure the grid is rectangular
    let nrows = grid.len();
    let ncols = grid[0].len();
    let goal = (nrows - 1, ncols - 1);

    let mut frontier = BinaryHeap::new();
    let mut cost_so_far = HashMap::with_capacity(nrows * ncols);
    let (min_nb_moves, max_nb_moves) = part.value((1, 3), (4, 10));
    for dir in [Direction::E, Direction::S] {
        let Some(loc) = dir.next_loc_by(min_nb_moves, (0, 0), (nrows, ncols)) else {
            continue;
        };
        let heat_loss: u16 = (1..=min_nb_moves)
            .map(|i| {
                let (r, c) = dir
                    .next_loc_by(i, (0, 0), (nrows, ncols))
                    .expect("(0, 0)..=loc segment is inside the grid");
                grid[r][c]
            })
            .sum();
        frontier.push(HeuristicItem::rev(heat_loss, (loc, dir, min_nb_moves)));
        cost_so_far.insert((loc, dir, min_nb_moves), heat_loss);
    }
    while let Some(HeuristicItem {
        item: (loc, dir, count),
        ..
    }) = frontier.pop()
    {
        if loc == goal {
            break;
        }
        let heat_loss = cost_so_far[&(loc, dir, count)];
        for new_dir in [Direction::E, Direction::S, Direction::N, Direction::W] {
            // Do not go back!
            if new_dir == dir.opposite() {
                continue;
            }
            let (nb_moves, new_count, new_loc) = if new_dir == dir {
                if count >= max_nb_moves {
                    continue;
                }
                (1, count + 1, new_dir.next_loc_by(1, loc, (nrows, ncols)))
            } else {
                if count < min_nb_moves {
                    continue;
                }
                (
                    min_nb_moves,
                    min_nb_moves,
                    new_dir.next_loc_by(min_nb_moves, loc, (nrows, ncols)),
                )
            };
            if let Some((r0, c0)) = new_loc {
                let new_heat_loss = heat_loss
                    + (1..=nb_moves)
                        .map(|i| {
                            let (r, c) = new_dir
                                .next_loc_by(i, loc, (nrows, ncols))
                                .expect("loc..=(r0, c0) segment is inside the grid");
                            grid[r][c]
                        })
                        .sum::<u16>();
                if new_heat_loss
                    < cost_so_far
                        .get(&((r0, c0), new_dir, new_count))
                        .copied()
                        .unwrap_or(u16::MAX)
                {
                    cost_so_far.insert(((r0, c0), new_dir, new_count), new_heat_loss);
                    frontier.push(HeuristicItem::rev(
                        new_heat_loss,
                        ((r0, c0), new_dir, new_count),
                    ));
                }
            }
        }
    }
    cost_so_far
        .iter()
        .filter_map(|((loc, ..), heat_loss)| (loc == &goal).then_some(*heat_loss))
        .min()
        .context("Goal not reached!")
}

impl Direction {
    const fn opposite(self) -> Self {
        match self {
            Self::N => Self::S,
            Self::S => Self::N,
            Self::W => Self::E,
            Self::E => Self::W,
        }
    }

    fn next_loc_by(
        self,
        amount: usize,
        (r, c): (usize, usize),
        (nrows, ncols): (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Self::N => r.checked_sub(amount).map(|i| (i, c)),
            Self::S => (r + amount < nrows).then_some((r + amount, c)),
            Self::W => c.checked_sub(amount).map(|i| (r, i)),
            Self::E => (c + amount < ncols).then_some((r, c + amount)),
        }
    }
}

pub const INPUTS: [&str; 2] = [
    "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
",
    include_input!(23 17),
];

#[test]
fn solver_23_17() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 102);
    assert_eq!(solver(Part1, INPUTS[1])?, 902);
    assert_eq!(solver(Part2, INPUTS[0])?, 94);
    assert_eq!(solver(Part2, INPUTS[1])?, 1073);
    Ok(())
}
