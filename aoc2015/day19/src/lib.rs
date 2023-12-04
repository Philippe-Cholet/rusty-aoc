use itertools::Itertools;
use memchr::memmem;

use common::prelude::*;
use utils::OkIterator;

/// Medicine for Rudolph
pub fn solver(part: Part, input: &str) -> Result<usize> {
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
        Part1 => replacements(molecule, &repls)
            .map(String::from)
            .unique()
            .count(),
        Part2 => beam_search("e", molecule, repls, 75).context("Too small beam width")?,
        // NOTE: The beam search does not guarantee an optimal solution.
    })
}

fn replacements<'a>(s: &'a str, repls: &'a [(&str, &str)]) -> impl Iterator<Item = Repl<'a>> + 'a {
    repls
        .iter()
        .filter(move |(src, _)| s.len() >= src.len())
        .flat_map(move |(src, dst)| {
            let j = src.len();
            let len = s.len() - j + dst.len();
            memmem::find_iter(s.as_bytes(), src)
                .map(move |i| Repl::new(len, &s[..i], dst, &s[i + j..]))
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
            // TODO: Use `Itertools::k_smallest_by` method if it exists in the future.
            .map(|s| utils::HeuristicItem::new(s.len(), s))
            .k_smallest(beam_width)
            .map(|hi| hi.item)
            .map(Into::into)
            .collect();
        if found {
            return Some(nb_steps);
        }
    }
    None
}

struct Repl<'a>(usize, &'a str, &'a str, &'a str);

impl<'a> Repl<'a> {
    #[inline]
    fn new(len: usize, a: &'a str, b: &'a str, c: &'a str) -> Self {
        debug_assert!(a.len() + b.len() + c.len() == len, "Wrong length provided");
        Self(len, a, b, c)
    }

    #[inline]
    const fn len(&self) -> usize {
        self.0
    }
}

// `impl<'a> std::fmt::Display for Repl<'a>` would not allow me to pre-allocate.
impl<'a> From<Repl<'a>> for String {
    #[inline]
    fn from(value: Repl<'a>) -> Self {
        let Repl(len, a, b, c) = value;
        let mut t = Self::with_capacity(len);
        t.push_str(a);
        t.push_str(b);
        t.push_str(c);
        t
    }
}

impl<'a> PartialEq<str> for Repl<'a> {
    #[inline]
    fn eq(&self, t: &str) -> bool {
        self.0 == t.len()
            && t.starts_with(self.1)
            && t.ends_with(self.3)
            && t[self.1.len()..].starts_with(self.2)
    }
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
    assert_eq!(solver(Part1, INPUTS[0])?, 4);
    assert_eq!(solver(Part1, INPUTS[1])?, 7);
    assert_eq!(solver(Part1, INPUTS[4])?, 509);
    assert_eq!(solver(Part2, INPUTS[2])?, 3);
    assert_eq!(solver(Part2, INPUTS[3])?, 6);
    assert_eq!(solver(Part2, INPUTS[4])?, 195);
    Ok(())
}
