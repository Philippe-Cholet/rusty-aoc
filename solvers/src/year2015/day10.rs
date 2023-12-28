use common::prelude::*;
use crate::utils::{char10, OkIterator};

/// Elves Look, Elves Say
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut ns = input
        .trim_end()
        .chars()
        .map(char10::<u8>)
        .ok_collect_vec()?;
    ensure!(
        ns.iter().all(|n| matches!(n, 1..=3)),
        "Look-and-say sequence consist of only 1s 2s and 3s.",
    );
    ensure!(
        !input.contains("1111") && !input.contains("2222") && !input.contains("3333"),
        "Look-and-say sequence must not contain more than 3 consecutive equal characters.",
    );
    for _ in 0..part.value(40, 50) {
        ns = {
            let mut it = ns.into_iter();
            let Some(mut number) = it.next() else {
                return Ok(0);
            };
            let mut ns = vec![];
            let mut count = 1;
            for n in it {
                if n == number {
                    count += 1;
                } else {
                    ns.push(count);
                    ns.push(number);
                    count = 1;
                    number = n;
                }
            }
            ns.push(count);
            ns.push(number);
            ns
        };
    }
    Ok(ns.len())
}

test_solver!(include_input!(15 10) => (252594, 3579328));
