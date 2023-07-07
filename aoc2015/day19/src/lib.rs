use itertools::Itertools;

use common::{ensure, Context, Part, Part1, Part2, Result};
use utils::OkIterator;

/// Medicine for Rudolph
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (repls, molecule) = input
        .trim_end()
        .split_once("\n\n")
        .context("No empty line")?;
    let repls = repls
        .lines()
        .map(|line| line.split_once(" => ").context("Wrong delimiter"))
        .ok_collect_vec()?;
    ensure!(
        repls.iter().all(|(src, dst)| src.len() <= dst.len()),
        "The process is supposed to grow molecules",
    );
    Ok(match part {
        Part1 => replacements(molecule, &repls).unique().count(),
        Part2 => beam_search("e", molecule, repls, 75).context("Too small beam width")?,
        // NOTE: The beam search does not guarantee an optimal solution.
    }
    .to_string())
}

fn replacements<'a>(s: &'a str, repls: &'a [(&str, &str)]) -> impl Iterator<Item = String> + 'a {
    repls.iter().flat_map(move |(src, dst)| {
        let j = src.len();
        let capacity = s.len() + dst.len() - j;
        s.match_indices(src).map(move |(i, _)| {
            let mut t = String::with_capacity(capacity);
            t.push_str(&s[..i]);
            t.push_str(dst);
            t.push_str(&s[i + j..]);
            t
        })
    })
}

fn beam_search<'a>(
    start: &str,
    end: &str,
    mut repls: Vec<(&'a str, &'a str)>,
    beam_width: usize,
) -> Option<usize> {
    for pair in &mut repls {
        *pair = (pair.1, pair.0);
    }
    let mut beam = vec![end.to_owned()];
    let mut nb_steps = 0;
    while !beam.is_empty() {
        nb_steps += 1;
        let mut found = false;
        beam = beam
            .iter()
            .flat_map(|s| replacements(s, &repls))
            .take_while(|s| {
                if !found {
                    found = s == start;
                }
                !found
            })
            // TODO: Use `Itertools::k_largest_by` method if it exists in the future.
            .map(|s| utils::HeuristicItem::new(s.len(), s))
            .k_smallest(beam_width)
            .map(|hi| hi.item)
            .collect();
        if found {
            return Some(nb_steps);
        }
    }
    None
}

pub const INPUTS: [&str; 5] = [
    "\
H => HO
H => OH
O => HH

HOH
",
    "\
H => HO
H => OH
O => HH

HOHOHO
",
    "\
e => H
e => O
H => HO
H => OH
O => HH

HOH
",
    "\
e => H
e => O
H => HO
H => OH
O => HH

HOHOHO
",
    include_str!("input.txt"),
];

#[test]
fn solver_15_19() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "4");
    assert_eq!(solver(Part1, INPUTS[1])?, "7");
    assert_eq!(solver(Part1, INPUTS[4])?, "509");
    assert_eq!(solver(Part2, INPUTS[2])?, "3");
    assert_eq!(solver(Part2, INPUTS[3])?, "6");
    assert_eq!(solver(Part2, INPUTS[4])?, "195");
    Ok(())
}
