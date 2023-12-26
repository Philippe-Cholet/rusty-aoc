use itertools::Itertools;

use common::prelude::*;

struct RangeMaps(Vec<[u64; 3]>);

impl RangeMaps {
    fn convert(&self, seed: u64) -> u64 {
        self.0
            .iter()
            .copied()
            .find(|&[_, src, len]| src <= seed && seed < src + len)
            .map_or(seed, |[dst, src, _]| seed - src + dst)
    }

    fn convert_range(&self, seed_range: (u64, u64)) -> Vec<(u64, u64)> {
        let mut ranges = vec![seed_range];
        let mut output = vec![];
        while let Some((start, end)) = ranges.pop() {
            if self.0.iter().all(|&[dst, src, len]| {
                let s = start.max(src);
                let e = end.min(src + len);
                if s < e {
                    output.push((s - src + dst, e - src + dst));
                    if start < s {
                        ranges.push((start, s));
                    }
                    if e < end {
                        ranges.push((e, end));
                    }
                    return false;
                }
                true // No intersection!
            }) {
                output.push((start, end));
            }
        }
        output
    }
}

/// If You Give A Seed A Fertilizer
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let mut input_parts = input.split("\n\n");
    let seeds: Vec<u64> = input_parts
        .next()
        .context("No seeds")?
        .strip_prefix("seeds: ")
        .context("no seeds prefix")?
        .split_whitespace()
        .map(str::parse)
        .try_collect()?;
    let maps: Vec<_> = input_parts
        .map(|text| {
            let mut lines = text.lines();
            let header = lines.next().context("no map header")?;
            lines
                .map(|line| {
                    let (a, b, c) = line
                        .split_whitespace()
                        .map(str::parse::<u64>)
                        .collect_tuple()
                        .context("Not 3")?;
                    common::Ok([a?, b?, c?])
                })
                .try_collect()
                .map(|v| (header, RangeMaps(v)))
        })
        .try_collect()?;
    ensure!(
        itertools::equal(
            maps.iter().map(|(header, _)| *header),
            [
                "seed-to-soil map:",
                "soil-to-fertilizer map:",
                "fertilizer-to-water map:",
                "water-to-light map:",
                "light-to-temperature map:",
                "temperature-to-humidity map:",
                "humidity-to-location map:",
            ]
        ),
        "The headers are wrongly ordered"
    );
    match part {
        Part1 => seeds
            .into_iter()
            .map(|seed| maps.iter().fold(seed, |acc, (_, map)| map.convert(acc)))
            .min(),
        Part2 => {
            ensure!(seeds.len() % 2 == 0, "Odd number of seeds");
            seeds
                .into_iter()
                .tuples()
                .flat_map(|(seed, len)| {
                    maps.iter().fold(vec![(seed, seed + len)], |acc, (_, map)| {
                        acc.into_iter()
                            .flat_map(|seed_range| map.convert_range(seed_range))
                            .collect()
                    })
                })
                .map(|(start, end)| {
                    debug_assert!(start < end, "Invalid range");
                    // start..end are all valid but we are looking for the minimum.
                    start
                })
                .min()
        }
    }
    .context("No seed")
}

pub const INPUTS: [&str; 2] = [
    "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
    include_input!(23 05),
];

#[test]
fn solver_23_05() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 35);
    assert_eq!(solver(Part1, INPUTS[1])?, 993500720);
    assert_eq!(solver(Part2, INPUTS[0])?, 46);
    assert_eq!(solver(Part2, INPUTS[1])?, 4917124);
    Ok(())
}
