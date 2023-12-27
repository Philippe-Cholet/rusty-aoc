use itertools::Itertools;

use common::{prelude::*, Ok};
use crate::utils::OkIterator;

/// Extended Polymerization
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let (template, rules) = input
        .split_once("\n\n")
        .context("No empty line before the rules")?;
    let rules = rules
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").context("No arrow?!")?;
            let pair = from
                .chars()
                .collect_tuple::<(_, _)>()
                .context("Not 2 chars")?;
            let (middle,) = to.chars().collect_tuple().context("Not 1 char")?;
            Ok((pair, middle))
        })
        .ok_collect_hmap()?;
    let steps = part.value(10, 40);
    let counts = if false {
        // Easier (and nice insertion closure IMO) but too slow.
        let insert = |text: String| {
            text.chars()
                .interleave(text.chars().tuple_windows().map(|pair| rules[&pair]))
                .collect()
        };
        let mut polymer = template.to_string();
        for _step in 1..=steps {
            polymer = insert(polymer);
        }
        polymer.chars().counts()
    } else {
        // I only store pairs and their counts, instead of the whole polymer.
        let mut pairs_counts = template.chars().tuple_windows().counts();
        for _step in 1..=steps {
            pairs_counts = pairs_counts
                .into_iter()
                .flat_map(|((start, end), count)| {
                    let middle = rules[&(start, end)];
                    [((start, middle), count), ((middle, end), count)]
                })
                .into_grouping_map_by(|(key, _count)| *key)
                .fold(0, |prev, _newkey, (_key, count)| prev + count);
        }
        // To not count an element twice, I only consider the first element of each pair.
        let mut counts = pairs_counts
            .into_iter()
            .map(|((start, _), count)| (start, count))
            .into_grouping_map_by(|(start, _count)| *start)
            .fold(0, |prev, _newkey, (_start, count)| prev + count);
        // However, an element is forgotten, the last element of the template,
        // which is constantly the second element of the last pair.
        let last_elem = template.chars().last().context("empty template")?;
        *counts.entry(last_elem).or_insert(0) += 1;
        counts
    };
    counts
        .into_values()
        .minmax()
        .into_option()
        .map(|(min, max)| max - min)
        .context("empty template")
}

pub const INPUTS: [&str; 2] = [
    "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
",
    include_input!(21 14),
];

#[test]
fn solver_21_14() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 1588);
    assert_eq!(solver(Part1, INPUTS[1])?, 4517);
    assert_eq!(solver(Part2, INPUTS[0])?, 2188189693529);
    assert_eq!(solver(Part2, INPUTS[1])?, 4704817645083);
    Ok(())
}
