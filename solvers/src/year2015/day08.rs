use common::prelude::*;
use crate::utils::OkIterator;

/// Matchsticks
pub fn solver(part: Part, input: &str) -> Result<usize> {
    input
        .lines()
        .map(|s| match part {
            Part1 => {
                ensure!(
                    s.starts_with('"') && s.ends_with('"'),
                    "Line should be \"...\".",
                );
                let mut prev_is_backslash = false;
                s.chars()
                    .map(|ch| match (prev_is_backslash, ch) {
                        (true, '\\' | '"') => {
                            prev_is_backslash = false;
                            Ok(1) // Double backslash OR Escaping the char '"'.
                        }
                        (true, 'x') => {
                            prev_is_backslash = false;
                            Ok(3) // "\x..".
                        }
                        (true, _) => bail!("Escaping wrong char: {}", ch),
                        (false, _) => {
                            prev_is_backslash = ch == '\\';
                            Ok(0_usize)
                        }
                    })
                    .ok_sum::<usize>()
                    .map(|n| n + 2) // "\"...\"" --> "...".
            }
            Part2 => Ok(format!("{s:?}").len() - s.len()),
        })
        .sum()
}

test_solver! {
    r#"""
"abc"
"aaa\"aaa"
"\x27"
"# => (12, 19),
    include_input!(15 08) => (1371, 2117),
}
