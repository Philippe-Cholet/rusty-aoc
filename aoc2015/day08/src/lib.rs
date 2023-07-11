use common::prelude::*;
use utils::OkIterator;

/// Matchsticks
pub fn solver(part: Part, input: &str) -> Result<String> {
    Ok(input
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
        .ok_sum::<usize>()?
        .to_string())
}

pub const INPUTS: [&str; 2] = [
    r#"""
"abc"
"aaa\"aaa"
"\x27"
"#,
    include_str!("input.txt"),
];

#[test]
fn solver_15_08() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "12");
    assert_eq!(solver(Part1, INPUTS[1])?, "1371");
    assert_eq!(solver(Part2, INPUTS[0])?, "19");
    assert_eq!(solver(Part2, INPUTS[1])?, "2117");
    Ok(())
}
