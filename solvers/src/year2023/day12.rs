use itertools::Itertools;

use common::prelude::*;

use self::Record::{Damaged, Operational, Unknown};

/// Hot Springs
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut cache = HashMap::with_capacity(256);
    input
        .lines()
        .map(|line| {
            let (records, nbs) = line.split_once(' ').context("No space")?;
            let mut records: Vec<Record> = records.chars().map(TryInto::try_into).try_collect()?;
            let mut nbs: Vec<u8> = nbs.split(',').map(str::parse).try_collect()?;
            if part.two() {
                let len = records.len();
                records.reserve(len * 4 + 4);
                for _ in 0..4 {
                    records.push(Unknown);
                    for i in 0..len {
                        records.push(records[i]);
                    }
                }
                nbs = itertools::repeat_n(&nbs, 5).flatten().copied().collect();
            }
            Ok((records, nbs))
        })
        .process_results(|it| {
            it.map(|(records, nums)| {
                cache.clear();
                multiple_damaged_groups(&mut cache, &records, &nums)
            })
            .sum()
        })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Record {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Record {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => bail!("Wrong record: {}", value),
        })
    }
}

fn one_damaged_group(records: &[Record], num: usize) -> usize {
    let mut dam_pos = records.iter().positions(|rec| rec == &Damaged);
    match dam_pos.next() {
        // No damaged, look for at least `num` successive unknown.
        None => records
            .split(|rec| rec == &Operational)
            .filter(|unknowns| unknowns.len() >= num)
            .map(|unknowns| unknowns.len() - num + 1)
            .sum(),
        Some(first) => {
            let last = dam_pos.next_back().unwrap_or(first);
            if last - first >= num {
                return 0; // damaged records are too far apart
            }
            records
                .split(|rec| rec == &Operational)
                .filter(|window| window.contains(&Damaged))
                .exactly_one()
                .map_or(0, |window| {
                    // I should have some explicit formula for this...
                    window
                        .windows(num)
                        .enumerate()
                        .filter(|(i, _)| {
                            !window[..*i].contains(&Damaged)
                                && !window[*i + num..].contains(&Damaged)
                        })
                        .count()
                })
        }
    }
}

// Some divide-and-conquer algorithm
fn multiple_damaged_groups(
    cache: &mut HashMap<CacheKey, usize>,
    records: &[Record],
    nums: &[u8],
) -> usize {
    let key = cache_key(records, nums);
    if let Some(count) = cache.get(&key) {
        return *count;
    }
    let n = nums.len();
    let total = if n == 0 {
        (!records.contains(&Damaged)).into()
    } else if n == 1 {
        one_damaged_group(records, nums[0] as usize)
    } else {
        let num_idx = n / 2;
        let num = nums[num_idx] as usize;
        // Find all possible places for `num`, then split the task in two smaller tasks.
        records
            .windows(num)
            .positions(|window| !window.contains(&Operational))
            .filter(|&pos| {
                // Positions around must not be damaged or the window would be too large!
                pos.checked_sub(1).map_or(true, |i| records[i] != Damaged)
                    && (pos + num < records.len())
                        .then_some(pos + num)
                        .map_or(true, |i| records[i] != Damaged)
            })
            .map(|pos| {
                let c0 = (num_idx != 0).then(|| {
                    multiple_damaged_groups(
                        cache,
                        &records[..pos.saturating_sub(1)],
                        &nums[..num_idx],
                    )
                });
                let c1 = (c0 != Some(0)).then(|| {
                    multiple_damaged_groups(
                        cache,
                        &records[(pos + num + 1).min(records.len())..],
                        &nums[num_idx + 1..],
                    )
                });
                match (c0, c1) {
                    (None, None) => 0,
                    (Some(c), None) | (None, Some(c)) => c,
                    (Some(c0), Some(c1)) => c0 * c1,
                }
            })
            .sum()
    };
    cache.insert(key, total);
    total
}

/// It depends on limits I assume from the inputs.
type CacheKey = ([u64; 4], u128);

fn cache_key(records: &[Record], nums: &[u8]) -> CacheKey {
    // Each line has 20 records max, so 104 for part 2. So 208 bits is enough to know `records`.
    // [u64; 4] (256 bits) is therefore enough.
    let r = records.len();
    let rec_id = core::array::from_fn(|i| {
        // Slices does not link out-of-bound ranges.
        records[r.min(32 * i)..r.min(32 * (i + 1))]
            .iter()
            .fold(0, |acc, rec| {
                acc << 2
                    | match rec {
                        Operational => 0b01,
                        Damaged => 0b10,
                        Unknown => 0b11,
                    }
            })
    });
    // Each line has 6 nums max, so 30 for part 2. The max number is 15 (4 bits).
    // So 120 bits is enough to know `nums`.
    let n_id = nums.iter().fold(0u128, |acc, n| acc << 4 | u128::from(*n));
    (rec_id, n_id)
}

pub const INPUTS: [&str; 2] = [
    "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
",
    include_input!(23 12),
];

#[test]
fn solver_23_12() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 21);
    assert_eq!(solver(Part1, INPUTS[1])?, 7307);
    assert_eq!(solver(Part2, INPUTS[0])?, 525152);
    assert_eq!(solver(Part2, INPUTS[1])?, 3415570893842);
    Ok(())
}

#[test]
fn examples() {
    let mut cache = HashMap::new();
    let mut part2_from = |s: &str, nums: &[u8]| {
        let records = [s; 5]
            .join("?")
            .chars()
            .map(|ch| Record::try_from(ch).expect("Invalid input"))
            .collect_vec();
        let nums = itertools::repeat_n(nums, 5)
            .flatten()
            .copied()
            .collect_vec();
        multiple_damaged_groups(&mut cache, &records, &nums)
    };
    assert_eq!(part2_from("???.###", &[1, 1, 3]), 1);
    assert_eq!(part2_from(".??..??...?##.", &[1, 1, 3]), 16384);
    assert_eq!(part2_from("?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
    assert_eq!(part2_from("????.#...#...", &[4, 1, 1]), 16);
    assert_eq!(part2_from("????.######..#####.", &[1, 6, 5]), 2500);
    assert_eq!(part2_from("?###????????", &[3, 2, 1]), 506250);
}
