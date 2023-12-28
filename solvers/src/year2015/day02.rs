use common::prelude::*;
use crate::utils::OkIterator;

/// I Was Told There Would Be No Math
pub fn solver(part: Part, input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(str::parse::<u32>)
                .ok_collect_array::<3>()
                .map(|mut dims| {
                    dims.sort_unstable();
                    let [w, h, l] = dims;
                    let (area, perimeter) = (w * h, 2 * (w + h));
                    match part {
                        Part1 => 3 * area + perimeter * l,
                        Part2 => perimeter + area * l,
                    }
                })
        })
        .sum()
}

test_solver!(include_input!(15 02) => (1586300, 3737498));
