use common::prelude::*;
use crate::utils::OkIterator;

/// No Such Thing as Too Much
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let containers: Vec<usize> = input.lines().map(str::parse).ok_collect()?;
    let size = if containers.iter().copied().sum::<usize>() >= 150 {
        150
    } else {
        25
    };
    Ok(match part {
        Part1 => {
            let mut ways = vec![0; size + 1];
            ways[0] = 1_u32;
            for container in containers {
                for i in (container..=size).rev() {
                    ways[i] += ways[i - container];
                }
            }
            ways[size]
        }
        Part2 => {
            let nb = containers.len();
            let mut ways = vec![vec![0; nb]; size + 1];
            ways[0] = vec![1; nb];
            for container in containers {
                for i in (container..=size).rev() {
                    for j in (1..nb).rev() {
                        ways[i][j] += ways[i - container][j - 1];
                    }
                }
            }
            ways[size]
                .iter()
                .copied()
                .find(|&nb| nb != 0)
                .unwrap_or_default()
        }
    })
}

test_solver! {
    "20\n15\n10\n5\n5\n" => (4, 3),
    include_input!(15 17) => (1304, 18),
}
