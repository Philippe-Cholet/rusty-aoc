use common::{ensure, Context, Part, Part1, Result};
use utils::OkIterator;

/// Science for Hungry People
pub fn solver(part: Part, input: &str) -> Result<String> {
    let data = input
        .lines()
        .map(|line| {
            let (_name, properties) = line.split_once(": ").context("No colon")?;
            properties
                .split(", ")
                .enumerate()
                .map(|(idx, s)| {
                    let (prop, n) = s.split_once(' ').context("No space")?;
                    ensure!(
                        matches!(
                            (idx, prop),
                            (0, "capacity")
                                | (1, "durability")
                                | (2, "flavor")
                                | (3, "texture")
                                | (4, "calories"),
                        ),
                        "Properties in wrong order",
                    );
                    Ok(n.parse::<i32>()?)
                })
                .ok_collect_array::<5>()
        })
        .ok_collect_vec()?;
    let nb_ings = data.len();
    let mut all_quantities = vec![vec![0; nb_ings]];
    for idx in 0..nb_ings {
        let last = idx + 1 == nb_ings;
        all_quantities = all_quantities
            .into_iter()
            .flat_map(|quantities| {
                let remaining = 100 - quantities.iter().copied().sum::<u8>();
                (if last { remaining } else { 0 }..=remaining).map(move |quantity| {
                    let mut new_qs = quantities.clone();
                    new_qs[idx] = quantity;
                    new_qs
                })
            })
            .collect();
    }
    #[cfg(debug_assertions)]
    println!("{:?} possibilities", all_quantities.len()); // 176851 for me
    let property_result = |idx, quantities: &[u8]| {
        data.iter()
            .zip(quantities.iter())
            .map(|(values, quantity)| values[idx] * i32::from(*quantity))
            .sum::<i32>()
            .max(0)
    };
    Ok(all_quantities
        .into_iter()
        .filter_map(|quantities| {
            (part == Part1 || property_result(4, &quantities) == 500).then(|| {
                (0..4)
                    .map(|idx| property_result(idx, &quantities))
                    .product::<i32>()
            })
        })
        .max()
        .context("No valid recipe")?
        .to_string())
}

pub const INPUTS: [&str; 2] = [
    "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
",
    include_str!("input.txt"),
];

#[test]
fn solver_15_15() -> Result<()> {
    use common::Part2;
    assert_eq!(solver(Part1, INPUTS[0])?, "62842880");
    assert_eq!(solver(Part1, INPUTS[1])?, "13882464");
    assert_eq!(solver(Part2, INPUTS[0])?, "57600000");
    assert_eq!(solver(Part2, INPUTS[1])?, "11171160");
    Ok(())
}
