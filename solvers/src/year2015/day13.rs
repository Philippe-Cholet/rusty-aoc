use itertools::Itertools;

use common::prelude::*;
use crate::utils::{permutations_map, OkIterator};

/// Knights of the Dinner Table
pub fn solver(part: Part, input: &str) -> Result<i32> {
    let data = input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once(" happiness units by sitting next to ")
                .context("Wrong delimiter")?;
            let (src, gain_lose) = left.split_once(" would ").context("No would")?;
            let dst = right.strip_suffix('.').context("No dot")?;
            let happiness: i32 = match gain_lose.split_once(' ') {
                Some(("gain", n)) => n.parse()?,
                Some(("lose", n)) => -n.parse()?,
                _ => bail!("No lose/gain"),
            };
            Ok((src, dst, happiness))
        })
        .ok_collect_vec()?;
    let name2idx: HashMap<_, _> = data
        .iter()
        .flat_map(|(src, dst, _)| [*src, *dst])
        .unique()
        .sorted()
        .enumerate()
        .map(|(idx, name)| (name, idx))
        .collect();
    let nb_people = name2idx.len() + usize::from(part.two());
    let mut happinesses = vec![vec![0; nb_people]; nb_people];
    for (src, dst, happiness) in data {
        happinesses[name2idx[&src]][name2idx[&dst]] = happiness;
    }
    permutations_map(&mut (0..nb_people).collect_vec(), |idxs| {
        idxs.iter()
            .circular_tuple_windows()
            .map(|(src, dst)| happinesses[*src][*dst] + happinesses[*dst][*src])
            .sum::<i32>()
    })
    .max()
    .context("No people around the rable")
}

test_solver! {
    "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
" => 330,
    include_input!(15 13) => (618, 601),
}
