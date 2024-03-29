use common::prelude::*;
use crate::utils::{permutations_map, OkIterator};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Segm7(u8);

impl std::str::FromStr for Segm7 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n = 0;
        for b in s.bytes() {
            ensure!(matches!(b, b'a'..=b'g'), "Not in a-g range");
            let flag = 1 << (b - b'a');
            ensure!(n & flag == 0, "Duplicate segment");
            n |= flag;
        }
        Ok(Self(n))
    }
}

impl std::fmt::Display for Segm7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, ch) in "abcdefg".chars().enumerate() {
            if self.0 & (1 << i) != 0 {
                write!(f, "{ch}")?;
            }
        }
        Ok(())
    }
}

impl Segm7 {
    #[allow(clippy::unreadable_literal)]
    const DIGITS: [Self; 10] = [
        //     gfedcba
        Self(0b1110111), // 0: abcefg
        Self(0b0100100), // 1: cf
        Self(0b1011101), // 2: acdeg
        Self(0b1101101), // 3: acdfg
        Self(0b0101110), // 4: bcdf
        Self(0b1101011), // 5: abdfg
        Self(0b1111011), // 6: abdefg
        Self(0b0100101), // 7: acf
        Self(0b1111111), // 8: abcdefg
        Self(0b1101111), // 9: abcdfg
    ];

    const fn count_segments(self) -> u32 {
        self.0.count_ones()
    }

    fn bitset(segments: &[Self]) -> u128 {
        segments.iter().fold(0, |n, seg| n | (1 << seg.0))
    }

    fn permute(self, perm: &[u8]) -> Self {
        let mut n = self.0;
        let bits = std::iter::repeat_with(|| {
            let bit = n & 1;
            n >>= 1;
            bit
        });
        Self(
            perm.iter()
                .zip(bits)
                .map(|(p, bit)| bit << p)
                .fold(0, std::ops::BitOr::bitor),
        )
    }

    fn to_digit(self) -> Option<usize> {
        Self::DIGITS.into_iter().position(|seg| seg == self)
    }

    fn read_number(numbers: &[Self], perm: &[u8]) -> Result<usize> {
        // In the example: abcdefg --> cfgabde
        //  aaaa       dddd
        // b    c     e    a
        // b    c     e    a
        //  dddd  -->  ffff
        // e    f     g    b
        // e    f     g    b
        //  gggg       cccc
        // println!("{}", (0..7).map(|i| Self(1 << i).permute(perm)).join(""));
        numbers
            .iter()
            .map(|seg| {
                seg.permute(perm)
                    .to_digit()
                    .context("Not a seven segment digit")
            })
            .ok_fold(0, |res, digit| res * 10 + digit)
    }
}

/// Seven Segment Search
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let data: Vec<([Segm7; 10], [Segm7; 4])> = input
        .lines()
        .map(|line| {
            let (signal, entry) = line.split_once(" | ").context("no delimiter")?;
            common::Ok((
                signal
                    .split_whitespace()
                    .map(str::parse)
                    .ok_collect_array()?,
                entry
                    .split_whitespace()
                    .map(str::parse)
                    .ok_collect_array()?,
            ))
        })
        .ok_collect()?;
    match part {
        Part1 => Ok(data
            .into_iter()
            .flat_map(|line| line.1)
            .filter(|seg| [2, 3, 4, 7].contains(&seg.count_segments()))
            .count()),
        Part2 => {
            let digit_bitset = Segm7::bitset(&Segm7::DIGITS);
            let mut all_perm7 = Vec::with_capacity(5040); // There are `7!` (5040) permutations.
            #[allow(clippy::expect_used)] // Slices of length 7.
            all_perm7.extend(permutations_map(&mut [0, 1, 2, 3, 4, 5, 6], |perm| {
                <[u8; 7]>::try_from(perm).expect("7 long")
            }));
            data.into_iter()
                .flat_map(|(signal, entry)| {
                    all_perm7
                        .iter()
                        .find_map(|perm| {
                            let perm_signal = signal.map(|seg| seg.permute(perm));
                            (Segm7::bitset(&perm_signal) == digit_bitset)
                                .then(|| Segm7::read_number(&entry, perm))
                        })
                        .context("No solution")
                })
                .sum()
        }
    }
}

test_solver! {
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf" => (0, 5353),
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
" => (26, 61229),
    include_input!(21 08) => (301, 908067),
}
