pub use anyhow::{bail, ensure, format_err, Context, Error, Result};

pub use self::{Day::*, Part::*, Year::*};

pub type AocSolver = fn(Part, &str) -> Result<String>;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Year {
    Year2015,
    Year2016,
    Year2017,
    Year2018,
    Year2019,
    Year2020,
    Year2021,
    Year2022,
    Year2023,
    Year2024,
}

impl TryFrom<String> for Year {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(match value.as_ref() {
            "2015" | "15" => Self::Year2015,
            "2016" | "16" => Self::Year2016,
            "2017" | "17" => Self::Year2017,
            "2018" | "18" => Self::Year2018,
            "2019" | "19" => Self::Year2019,
            "2020" | "20" => Self::Year2020,
            "2021" | "21" => Self::Year2021,
            "2022" | "22" => Self::Year2022,
            "2023" | "23" => Self::Year2023,
            "2024" | "24" => Self::Year2024,
            v => bail!("Failed to parse year (2015..): {}", v),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

impl TryFrom<String> for Day {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(match value.as_ref() {
            "01" | "1" => Self::Day1,
            "02" | "2" => Self::Day2,
            "03" | "3" => Self::Day3,
            "04" | "4" => Self::Day4,
            "05" | "5" => Self::Day5,
            "06" | "6" => Self::Day6,
            "07" | "7" => Self::Day7,
            "08" | "8" => Self::Day8,
            "09" | "9" => Self::Day9,
            "10" => Self::Day10,
            "11" => Self::Day11,
            "12" => Self::Day12,
            "13" => Self::Day13,
            "14" => Self::Day14,
            "15" => Self::Day15,
            "16" => Self::Day16,
            "17" => Self::Day17,
            "18" => Self::Day18,
            "19" => Self::Day19,
            "20" => Self::Day20,
            "21" => Self::Day21,
            "22" => Self::Day22,
            "23" => Self::Day23,
            "24" => Self::Day24,
            "25" => Self::Day25,
            v => bail!("Failed to parse day (1..=25): {}", v),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Part {
    Part1,
    Part2,
}

impl TryFrom<String> for Part {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(match value.as_ref() {
            "1" => Self::Part1,
            "2" => Self::Part2,
            v => bail!("Failed to parse part (1..=2): {}", v),
        })
    }
}
