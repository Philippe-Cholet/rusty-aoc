#![allow(clippy::cast_possible_truncation, clippy::expect_used)]
use itertools::Itertools;
use rand::{self, seq::SliceRandom};

use common::prelude::*;

#[derive(Debug, Clone)]
struct Graph {
    // could alternatively be `HashMap<u16, u32>`
    counts: Vec<Option<u32>>,
    edges: Vec<[u32; 2]>,
    nb_nodes: u32,
}

/// Snowverload
pub fn solver(part: Part, input: &str) -> Result<String> {
    if part.two() {
        return Ok(SUCCESS.to_owned());
    }
    let graph: Graph = input.parse()?;
    Ok(loop {
        let mut alt = graph.clone();
        alt.fast_min_cut();
        if alt.nb_edges() == 3 {
            let (a, b) = alt.two_counts().context("What?!")?;
            #[cfg(debug_assertions)]
            println!("{a} * {b}");
            break a * b;
        }
        #[cfg(debug_assertions)]
        println!("Tick!");
    }
    .to_string())
}

impl Graph {
    fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    fn contract(&mut self, t: u32) {
        let mut rng = rand::thread_rng();
        while self.nb_nodes > t {
            let [src, dst] = *self.edges.choose(&mut rng).expect("No edge?!");
            // Merge `dst` into `src`.
            self.edges.retain_mut(|[a, b]| {
                if *a == dst {
                    *a = src;
                } else if *b == dst {
                    *b = src;
                }
                a != b
            });
            let new_count = self.counts[dst as usize].take().expect("Dead dst");
            *self.counts[src as usize].as_mut().expect("Dead src") += new_count;
            self.nb_nodes -= 1;
        }
    }

    /// [Karger–Stein algorithm](https://en.wikipedia.org/wiki/Karger's_algorithm#Karger–Stein_algorithm)
    fn fast_min_cut(&mut self) {
        if self.nb_nodes <= 6 {
            self.contract(2);
        } else {
            #[allow(clippy::cast_sign_loss)] // It can't become negative.
            let t = 1 + (f64::from(self.nb_nodes) / f64::sqrt(2.0)).ceil() as u32;
            let mut alt = self.clone();
            self.contract(t);
            self.fast_min_cut();
            if self.nb_edges() == 3 {
                return;
            }
            alt.contract(t);
            alt.fast_min_cut();
            if self.nb_edges() > alt.nb_edges() {
                *self = alt;
            }
        }
    }

    fn two_counts(&self) -> Option<(u32, u32)> {
        self.counts.iter().copied().flatten().collect_tuple()
    }
}

impl std::str::FromStr for Graph {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let edges: Vec<_> = s
            .lines()
            .map(|line| {
                line.split_once(": ")
                    .map(|(src, dsts)| dsts.split_whitespace().map(move |dst| [src, dst]))
                    .context("No colon delimiter")
            })
            .flatten_ok()
            .try_collect()?;
        let names = edges
            .iter()
            .flatten()
            .sorted()
            .dedup()
            .copied()
            .collect_vec();
        let edges = edges
            .into_iter()
            .map(|edge| {
                edge.map(|name| {
                    names
                        .iter()
                        .position(|s| s == &name)
                        .expect("names have it all") as u32
                })
            })
            .collect();
        Ok(Self {
            nb_nodes: names.len().try_into()?,
            counts: vec![Some(1); names.len()],
            edges,
        })
    }
}

const SUCCESS: &str = "Global snow production restarted!
Now, how am I gonna go down safely to enjoy all this snow?!";

test_solver! {
    "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
" => "54", // 9 * 6
    include_input!(23 25) => "601344", // 768 * 783
}
