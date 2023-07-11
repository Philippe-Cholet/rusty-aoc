use common::prelude::*;
use utils::OkIterator;

/// Shuttle Search
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (timestamp, bus_ids) = input.trim_end().split_once('\n').context("Not two lines")?;
    let timestamp = timestamp.parse::<i64>()?;
    let bus_ids = bus_ids
        .split(',')
        .map(|s| (s != "x").then(|| s.parse::<i64>()).transpose())
        .ok_collect_vec()?;
    Ok(match part {
        Part1 => bus_ids
            .into_iter()
            .flatten()
            .map(|bus_id| {
                let rem = timestamp % bus_id; // == 0 is unlikely.
                let time = if rem == 0 { 0 } else { bus_id - rem };
                (bus_id, time)
            })
            .min_by_key(|(_, time)| *time)
            .map(|(bus_id, time)| bus_id * time)
            .context("No bus")?,
        Part2 => {
            let modular_equation: Vec<_> = bus_ids
                .into_iter()
                .enumerate()
                .filter_map(|(idx, bus_id)| bus_id.map(|bus_id| (idx as i64, bus_id)))
                .collect();
            let n = modular_equation.iter().map(|(_, m)| m).product();
            // for (k, m) in modular_equation: unknown_result + k == 0 (modulo m)
            #[cfg(debug_assertions)]
            println!("{modular_equation:?} mod {n}");
            modular_equation
                .iter()
                .map(|(k, m)| {
                    let n0 = n / m;
                    mod_inv(n0, *m)
                        .map(|n0_inv| n0 * n0_inv * -k)
                        .with_context(|| format_err!("{} has no inverse mod {}", n0, m))
                })
                .ok_sum::<i64>()?
                .rem_euclid(n)
        }
    }
    .to_string())
}

fn mod_inv(a: i64, n: i64) -> Option<i64> {
    let mut t = (0, 1);
    let mut r = (n, a);
    while r.1 != 0 {
        t = (t.1, t.0 - r.0.div_euclid(r.1) * t.1);
        r = (r.1, r.0.rem_euclid(r.1));
    }
    (r.0 <= 1).then_some(t.0)
}

pub const INPUTS: [&str; 2] = ["939\n7,13,x,x,59,x,31,19\n", include_str!("input.txt")];

#[test]
fn solver_20_13() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "295");
    assert_eq!(solver(Part1, INPUTS[1])?, "4938");
    assert_eq!(solver(Part2, INPUTS[0])?, "1068781");
    assert_eq!(solver(Part2, INPUTS[1])?, "230903629977901");
    Ok(())
}

#[test]
fn examples() -> Result<()> {
    assert_eq!(solver(Part2, "0\n17,x,13,19")?, "3417");
    assert_eq!(solver(Part2, "0\n67,7,59,61")?, "754018");
    assert_eq!(solver(Part2, "0\n67,x,7,59,61")?, "779210");
    assert_eq!(solver(Part2, "0\n67,7,x,59,61")?, "1261476");
    assert_eq!(solver(Part2, "0\n1789,37,47,1889")?, "1202161486");
    Ok(())
}
