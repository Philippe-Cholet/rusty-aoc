use common::prelude::*;
use crate::utils::OkIterator;

/// Camp Cleanup
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let assignment_pairs = input
        .lines()
        .map(|line| {
            let ns: Vec<u32> = line.splitn(4, [',', '-']).map(str::parse).ok_collect()?;
            ensure!(ns.len() == 4, "Not 4 integers");
            debug_assert!(ns[0] <= ns[1] && ns[2] <= ns[3]);
            Ok(((ns[0], ns[1]), (ns[2], ns[3])))
        })
        .ok_collect_vec()?;
    Ok(assignment_pairs
        .into_iter()
        .filter(|(a, b)| match part {
            Part1 => (a.0 <= b.0 && b.1 <= a.1) || (b.0 <= a.0 && a.1 <= b.1),
            Part2 => !(a.1 < b.0 || b.1 < a.0),
        })
        .count())
}

pub const INPUTS: [&str; 2] = [
    "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
",
    include_input!(22 04),
];

#[test]
fn solver_22_04() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 2);
    assert_eq!(solver(Part1, INPUTS[1])?, 644);
    assert_eq!(solver(Part2, INPUTS[0])?, 4);
    assert_eq!(solver(Part2, INPUTS[1])?, 926);
    Ok(())
}
