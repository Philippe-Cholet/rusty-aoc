use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

/// Passage Pathing
pub fn solver(part: Part, input: &str) -> Result<usize> {
    Ok(input.parse::<CaveGraph>()?.nb_paths(part.two()))
}

#[derive(Debug)]
struct CaveGraph {
    // "i -- j" is an edge of the (undirected) graph when `adjacency[i]` contains `j`.
    adjacency: Vec<Vec<usize>>,
    // `0..first_big` for small caves, `first_big..` for big caves.
    first_big: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct PathCounter {
    visited: Vec<bool>,
    // path: Vec<usize>,
    current: usize,
    bonus: bool,
    nb_paths: usize,
}

impl CaveGraph {
    const fn is_big(&self, index: usize) -> bool {
        index >= self.first_big
    }

    const fn is_endpoint(&self, index: usize) -> bool {
        index == self.start || index == self.end
    }

    fn nb_paths(&self, bonus: bool) -> usize {
        let mut visited = vec![false; self.adjacency.len()];
        visited[self.start] = true;
        let mut path_counter = PathCounter {
            visited,
            // path: vec![self.start],
            current: self.start,
            bonus,
            nb_paths: 0,
        };
        path_counter.backtrack(self);
        path_counter.nb_paths
    }
}

impl PathCounter {
    fn backtrack(&mut self, graph: &CaveGraph) {
        if self.current == graph.end {
            self.nb_paths += 1;
        } else {
            let current = self.current;
            for &next in &graph.adjacency[current] {
                let unvisited = !self.visited[next];
                let use_bonus = if unvisited || graph.is_big(next) {
                    false
                } else if self.bonus && !graph.is_endpoint(next) {
                    true
                } else {
                    continue;
                };
                self.visited[next] = true;
                // self.path.push(next);
                self.current = next;
                if use_bonus {
                    self.bonus = false;
                }
                self.backtrack(graph);
                if use_bonus {
                    self.bonus = true;
                }
                self.current = current;
                // let last = self.path.pop();
                // debug_assert_eq!(last, Some(next), "next was just visited");
                if unvisited {
                    self.visited[next] = false;
                }
            }
        }
    }
}

impl std::str::FromStr for CaveGraph {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let edges = s
            .lines()
            .map(|line| line.split_once('-').context("No delimiter"))
            .ok_collect_vec()?;
        let mut caves = edges
            .iter()
            .flat_map(|(u, v)| [u, v])
            .unique()
            .collect_vec();
        let nb_caves = caves.len();
        let is_big = |cave: &&&str| cave.chars().all(char::is_uppercase);
        caves.sort_by_key(is_big); // small caves then big caves
        let name2idx: HashMap<_, _> = caves
            .iter()
            .enumerate()
            .map(|(idx, cave)| (**cave, idx))
            .collect();
        let first_big = caves
            .into_iter()
            .find_position(is_big)
            .context("context")?
            .0;
        let start = *name2idx.get("start").context("missing start")?;
        let end = *name2idx.get("end").context("missing end")?;
        let mut adjacency = vec![vec![]; nb_caves];
        for (u, v) in edges {
            let (i, j) = (name2idx[u], name2idx[v]);
            adjacency[i].push(j);
            adjacency[j].push(i);
        }
        Ok(Self {
            adjacency,
            first_big,
            start,
            end,
        })
    }
}

pub const INPUTS: [&str; 4] = [
    "start-A
start-b
A-c
A-b
b-d
A-end
b-end
",
    "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
",
    "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
",
    include_input!(21 12),
];

#[test]
fn solver_21_12() -> Result<()> {
    for (input, answer) in INPUTS.iter().zip([10, 19, 226, 3497]) {
        assert_eq!(solver(Part1, input)?, answer);
    }
    for (input, answer) in INPUTS.iter().zip([36, 103, 3509, 93686]) {
        assert_eq!(solver(Part2, input)?, answer);
    }
    Ok(())
}
