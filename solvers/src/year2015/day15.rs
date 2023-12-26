use common::prelude::*;
use utils::OkIterator;

/// Science for Hungry People
pub fn solver(part: Part, input: &str) -> Result<i32> {
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
    // `quantities` in `highest_score` should be as long as `data` but `data.len()` is so small that
    // I do not want to allocate (176k times for me) on the heap using `Vec` for so few numbers.
    // I could use assume an upper bound on `data.len()` but I currently don't.
    match data.len() {
        2 => highest_score::<2>(part.one(), &data),
        4 => highest_score::<4>(part.one(), &data),
        _ => bail!("Currently only 2 or 4 ingredients are allowed!"),
    }
    .context("No valid recipe")
}

fn highest_score<const NB_INGS: usize>(original: bool, data: &[[i32; 5]]) -> Option<i32> {
    let property_result = |idx, quantities: &[u8]| {
        data.iter()
            .zip(quantities.iter())
            .map(|(values, quantity)| values[idx] * i32::from(*quantity))
            .sum::<i32>()
            .max(0)
    };
    let mut stack = Vec::with_capacity(150); // No reallocation on all the tests I have.
    stack.push((0, 100, [0; NB_INGS]));
    let mut res: Option<i32> = None;
    while let Some((idx, remaining, mut quantities)) = stack.pop() {
        if idx + 1 == NB_INGS || remaining == 0 {
            quantities[idx] = remaining;
            if original || property_result(4, &quantities) == 500 {
                let candidate: i32 = (0..4).map(|i| property_result(i, &quantities)).product();
                if let Some(m) = &mut res {
                    *m = candidate.max(*m);
                } else {
                    res = Some(candidate);
                }
            }
        } else {
            stack.extend((0..=remaining).map(move |quantity| {
                let mut new_qs = quantities;
                new_qs[idx] = quantity;
                (idx + 1, remaining - quantity, new_qs)
            }));
        }
    }
    res
}

pub const INPUTS: [&str; 2] = [
    "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
",
    include_input!(15 15),
];

#[test]
fn solver_15_15() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 62842880);
    assert_eq!(solver(Part1, INPUTS[1])?, 13882464);
    assert_eq!(solver(Part2, INPUTS[0])?, 57600000);
    assert_eq!(solver(Part2, INPUTS[1])?, 11171160);
    Ok(())
}
