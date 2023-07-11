#![allow(clippy::expect_used)]
use std::collections::{BinaryHeap, HashMap};

use itertools::Itertools;
use petgraph::{algo::floyd_warshall, graph::NodeIndex, Graph, Undirected};

use common::{prelude::*, Ok};
use utils::{HeuristicItem, OkIterator};

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

const TOP: u16 = 300;

fn best_pressure<const N: usize>(
    rates: &[u32],
    distances: &[Vec<u32>],
    mut minutes_left: u32,
) -> u32 {
    // println!("Flow rates: {:?}", rates);
    // println!("Distances between valuable valves:");
    // for d in distances {
    //     println!("{:?}", d);
    // }
    let start = State::<N>::new(distances.len(), minutes_left);
    // The priority queue alone would consider all states with slowly decreasing `minutes_left`.
    // I only consider up to `TOP` states for each `minutes_left` I encounter to fasten the search.
    // That way, I ensure it only considers up to `(minutes_left + 1) * TOP` states.
    let mut pqueue = BinaryHeap::from([start.h_item()]);
    let mut max_pressure = 0;
    let mut count = TOP;
    while let Some(HeuristicItem {
        heuristic: (min_left, pressure),
        item,
    }) = pqueue.pop()
    {
        debug_assert!(min_left <= minutes_left);
        if min_left < minutes_left {
            (minutes_left, count) = (min_left, TOP);
        }
        if max_pressure < pressure {
            max_pressure = pressure;
        }
        let mut new = item.neighbors(distances, rates);
        count -= 1;
        if count == 0 {
            let keep = |hi: &HeuristicItem<(u32, _), _>| {
                debug_assert!(hi.heuristic.0 <= minutes_left);
                hi.heuristic.0 < minutes_left
            };
            pqueue.retain(keep);
            new.retain(keep);
            // NOTE: `count` will be reset to `TOP` next time
            // because there is no item left with `minutes_left`.
        }
        pqueue.extend(new);
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

    fn h_item(self) -> HeuristicItem<(u32, u32), Self> {
        HeuristicItem::new((self.minutes_left(), self.pressure), self)
    }

    fn neighbors(
        &self,
        distances: &[Vec<u32>],
        rates: &[u32],
    ) -> Vec<HeuristicItem<(u32, u32), Self>> {
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
                res.map(Self::h_item)
            })
            .collect()
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
