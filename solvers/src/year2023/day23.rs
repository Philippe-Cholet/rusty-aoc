use itertools::Itertools;
use petgraph::{algo::all_simple_paths, graphmap::GraphMap, Directed, EdgeType, Undirected};

use common::prelude::*;
use utils::parse_to_grid;

type Pt = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Path,
    Forest,
    SteepSlope(Direction),
}

fn longest_path<Ty: EdgeType>(edges: Vec<(Pt, Pt, usize)>, from: Pt, to: Pt) -> Option<usize> {
    let graph = GraphMap::<_, _, Ty>::from_edges(edges);
    all_simple_paths(&graph, from, to, 0, None)
        .map(|path: Vec<_>| {
            path.into_iter()
                .tuple_windows()
                .map(|edge| graph[edge])
                .sum()
        })
        .max()
}

/// A Long Walk
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let grid = parse_to_grid(input.lines(), |ch| match ch {
        '#' => Ok(Cell::Forest),
        '.' => Ok(Cell::Path),
        '>' => Ok(Cell::SteepSlope(Direction::E)),
        'v' => Ok(Cell::SteepSlope(Direction::S)),
        // '<' => Ok(Cell::SteepSlope(Direction::W)),
        // '^' => Ok(Cell::SteepSlope(Direction::N)),
        _ => bail!("Wrong char: {}", ch),
    })?;
    // TODO: ensure the grid is rectangular
    let (nrows, ncols) = (grid.len(), grid[0].len());
    let start = (0, 1);
    let mut edges = vec![];
    let mut stack = vec![(start, Direction::S)];
    let mut been = HashSet::new();
    while let Some((path_start, mut dir)) = stack.pop() {
        if !been.insert((path_start, dir)) {
            continue;
        }
        let (mut r, mut c) = dir
            .next_loc(path_start, (nrows, ncols))
            .context("Wrong direction")?;
        let mut path_length = 1usize;
        loop {
            #[allow(clippy::match_on_vec_items)]
            let nexts = [Direction::S, Direction::E, Direction::N, Direction::W]
                .into_iter()
                .filter(|d| d.opposite() != dir)
                .filter_map(|d| d.next_loc((r, c), (nrows, ncols)).map(|loc| (loc, d)))
                .filter(|((r, c), d)| match grid[*r][*c] {
                    Cell::Forest => false,
                    Cell::Path => true,
                    Cell::SteepSlope(slope) => part.value(d == &slope, true),
                });
            match nexts.exactly_one() {
                Ok(item) => {
                    ((r, c), dir) = item;
                    path_length += 1;
                }
                Err(it) => {
                    edges.push((path_start, (r, c), path_length));
                    for (_, d) in it {
                        stack.push(((r, c), d));
                    }
                    break;
                }
            }
        }
    }
    edges.sort_unstable();
    // For a same edge with multiple weights, keep the biggest one.
    edges = edges
        .into_iter()
        .coalesce(|x, y| {
            if x.0 == y.0 && x.1 == y.1 {
                Ok(if x.2 >= y.2 { x } else { y })
            } else {
                Err((x, y))
            }
        })
        .collect();
    let goal = (nrows - 1, ncols - 2);
    match part {
        Part1 => longest_path::<Directed>(edges, start, goal),
        Part2 => longest_path::<Undirected>(edges, start, goal),
    }
    .context("No valid path")
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

    fn next_loc(self, (r, c): Pt, (nrows, ncols): Pt) -> Option<Pt> {
        match self {
            Self::N => r.checked_sub(1).map(|i| (i, c)),
            Self::S => (r + 1 < nrows).then_some((r + 1, c)),
            Self::W => c.checked_sub(1).map(|i| (r, i)),
            Self::E => (c + 1 < ncols).then_some((r, c + 1)),
        }
    }
}

pub const INPUTS: [&str; 2] = [
    "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
",
    include_input!(23 23),
];

#[test]
fn solver_23_23() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 94);
    assert_eq!(solver(Part1, INPUTS[1])?, 2294);
    assert_eq!(solver(Part2, INPUTS[0])?, 154);
    assert_eq!(solver(Part2, INPUTS[1])?, 6418);
    Ok(())
}
