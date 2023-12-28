use std::collections::hash_map::Entry;
use std::ops::{Add, Sub};

use itertools::{iproduct, Itertools};

use common::{prelude::*, Ok};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Xyz(i32, i32, i32);

#[derive(Debug, Clone, Copy)]
enum Axis3D {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
struct Up(bool);

#[derive(Debug, Clone, Copy)]
enum Rot {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

trait Permute<T> {
    fn permute(self, item: T) -> T;
}

impl Xyz {
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

impl Permute<Xyz> for Axis3D {
    /// Which is the `Z` axis?
    #[inline]
    fn permute(self, Xyz(x, y, z): Xyz) -> Xyz {
        match self {
            Self::X => Xyz(y, z, x),
            Self::Y => Xyz(z, x, y),
            Self::Z => Xyz(x, y, z),
        }
    }
}

impl Permute<Xyz> for Up {
    /// The `Z` axis is up or down?
    #[inline]
    fn permute(self, Xyz(x, y, z): Xyz) -> Xyz {
        if self.0 {
            Xyz(x, y, z)
        } else {
            Xyz(y, x, -z)
        }
    }
}

impl Permute<Xyz> for Rot {
    /// Rotations around the `Z` axis.
    #[inline]
    fn permute(self, Xyz(x, y, z): Xyz) -> Xyz {
        match self {
            Self::Rot0 => Xyz(x, y, z),
            Self::Rot90 => Xyz(-y, x, z),
            Self::Rot180 => Xyz(-x, -y, z),
            Self::Rot270 => Xyz(y, -x, z),
        }
    }
}

// Let's be fancy!
impl<T, A: Permute<T>, B: Permute<T>, C: Permute<T>> Permute<T> for (A, B, C) {
    /// Permute for each parameter.
    #[inline]
    fn permute(self, item: T) -> T {
        self.2.permute(self.1.permute(self.0.permute(item)))
    }
}

fn merge12(
    positions: &HashSet<Xyz>,
    group: &[Xyz],
    offsets: &mut HashMap<Xyz, usize>,
) -> Option<(Xyz, Vec<Xyz>)> {
    offsets.reserve((positions.len() * group.len()).saturating_sub(offsets.len()));
    let mut aligned = Vec::with_capacity(group.len());
    iproduct!(
        [Axis3D::X, Axis3D::Y, Axis3D::Z],
        [Up(true), (Up(false))],
        [Rot::Rot0, Rot::Rot90, Rot::Rot180, Rot::Rot270]
    )
    .find_map(|transformation| {
        aligned.clear();
        aligned.extend(group.iter().cloned().map(|xyz| transformation.permute(xyz)));
        offsets.clear();
        iproduct!(positions.iter(), aligned.iter()).find_map(|(pos, align)| {
            match offsets.entry(pos - align) {
                Entry::Vacant(vacant) => {
                    vacant.insert(1);
                    None
                }
                Entry::Occupied(occupied) => {
                    let count = occupied.into_mut();
                    *count += 1;
                    (*count >= 12).then_some(pos - align)
                }
            }
        })
    })
    .map(|offset| (offset, aligned))
}

/// Beacon Scanner
pub fn solver(part: Part, input: &str) -> Result<u32> {
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
    let mut offsets = HashMap::new();
    while !groups.is_empty() {
        let mut no_reunion = true;
        groups.retain(|group| {
            merge12(&beacons, group, &mut offsets).map_or(true, |(offset, aligned)| {
                beacons.extend(aligned.into_iter().map(|p| &p + &offset));
                scanners.push(offset);
                no_reunion = false;
                false
            })
        });
        ensure!(!no_reunion, "Scanners can not be grouped together");
    }
    match part {
        Part1 => Ok(beacons.len().try_into()?),
        Part2 => scanners
            .iter()
            .tuple_combinations()
            .map(|(a, b)| (b - a).norm())
            .max()
            .context("No offset"),
    }
}

test_solver! {
    include_str!("day19-example.txt") => (79, 3621),
    include_input!(21 19) => (434, 11906)
}
