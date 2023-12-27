use common::prelude::*;
use crate::utils::parse_to_grid;

const NEIGHBORS_8: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
const OCCUPIED: Option<bool> = Some(false); // Some seat that is NOT available.

/// Seating System
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut available_seats = parse_to_grid(input.lines(), |ch| match ch {
        'L' => Ok(Some(true)),
        '.' => Ok(None),
        ch => bail!("Wrong char: {}", ch),
    })?;
    let (nrows, ncols) = (available_seats.len(), available_seats[0].len());
    let (min_seats, max_k) = part.value((4, 1), (5, isize::MAX));
    loop {
        let first_seat = |r: usize, c: usize, (dr, dc)| {
            (1..=max_k)
                // Map to coords while it remains in the grid.
                .map_while(|k| {
                    let r0 = r.checked_add_signed(k * dr)?;
                    let c0 = c.checked_add_signed(k * dc)?;
                    (r0 < nrows && c0 < ncols).then_some((r0, c0))
                })
                // then first seat, available or not.
                .find(|&(r0, c0)| available_seats[r0][c0].is_some())
        };
        let mut new_seats = available_seats.clone();
        let mut changed = false;
        for (r, row) in available_seats.iter().enumerate() {
            for (c, opt_seat) in row.iter().enumerate() {
                let &Some(free) = opt_seat else {
                    continue; // It's the floor!
                };
                let nb_occup = NEIGHBORS_8
                    .iter()
                    .filter_map(|d| first_seat(r, c, *d))
                    .filter(|&(r0, c0)| available_seats[r0][c0] == OCCUPIED)
                    .take(if free { 1 } else { min_seats }) // just to cut down the count
                    .count();
                if (free && nb_occup == 0) || (!free && nb_occup >= min_seats) {
                    new_seats[r][c] = Some(!free);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        available_seats = new_seats;
        // for row in &available_seats {
        //     for s in row {
        //         let ch = match s {
        //             Some(true) => 'L',
        //             Some(false) => '#',
        //             None => '.',
        //         };
        //         print!("{ch}");
        //     }
        //     println!();
        // }
        // println!();
    }
    Ok(available_seats
        .into_iter()
        .flatten()
        .filter(|cell| cell == &OCCUPIED)
        .count())
}

pub const INPUTS: [&str; 2] = [
    "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
",
    include_input!(20 11),
];

#[test]
fn solver_20_11() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 37);
    assert_eq!(solver(Part1, INPUTS[1])?, 2354);
    assert_eq!(solver(Part2, INPUTS[0])?, 26);
    assert_eq!(solver(Part2, INPUTS[1])?, 2072);
    Ok(())
}
