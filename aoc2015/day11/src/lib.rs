use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

const ILOS: [u8; 3] = [8, 11, 14]; // ['i', 'l', 'o'].map(|ch| ch as u8 - b'a')

struct Password([u8; 8]);

/// Corporate Policy
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut pwd: Password = input.trim_end().parse()?;
    pwd.nth(part.value(1, 2));
    Ok(pwd.to_string())
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for n in self.0 {
            write!(f, "{}", (n + b'a') as char)?;
        }
        Ok(())
    }
}

impl Password {
    pub fn new(mut pwd: [u8; 8]) -> Self {
        // Ensure it does not contain I L O.
        let mut ilo = false;
        for n in &mut pwd {
            if ilo {
                *n = 0;
            } else if ILOS.contains(n) {
                *n += 1;
                ilo = true;
            }
        }
        Self(pwd)
    }

    pub fn nth(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    pub fn next(&mut self) {
        loop {
            self.increment();
            if self.is_valid() {
                break;
            }
        }
    }

    fn increment(&mut self) {
        for i in (0..8).rev() {
            self.0[i] += 1;
            if ILOS.contains(&self.0[i]) {
                self.0[i] += 1;
                for idx in i + 1..8 {
                    self.0[idx] = 0;
                }
            }
            if self.0[i] < 26 {
                return;
            }
            self.0[i] = 0;
        }
        self.0 = [0; 8];
    }

    fn is_valid(&self) -> bool {
        self.0
            .iter()
            .tuple_windows()
            .any(|(a, b, c)| *a + 1 == *b && *b + 1 == *c)
            && self
                .0
                .iter()
                .group_by(|n| **n)
                .into_iter()
                .map(|(_, g)| g.count() / 2)
                .sum::<usize>()
                >= 2
    }
}

impl std::str::FromStr for Password {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        s.chars()
            .map(|ch| {
                ch.is_ascii_lowercase()
                    .then_some(ch as u8 - b'a')
                    .context("Not a-z")
            })
            .ok_collect_array::<8>()
            .map(Self::new)
    }
}

pub const INPUTS: [&str; 3] = ["abcdefgh", "ghijklmn", include_str!("input.txt")];

#[test]
fn solver_15_11() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "abcdffaa");
    assert_eq!(solver(Part1, INPUTS[1])?, "ghjaabcc");
    assert_eq!(solver(Part1, INPUTS[2])?, "vzbxxyzz");
    assert_eq!(solver(Part2, INPUTS[2])?, "vzcaabcc");
    Ok(())
}
