use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

use itertools::{iproduct, Itertools};

use common::{ensure, Context, Ok, Part, Part1, Part2, Result};

#[derive(Debug, PartialEq, Hash, Eq)]
struct Xyz(i32, i32, i32);

impl Xyz {
    fn arrange(&self, sx: bool, sy: bool, sz: bool, p: u8) -> Self {
        let x = if sx { -self.0 } else { self.0 };
        let y = if sy { -self.1 } else { self.1 };
        let z = if sz { -self.2 } else { self.2 };
        match p {
            0 => Self(x, y, z),
            1 => Self(y, x, z), // x <-> y
            2 => Self(z, y, x), // x <-> z
            3 => Self(x, z, y), // y <-> z
            4 => Self(z, x, y), // x --> y --> z --> x
            5 => Self(y, z, x), // x <-- y <-- z <-- x
            _ => unreachable!("p is supposed to be in 0..6 but p={p}"),
        }
    }

    const fn norm(&self) -> u32 {
        self.0.unsigned_abs() + self.1.unsigned_abs() + self.2.unsigned_abs()
    }
}

impl Add for &Xyz {
    type Output = Xyz;

    fn add(self, other: &Xyz) -> Xyz {
        Xyz(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for &Xyz {
    type Output = Xyz;

    fn sub(self, other: &Xyz) -> Xyz {
        Xyz(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

// All 48 changes are considered here...
fn merge12(positions: &mut HashSet<Xyz>, group: &[Xyz]) -> Option<Xyz> {
    for (sx, sy, sz, p) in iproduct!([false, true], [false, true], [false, true], 0..6) {
        let aligned = group
            .iter()
            .map(|xyz| xyz.arrange(sx, sy, sz, p))
            .collect_vec();
        let offsets = iproduct!(positions.iter(), aligned.iter())
            .map(|(pos, align)| pos - align)
            .counts();
        for (offset, count) in offsets {
            if count >= 12 {
                positions.extend(aligned.into_iter().map(|p| &p + &offset));
                return Some(offset);
            }
        }
    }
    None
}

/// Beacon Scanner
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut data = input.split("\n\n").map(|group| {
        group
            .lines()
            .skip(1) // --- scanner # ---
            .map(|line| {
                let (x, y, z) = line
                    .splitn(3, ',')
                    .map(str::parse)
                    .collect_tuple()
                    .context("Not x,y,z")?;
                Ok(Xyz(x?, y?, z?))
            })
    });
    let mut beacons: HashSet<_> = data.next().context("No scanner")?.try_collect()?;
    let mut groups: Vec<Vec<_>> = data.map(Iterator::collect).try_collect()?;
    let mut scanners = vec![Xyz(0, 0, 0)];
    while !groups.is_empty() {
        let mut no_reunion = true;
        groups.retain(|group| {
            merge12(&mut beacons, group).map_or(true, |offset| {
                scanners.push(offset);
                no_reunion = false;
                false
            })
        });
        ensure!(!no_reunion, "Scanners can not be grouped together");
    }
    Ok(match part {
        Part1 => beacons.len().to_string(),
        Part2 => scanners
            .iter()
            .tuple_combinations()
            .map(|(a, b)| (b - a).norm())
            .max()
            .context("No offset")?
            .to_string(),
    })
}

pub const INPUTS: [&str; 2] = [include_str!("input0.txt"), include_str!("input.txt")];

#[test]
fn solver_21_19() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "79");
    assert_eq!(solver(Part1, INPUTS[1])?, "434");
    assert_eq!(solver(Part2, INPUTS[0])?, "3621");
    assert_eq!(solver(Part2, INPUTS[1])?, "11906");
    Ok(())
}
