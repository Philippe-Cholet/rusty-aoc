use common::{ensure, Part, Part1, Part2, Result};
use utils::FromIterStr;

/// Lanternfish
pub fn solver(part: Part, input: &str) -> Result<String> {
    let days = match part {
        Part1 => 80,
        Part2 => 256,
    };
    let ages: Vec<usize> = input.split(',').parse_str_to_vec()?;
    let mut state = [0usize; 9];
    for age in ages {
        ensure!(
            age < 9,
            "I'm too young for this, how can I have {} days left?!",
            age
        );
        state[age] += 1;
    }
    println!("Initial state: {state:?}");
    for day in 1..=days {
        let nb_new_fish = state[0];
        state.rotate_left(1);
        state[6] += nb_new_fish;
        if day <= 18 || day == days {
            println!("After {day: >2} days: {state:?}");
        }
    }
    let result: usize = state.iter().sum();
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = ["3,4,3,1,2", include_str!("input.txt")];

#[test]
fn solver_21_06() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "5934");
    assert_eq!(solver(Part1, INPUTS[1])?, "363101");
    assert_eq!(solver(Part2, INPUTS[0])?, "26984457539");
    assert_eq!(solver(Part2, INPUTS[1])?, "1644286074024");
    Ok(())
}
