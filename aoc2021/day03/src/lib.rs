use common::{bail, Context, Part, Part1, Part2, Result};

/// Binary Diagnostic
pub fn solver(part: Part, input: &str) -> Result<String> {
    let result: u32 = match part {
        Part1 => {
            let mut counts = vec![];
            for line in input.lines() {
                if counts.is_empty() {
                    counts = vec![0; line.chars().count()];
                }
                for (k, ch) in line.chars().enumerate() {
                    match ch {
                        '0' => counts[k] -= 1,
                        '1' => counts[k] += 1,
                        _ => bail!("Not binary: {}", ch),
                    }
                }
            }
            let mut gamma_rate = 0;
            let mut epsilon_rate = 0;
            for (k, count) in counts.into_iter().rev().enumerate() {
                match count {
                    0 => bail!("There are as many 0s as 1s."),
                    1.. => gamma_rate |= 1 << k,
                    _ => epsilon_rate |= 1 << k,
                }
            }
            gamma_rate * epsilon_rate // power_consumption
        }
        Part2 => {
            fn get_rating(lines: Vec<&str>, bit_criteria: fn(usize, usize) -> bool) -> Result<u32> {
                let nb_bits = lines.first().context("No line")?.chars().count();
                let mut to_look = lines;
                for k in 0..nb_bits {
                    let mut zeros = vec![];
                    let mut ones = vec![];
                    for line in to_look {
                        match line.chars().nth(k) {
                            Some('0') => zeros.push(line),
                            Some('1') => ones.push(line),
                            Some(ch) => bail!("Not binary: {}", ch),
                            None => bail!("Should be able to get the {}-th char", k),
                        }
                    }
                    to_look = if bit_criteria(zeros.len(), ones.len()) {
                        ones
                    } else {
                        zeros
                    };
                    if to_look.len() <= 1 {
                        break;
                    }
                }
                Ok(u32::from_str_radix(to_look[0], 2)?)
            }
            let lines: Vec<_> = input.lines().collect();
            let oxygen_generator = get_rating(lines.clone(), |n0, n1| n1 >= n0)?;
            let co2_scrubber = get_rating(lines, |n0, n1| n1 < n0)?;
            oxygen_generator * co2_scrubber // life_support
        }
    };
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_03() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "198");
    assert_eq!(solver(Part1, INPUTS[1])?, "3958484");
    assert_eq!(solver(Part2, INPUTS[0])?, "230");
    assert_eq!(solver(Part2, INPUTS[1])?, "1613181");
    Ok(())
}
