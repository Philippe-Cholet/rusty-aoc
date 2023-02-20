use common::{Part, Part1, Part2, Result};
use utils::OkIterator;

/// I Was Told There Would Be No Math
pub fn solver(part: Part, input: &str) -> Result<String> {
    Ok(input
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
        .ok_sum::<u32>()?
        .to_string())
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_15_02() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "1586300");
    assert_eq!(solver(Part2, INPUTS[0])?, "3737498");
    Ok(())
}
