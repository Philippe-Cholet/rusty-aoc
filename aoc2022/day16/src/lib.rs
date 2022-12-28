#![allow(clippy::expect_used)]
use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;
use petgraph::{algo::floyd_warshall, graph::NodeIndex, Graph, Undirected};

use common::{ensure, Context, Part, Part1, Part2, Result};
use utils::OkIterator;

/// Proboscidea Volcanium
pub fn solver(part: Part, input: &str) -> Result<String> {
    let data = input
        .lines()
        .map(|line| {
            let (start, s) = line
                .strip_prefix("Valve ")
                .context("prefix")?
                .split_once(" has flow rate=")
                .context("separator 1")?;
            let (rate, ends) = if s.contains("valves") {
                s.split_once("; tunnels lead to valves ")
                    .context("separator 2")?
            } else {
                s.split_once("; tunnel leads to valve ")
                    .context("separator 3")?
            };
            let ends = ends.split(", ").collect_vec();
            Ok((start, rate.parse::<u32>()?, ends))
        })
        .ok_collect_vec()?;
    let (rates, distances) = valuable_valves(&data)?;
    Ok(match part {
        Part1 => best_pressure::<1>(&rates, &distances, 30),
        Part2 => best_pressure::<2>(&rates, &distances, 26),
    }
    .to_string())
}

fn valuable_valves(data: &[(&str, u32, Vec<&str>)]) -> Result<(Vec<u32>, Vec<Vec<u32>>)> {
    let valve_to_index: HashMap<_, _> = data
        .iter()
        .enumerate()
        .map(|(idx, (valve, _, _))| (*valve, idx))
        .collect();
    let (mut valves, mut rates): (Vec<_>, Vec<_>) = data
        .iter()
        .filter_map(|(start, rate, _)| (rate > &0).then_some((valve_to_index[start], *rate)))
        .unzip();
    ensure!(
        !valves.contains(&valve_to_index["AA"]),
        "AA should have no flow",
    );
    // Insert AA at the start of both valuable vectors.
    valves.insert(0, valve_to_index["AA"]);
    rates.insert(0, 0);
    let graph: Graph<(), (), Undirected, usize> =
        Graph::from_edges(data.iter().flat_map(|(start, _, ends)| {
            ends.iter()
                .map(|end| (valve_to_index[start], valve_to_index[end]))
                .collect_vec()
        }));
    let all_dists = floyd_warshall(&graph, |_| 1u32).expect("Can not have negative cycles");
    let distances: Vec<Vec<_>> = valves
        .iter()
        .map(|u| {
            // +1 (minute) to open the valve
            valves
                .iter()
                .map(|v| all_dists[&(NodeIndex::new(*u), NodeIndex::new(*v))] + 1)
                .collect()
        })
        .collect();
    Ok((rates, distances))
}

const TOP: i32 = 10; // I first solved it with 100_000 but 10 is enough.

fn best_pressure<const N: usize>(rates: &[u32], distances: &[Vec<u32>], minutes_left: u32) -> u32 {
    // println!("Flow rates: {:?}", rates);
    // println!("Distances between valuable valves:");
    // for d in distances {
    //     println!("{:?}", d);
    // }
    let start = State::<N>::new(distances.len(), minutes_left);
    let mut pqueue = BinaryHeap::from([start]);
    let mut max_pressure = 0;
    let mut n = minutes_left;
    let mut top = TOP;
    while let Some(item) = pqueue.pop() {
        let min_left = item.minutes_left();
        match min_left.cmp(&n) {
            std::cmp::Ordering::Equal => top -= 1,
            std::cmp::Ordering::Less => {
                n = min_left;
                top = TOP;
            }
            std::cmp::Ordering::Greater => {}
        }
        if top < 0 {
            continue;
        }
        if item.pressure > max_pressure {
            max_pressure = item.pressure;
        }
        pqueue.extend(item.neighbors(distances, rates));
    }
    max_pressure
}

#[derive(Debug, Clone, Copy)]
struct Active {
    loc: usize,
    minutes_left: u32,
}

#[derive(Debug, Clone)]
struct State<const N: usize> {
    actives: [Active; N],
    visited: Vec<bool>,
    pressure: u32,
}

impl<const N: usize> State<N> {
    fn new(nb_valves: usize, minutes_left: u32) -> Self {
        let mut visited = vec![false; nb_valves];
        visited[0] = true;
        Self {
            actives: [Active {
                loc: 0,
                minutes_left,
            }; N],
            visited,
            pressure: 0,
        }
    }

    fn minutes_left(&self) -> u32 {
        self.actives
            .iter()
            .map(|a| a.minutes_left)
            .min()
            .expect("N != 0")
    }

    fn neighbors(&self, distances: &[Vec<u32>], rates: &[u32]) -> Vec<Self> {
        self.actives
            .map(|active| {
                self.visited
                    .iter()
                    .enumerate()
                    .filter_map(|(new_loc, visited)| {
                        (!visited && active.minutes_left >= distances[active.loc][new_loc]).then(
                            || Active {
                                loc: new_loc,
                                minutes_left: active.minutes_left - distances[active.loc][new_loc],
                            },
                        )
                    })
                    .map(Some)
                    .chain([None])
                    .collect_vec()
            })
            .into_iter()
            .multi_cartesian_product()
            .filter_map(|new_actives| {
                // If active are not in conflict and at least one is active
                // then make a new (neighboring) state.
                let mut res = None;
                for (idx, active) in new_actives.into_iter().enumerate() {
                    if let Some(active) = active {
                        let new = res.get_or_insert_with(|| self.clone());
                        if new.visited[active.loc] {
                            // Multiple actives want to visit the same location.
                            return None;
                        }
                        new.actives[idx] = active;
                        new.visited[active.loc] = true;
                        new.pressure += active.minutes_left * rates[active.loc];
                    }
                }
                res
            })
            .collect()
    }
}

impl<const N: usize> PartialEq for State<N> {
    fn eq(&self, other: &Self) -> bool {
        self.pressure == other.pressure && self.minutes_left() == other.minutes_left()
    }
}

impl<const N: usize> Eq for State<N> {}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.minutes_left()
            .cmp(&other.minutes_left())
            .then(self.pressure.cmp(&other.pressure))
    }
}

pub const INPUTS: [&str; 2] = [
    "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
",
    include_str!("input.txt"),
];

#[test]
fn solver_22_16() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "1651");
    assert_eq!(solver(Part1, INPUTS[1])?, "2059");
    assert_eq!(solver(Part2, INPUTS[0])?, "1707");
    assert_eq!(solver(Part2, INPUTS[1])?, "2790");
    Ok(())
}
