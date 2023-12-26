use common::prelude::*;
use utils::OkIterator;

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

pub const INPUTS: [&str; 1] = [include_input!(15 02)];

#[test]
fn solver_15_02() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 1586300);
    assert_eq!(solver(Part2, INPUTS[0])?, 3737498);
    Ok(())
}
