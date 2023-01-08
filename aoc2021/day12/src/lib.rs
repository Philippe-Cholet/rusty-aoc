use std::collections::{HashMap, HashSet};

use common::{ensure, Context, Part, Part2, Result};

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn is_big(name: &str) -> bool {
    name.chars().all(char::is_uppercase)
}

#[derive(Debug)]
struct Path<'a> {
    points: Vec<&'a str>,
    bonus: bool,
}

impl<'a> Path<'a> {
    fn unvisited(&self, name: &str) -> bool {
        !self.points.contains(&name)
    }

    #[allow(clippy::expect_used)]
    fn neighbors(&self, graph: &'a Graph) -> Vec<&'a str> {
        let last = self.points.last().expect("points is never empty");
        graph[last]
            .iter()
            .filter(|next| self.bonus || is_big(next) || self.unvisited(next))
            .copied()
            .collect()
    }

    fn push(&self, name: &'a str) -> Option<Self> {
        let mut points = self.points.clone();
        points.push(name);
        if is_big(name) || self.unvisited(name) {
            Some(Self {
                points,
                bonus: self.bonus,
            })
        } else if self.bonus && name != "start" && name != "end" {
            Some(Self {
                points,
                bonus: false,
            })
        } else {
            None
        }
    }
}

/// Passage Pathing
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let (u, v) = line.split_once('-').context("No delimiter")?;
        graph.entry(u).or_insert_with(HashSet::new).insert(v);
        graph.entry(v).or_insert_with(HashSet::new).insert(u);
    }
    ensure!(
        graph.contains_key("start") && graph.contains_key("end"),
        "The graph does not contain an endpoint"
    );
    let mut stack = vec![Path {
        points: vec!["start"],
        bonus: part == Part2,
    }];
    let mut nb_paths = 0;
    while let Some(path) = stack.pop() {
        for neighbor in path.neighbors(&graph) {
            if neighbor == "end" {
                // println!("{},end (bonus={})", path.points.join(","), path.bonus);
                nb_paths += 1;
            } else if let Some(new_path) = path.push(neighbor) {
                stack.push(new_path);
            }
        }
    }
    Ok(nb_paths.to_string())
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
    include_str!("input.txt"),
];

#[test]
fn solver_21_12() -> Result<()> {
    use common::Part1;
    for (input, answer) in INPUTS.iter().zip(["10", "19", "226", "3497"]) {
        assert_eq!(solver(Part1, input)?, answer);
    }
    for (input, answer) in INPUTS.iter().zip(["36", "103", "3509", "93686"]) {
        assert_eq!(solver(Part2, input)?, answer);
    }
    Ok(())
}
