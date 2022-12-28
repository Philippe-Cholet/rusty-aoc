use common::{ensure, Context, Part, Part1, Part2, Result};
use utils::{str_parse, OkIterator};

const DECRYPTION_KEY: i64 = 811_589_153;

/// Grove Positioning System
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut file: Vec<i64> = input.lines().map(str_parse).ok_collect()?;
    let nb = file.len();
    // Otherwise, `nb - 1` would overflow or `rem_euclid(modulus)` below would panic.
    ensure!(nb >= 2, "Not enough numbers");
    let nb_times = match part {
        Part1 => 1,
        Part2 => {
            file.iter_mut().for_each(|elem| *elem *= DECRYPTION_KEY);
            10
        }
    };
    let mut data: Vec<_> = file.into_iter().enumerate().collect();
    let modulus = (nb - 1) as i64;
    for _ in 0..nb_times {
        for idx in 0..nb {
            let i0 = data
                .iter()
                .position(|(i, _)| i == &idx)
                .context("Missing index")?;
            let i1 = ((i0 as i64 + data[i0].1 - 1).rem_euclid(modulus) + 1).try_into()?;
            if i1 != i0 {
                let elem = data.remove(i0);
                data.insert(i1, elem);
            }
        }
    }
    let i0 = data
        .iter()
        .position(|(_, v)| v == &0)
        .context("data does not contain 0")?;
    let result = data[(i0 + 1000) % nb].1 + data[(i0 + 2000) % nb].1 + data[(i0 + 3000) % nb].1;
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = ["1\n2\n-3\n3\n-2\n0\n4\n", include_str!("input.txt")];

#[test]
fn solver_22_20() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[1])?, "988");
    assert_eq!(solver(Part2, INPUTS[1])?, "7768531372516");
    Ok(())
}
