use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use common::{Context, Part, Part1, Part2, Result};
use utils::FromIterStr;

/// Seven Segment Search
pub fn solver(part: Part, input: &str) -> Result<String> {
    let data: Vec<([&str; 10], [&str; 4])> = input.lines().parse_to_vec(|line| {
        let (signals, four) = line.split_once(" | ").context("no delimiter")?;
        let signals: Vec<_> = signals.split(' ').collect();
        let four: Vec<_> = four.split(' ').collect();
        Ok((
            signals.try_into().ok().context("Not 10 elements")?,
            four.try_into().ok().context("Not 4 elements")?,
        ))
    })?;
    let result = match part {
        Part1 => data
            .iter()
            .flat_map(|(_signal, four)| {
                four.iter()
                    .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
            })
            .count(),
        Part2 => {
            let segments = [
                "abcefg".to_owned(),  // 0
                "cf".to_owned(),      // 1
                "acdeg".to_owned(),   // 2
                "acdfg".to_owned(),   // 3
                "bcdf".to_owned(),    // 4
                "abdfg".to_owned(),   // 5
                "abdefg".to_owned(),  // 6
                "acf".to_owned(),     // 7
                "abcdefg".to_owned(), // 8
                "abcdfg".to_owned(),  // 9
            ];
            let segment2idx: HashMap<String, usize> = segments
                .iter()
                .enumerate()
                .map(|(idx, s)| (s.clone(), idx))
                .collect();
            let answer = HashSet::from(segments);
            let translate = |s: &str, table: &HashMap<char, char>| -> String {
                let mut new_s: Vec<char> = s.chars().map(|c| table[&c]).collect();
                new_s.sort_unstable();
                new_s.into_iter().collect()
            };
            data.iter()
                .map(|(signal, four)| {
                    "abcdefg"
                        .chars()
                        .permutations(7)
                        .find_map(|perm| {
                            let table: HashMap<char, char> = "abcdefg".chars().zip(perm).collect();
                            if (signal
                                .iter()
                                .map(|s| translate(s, &table))
                                .collect::<HashSet<_>>())
                                == answer
                            {
                                Some(
                                    four.map(|s| segment2idx[&translate(s, &table)])
                                        .iter()
                                        .fold(0, |prev, d| prev * 10 + d),
                                )
                            } else {
                                None
                            }
                        })
                        .context("No solution")
                })
                .sum::<Result<_>>()?
        }
    };
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_08() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "26");
    assert_eq!(solver(Part1, INPUTS[1])?, "301");
    assert_eq!(solver(Part2, INPUTS[0])?, "61229");
    assert_eq!(solver(Part2, INPUTS[1])?, "908067");
    Ok(())
}
