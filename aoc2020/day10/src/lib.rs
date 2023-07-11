use common::prelude::*;
use utils::OkIterator;

/// Adapter Array
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut data = input.lines().map(str::parse::<usize>).ok_collect_vec()?;
    data.push(0);
    data.sort_unstable();
    Ok(match part {
        Part1 => {
            let mut n1: usize = 0;
            let mut n3 = 1; // To the built-in adapter!
            for s in data.windows(2) {
                match s[1] - s[0] {
                    0 => bail!("Two adapters with the same jolt"),
                    1 => n1 += 1,
                    2 => {}
                    3 => n3 += 1,
                    _ => bail!("Too big difference between two adapters"),
                }
            }
            // println!("{:?}", (n1, n3));
            n1 * n3
        }
        Part2 => {
            let max = *data.last().context("I just pushed 0")?;
            let mut nb_paths = vec![0; max + 1];
            nb_paths[0] = 1; // Start at 0, then skip 0 as we know its value.
            for new in data.iter().skip(1) {
                nb_paths[*new] = (1..=3)
                    .filter_map(|diff| new.checked_sub(diff))
                    .filter_map(|prev| data.contains(&prev).then_some(nb_paths[prev]))
                    .sum();
            }
            nb_paths[max]
        }
    }
    .to_string())
}

pub const INPUTS: [&str; 3] = [
    "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4\n",
    "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3\n",
    include_str!("input.txt"),
];

#[test]
fn solver_20_10() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "35"); // 7 * 5
    assert_eq!(solver(Part1, INPUTS[1])?, "220"); // 22 * 10
    assert_eq!(solver(Part1, INPUTS[2])?, "2170"); // 70 * 31
    assert_eq!(solver(Part2, INPUTS[0])?, "8");
    assert_eq!(solver(Part2, INPUTS[1])?, "19208");
    assert_eq!(solver(Part2, INPUTS[2])?, "24803586664192");
    Ok(())
}
