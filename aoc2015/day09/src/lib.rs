use std::collections::HashMap;

use itertools::{iproduct, Itertools};

use common::{ensure, Context, Part, Part1, Part2, Result};
use utils::OkIterator;

/// All in a Single Night
pub fn solver(part: Part, input: &str) -> Result<String> {
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
    let all_dists = (0..nb_cities).permutations(nb_cities).map(|path| {
        path.into_iter()
            .tuple_windows()
            .map(|(u, v)| dists[u][v])
            .sum::<u32>()
    });
    Ok(match part {
        Part1 => all_dists.min(),
        Part2 => all_dists.max(),
    }
    .context("No city")?
    .to_string())
}

pub const INPUTS: [&str; 2] = [
    "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
",
    include_str!("input.txt"),
];

#[test]
fn solver_15_09() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "605");
    assert_eq!(solver(Part1, INPUTS[1])?, "251");
    assert_eq!(solver(Part2, INPUTS[0])?, "982");
    assert_eq!(solver(Part2, INPUTS[1])?, "898");
    Ok(())
}