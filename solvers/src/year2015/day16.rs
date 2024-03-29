use std::cmp::Ordering;

use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

/// Aunt Sue
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let msg: HashMap<_, _> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect();
    let data = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let aunt_id = idx + 1;
            let prefix = format!("Sue {aunt_id}: ");
            let line = line.strip_prefix(&prefix).context("Wrong sue prefix")?;
            line.split(", ")
                .map(|s| {
                    let (key, n) = s.split_once(": ").context("No colon")?;
                    ensure!(msg.contains_key(key), "Wrong item: {}", key);
                    Ok((key, n.parse::<u8>()?))
                })
                .ok_collect_vec()
                .map(|kvs| (aunt_id, kvs))
        })
        .ok_collect_vec()?;
    data.into_iter()
        .filter_map(|(aunt_id, keyed_values)| {
            keyed_values
                .into_iter()
                .all(|(key, value)| {
                    value.cmp(&msg[&key])
                        == match (part, key) {
                            (Part2, "cats" | "trees") => Ordering::Greater,
                            (Part2, "pomeranians" | "goldfish") => Ordering::Less,
                            _ => Ordering::Equal,
                        }
                })
                .then_some(aunt_id)
        })
        .exactly_one()
        .map_err(|it| format_err!("Not one aunt but {}.", it.count()))
}

test_solver!(include_input!(15 16) => (373, 260));
