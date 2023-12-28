use common::prelude::*;
use crate::utils::OkIterator;

/// Lanternfish
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let days = part.value(80, 256);
    let ages: Vec<usize> = input.split(',').map(str::parse).ok_collect()?;
    let mut state = [0usize; 9];
    for age in ages {
        ensure!(
            age < 9,
            "I'm too young for this, how can I have {} days left?!",
            age
        );
        state[age] += 1;
    }
    #[cfg(debug_assertions)]
    println!("Initial state: {state:?}");
    for day in 1..=days {
        let nb_new_fish = state[0];
        state.rotate_left(1);
        state[6] += nb_new_fish;
        if day <= 18 || day == days {
            #[cfg(debug_assertions)]
            println!("After {day: >2} days: {state:?}");
        }
    }
    Ok(state.iter().sum())
}

test_solver! {
    "3,4,3,1,2" => (5934, 26984457539),
    include_input!(21 06) => (363101, 1644286074024),
}
