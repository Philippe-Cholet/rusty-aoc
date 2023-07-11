use itertools::Itertools;

use common::prelude::*;
use utils::{char10, OkIterator};

/// Crab Cups
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut cups = input
        .trim_end()
        .chars()
        .map(char10::<u32>)
        .ok_collect_vec()?;
    ensure!(
        {
            let mut copy = cups.clone();
            copy.sort_unstable();
            copy == (1..=9).collect_vec()
        },
        "Not 123456789"
    );
    let (limit, nb_steps) = match part {
        Part1 => (9, 100),
        Part2 => (1_000_000, 10_000_000),
    };
    cups.extend(10..=limit);
    let mut src = cups[0];
    let mut table = vec![0; limit as usize + 1];
    for (start, end) in cups.into_iter().circular_tuple_windows() {
        table[start as usize] = end;
    }
    let mut prev_dst = limit + 1; // No previous destination yet.
    for _step in 1..=nb_steps {
        // src -----> n0 -> n1 -> n2 -----> next_src
        //         dst -----> after_dst
        let n0 = table[src as usize];
        let n1 = table[n0 as usize];
        let n2 = table[n1 as usize];
        let next_src = table[n2 as usize];
        let not_dst = [prev_dst, n0, n1, n2];
        let dst = (1..=5)
            .map(|k| if src > k { src - k } else { limit - (k - src) })
            .find(|dst| !not_dst.contains(dst))
            .context("not enough candidates!?!")?;
        let after_dst = table[dst as usize];
        // Replace 3 links:
        //    >--------------------------->
        //   /                             \
        //  /                               v
        // src  -XX-  n0 -> n1 -> n2  -XX-  next_src
        //            ^            \
        //           /              v
        //         dst  -XX-  after_dst
        table[src as usize] = next_src;
        table[dst as usize] = n0;
        table[n2 as usize] = after_dst;
        prev_dst = dst;
        src = next_src;
    }
    Ok(match part {
        Part1 => (0..8)
            .fold((0, table[1]), |(res, n), _| {
                (10 * res + n, table[n as usize])
            })
            .0
            .to_string(),
        Part2 => {
            let star1 = table[1];
            let star2 = table[star1 as usize];
            #[cfg(debug_assertions)]
            println!("Stars under cups {star1} and {star2}");
            (u64::from(star1) * u64::from(star2)).to_string()
        }
    })
}

pub const INPUTS: [&str; 2] = ["389125467", include_str!("input.txt")];

#[test]
fn solver_20_23() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "67384529");
    assert_eq!(solver(Part1, INPUTS[1])?, "27956483");
    assert_eq!(solver(Part2, INPUTS[0])?, "149245887792"); // 934001 * 159792
    assert_eq!(solver(Part2, INPUTS[1])?, "18930983775");
    Ok(())
}
