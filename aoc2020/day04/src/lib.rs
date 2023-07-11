use common::prelude::*;
use utils::OkIterator;

#[derive(Debug)]
struct Passport<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    _country_id: Option<&'a str>,
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        let mut fields = value
            .split_whitespace()
            .map(|s| s.split_once(':').context("no colon"))
            .ok_collect_hmap()?;
        let res = Self {
            birth_year: fields.remove(&"byr"),
            issue_year: fields.remove(&"iyr"),
            expiration_year: fields.remove(&"eyr"),
            height: fields.remove(&"hgt"),
            hair_color: fields.remove(&"hcl"),
            eye_color: fields.remove(&"ecl"),
            passport_id: fields.remove(&"pid"),
            _country_id: fields.remove(&"cid"),
        };
        ensure!(fields.is_empty(), "Unknown fields");
        Ok(res)
    }
}

fn range_check(mut s: &str, mini: i32, maxi: i32, suffix: Option<&str>) -> bool {
    if let Some(suffix) = suffix {
        let Some(s2) = s.strip_suffix(suffix) else {
            return false;
        };
        s = s2;
    };
    s.parse().map_or(false, |nb| mini <= nb && nb <= maxi)
}

impl<'a> Passport<'a> {
    const fn basic_validation(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn real_validation(&self) -> bool {
        let Self {
            birth_year: Some(birth),
            issue_year: Some(issue),
            expiration_year: Some(expiration),
            height: Some(height),
            hair_color: Some(hair),
            eye_color: Some(eye),
            passport_id: Some(pid),
            ..
        } = self else {
            return false;
        };
        range_check(birth, 1920, 2002, None)
            && range_check(issue, 2010, 2020, None)
            && range_check(expiration, 2020, 2030, None)
            && (range_check(height, 150, 193, Some("cm"))
                || range_check(height, 59, 76, Some("in")))
            && hair.strip_prefix('#').map_or(false, |hexes| {
                hexes.len() == 6 && hexes.chars().all(|ch| matches!(ch, '0'..='9' | 'a'..='f'))
            })
            && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(eye)
            && pid.len() == 9
            && pid.chars().all(|ch| ch.is_ascii_digit())
    }
}

/// Passport Processing
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut data = input
        .split("\n\n")
        .map(Passport::try_from)
        .ok_collect_vec()?;
    match part {
        Part1 => data.retain(Passport::basic_validation),
        Part2 => data.retain(Passport::real_validation),
    }
    Ok(data.len().to_string())
}

pub const INPUTS: [&str; 2] = [
    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
",
    include_str!("input.txt"),
];

#[test]
fn solver_20_04() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "2");
    assert_eq!(solver(Part1, INPUTS[1])?, "226");
    // assert_eq!(solver(Part2, INPUTS[0])?, "2");
    assert_eq!(solver(Part2, INPUTS[1])?, "160");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{OkIterator, Passport, Result};

    const INVALIDS: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";
    const VALIDS: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";

    #[test]
    fn invalid_passports() -> Result<()> {
        let passports = INVALIDS
            .split("\n\n")
            .map(Passport::try_from)
            .ok_collect_vec()?;
        for invalid in passports {
            assert!(!invalid.real_validation(), "{invalid:?}");
        }
        Ok(())
    }

    #[test]
    fn valid_passports() -> Result<()> {
        let passports = VALIDS
            .split("\n\n")
            .map(Passport::try_from)
            .ok_collect_vec()?;
        for valid in passports {
            assert!(valid.real_validation(), "{valid:?}");
        }
        Ok(())
    }
}
