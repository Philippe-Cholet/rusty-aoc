use itertools::{iproduct, Itertools};

use common::prelude::*;
use crate::utils::{permutations_map, OkIterator};

/// All in a Single Night
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let dists_between_cities = input
        .lines()
        .map(|line| {
            let (s, n) = line.split_once(" = ").context("No equal")?;
            let dist: u32 = n.parse()?;
            s.split_once(" to ")
                .context("No to")
                .map(|pair| (pair, dist))
        })
        .ok_collect_vec()?;
    let city2idx: HashMap<_, _> = dists_between_cities
        .iter()
        .flat_map(|((src, dst), _)| [*src, *dst])
        .unique()
        .sorted()
        .enumerate()
        .map(|(idx, city)| (city, idx))
        .collect();
    let nb_cities = city2idx.len();
    let mut dists = vec![vec![0; nb_cities]; nb_cities];
    for ((src, dst), dist) in dists_between_cities {
        let (src, dst) = (city2idx[&src], city2idx[&dst]);
        dists[src][dst] = dist;
        dists[dst][src] = dist;
    }
    #[cfg(debug_assertions)]
    for ds in &dists {
        for d in ds {
            print!(" {d: >3}");
        }
        println!();
    }
    ensure!(
        iproduct!(0..nb_cities, 0..nb_cities).all(|(r, c)| (r == c) == (dists[r][c] == 0)),
        "Positive distances between different cities",
    );
    let mut indexes = (0..nb_cities).collect_vec();
    let all_dists = permutations_map(&mut indexes, |path| {
        path.iter()
            .copied()
            .tuple_windows()
            .map(|(u, v)| dists[u][v])
            .sum::<u32>()
    });
    match part {
        Part1 => all_dists.min(),
        Part2 => all_dists.max(),
    }
    .context("No city")
}

test_solver! {
    "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
" => (605, 982),
    include_input!(15 09) => (251, 898),
}
