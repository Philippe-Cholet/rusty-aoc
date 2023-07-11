use common::prelude::*;

/// The Ideal Stocking Stuffer
pub fn solver(part: Part, input: &str) -> Result<String> {
    let max3 = match part {
        Part1 => [0, 0, 0x0F],
        Part2 => [0, 0, 0],
    };
    let mut context = md5::Context::new();
    context.consume(input.trim_end());
    let mut data = vec![b'0'];
    loop {
        let mut ctx = context.clone();
        increment(&mut data);
        ctx.consume(&data);
        let digest = ctx.compute();
        if &digest.0[..3] <= &max3 {
            return Ok(String::from_utf8(data)?);
        }
    }
}

fn increment(data: &mut Vec<u8>) {
    let idx = data
        .iter_mut()
        .rposition(|d| {
            let nine = d == &b'9';
            if nine {
                *d = b'0';
            }
            !nine
        })
        .unwrap_or_else(|| {
            data.push(b'0');
            0
        });
    data[idx] += 1;
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
    let mut data = vec![b'0'];
    for n in 0..=100 {
        assert_eq!(data, n.to_string().into_bytes(), "{}", n);
        increment(&mut data);
    }
}
