use common::{Context, Part, Part1, Part2, Result};

/// The Ideal Stocking Stuffer
pub fn solver(part: Part, input: &str) -> Result<String> {
    let max3 = match part {
        Part1 => [0, 0, 0x0F],
        Part2 => [0, 0, 0],
    };
    let mut context = md5::Context::new();
    context.consume(input.trim_end());
    let mut data = vec![b'0'];
    Ok((1..)
        .find_map(|n| {
            let mut ctx = context.clone();
            increment(&mut data);
            ctx.consume(&data);
            let digest = ctx.compute();
            (&digest.0[..3] <= &max3).then_some(n)
        })
        .context("No solution")?
        .to_string())
}

fn increment(data: &mut Vec<u8>) {
    if let Some(idx) = data.iter().rposition(|x| x != &b'9') {
        for n in data.iter_mut().skip(idx + 1) {
            *n = b'0';
        }
        data[idx] += 1;
    } else {
        for n in data.iter_mut() {
            *n = b'0';
        }
        data.push(b'0');
        data[0] += 1;
    }
}

pub const INPUTS: [&str; 2] = ["abcdef", include_str!("input.txt")];

#[test]
fn solver_15_04() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "609043");
    assert_eq!(solver(Part1, INPUTS[1])?, "117946");
    assert_eq!(solver(Part2, INPUTS[1])?, "3938038");
    Ok(())
}

#[test]
#[ignore]
fn test_increment() {
    for n in 0..=100 {
        let mut data = vec![b'0'];
        for _ in 0..n {
            increment(&mut data);
        }
        assert_eq!(data, n.to_string().into_bytes(), "{}", n);
    }
}
