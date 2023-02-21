use common::{Context, Part, Part1, Part2, Result};

/// The Ideal Stocking Stuffer
pub fn solver(part: Part, input: &str) -> Result<String> {
    let digest_prefix = "0".repeat(match part {
        Part1 => 5,
        Part2 => 6,
    });
    let mut context = md5::Context::new();
    context.consume(input.trim_end());
    Ok((1..)
        .find_map(|n| {
            let mut ctx = context.clone();
            ctx.consume(n.to_string());
            let digest = ctx.compute();
            format!("{digest:x}")
                .starts_with(&digest_prefix)
                .then_some(n)
        })
        .context("No solution")?
        .to_string())
}

pub const INPUTS: [&str; 2] = ["abcdef", include_str!("input.txt")];

#[test]
fn solver_15_04() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "609043");
    assert_eq!(solver(Part1, INPUTS[1])?, "117946");
    assert_eq!(solver(Part2, INPUTS[1])?, "3938038");
    Ok(())
}
