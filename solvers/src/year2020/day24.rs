use std::ops::Add;

use itertools::Itertools;

use common::prelude::*;
use utils::OkIterator;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct CubeCoords(i32, i32, i32);

impl Add for CubeCoords {
    type Output = Self;

    fn add(self, pt: Self) -> Self::Output {
        Self(self.0 + pt.0, self.1 + pt.1, self.2 + pt.2)
    }
}

impl CubeCoords {
    // https://www.redblobgames.com/grids/hexagons/#coordinates-cube
    const EAST: Self = Self(1, 0, -1);
    const NORTH_EAST: Self = Self(1, -1, 0);
    const NORTH_WEST: Self = Self(0, -1, 1);
    const SOUTH_EAST: Self = Self(0, 1, -1);
    const SOUTH_WEST: Self = Self(-1, 1, 0);
    const WEST: Self = Self(-1, 0, 1);

    const ZERO: Self = Self(0, 0, 0);
    const DIRS: [Self; 6] = [
        Self::EAST,
        Self::NORTH_EAST,
        Self::NORTH_WEST,
        Self::SOUTH_EAST,
        Self::SOUTH_WEST,
        Self::WEST,
    ];
    const OPT_DIRS: [Self; 7] = [
        Self::ZERO,
        Self::EAST,
        Self::NORTH_EAST,
        Self::NORTH_WEST,
        Self::SOUTH_EAST,
        Self::SOUTH_WEST,
        Self::WEST,
    ];
}

/// Lobby Layout
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let data = input
        .lines()
        .map(|line| {
            let mut dirs = vec![];
            let mut chars = line.chars();
            while let Some(ch) = chars.next() {
                dirs.push(match ch {
                    'e' => CubeCoords::EAST,
                    'w' => CubeCoords::WEST,
                    'n' => match chars.next() {
                        None => bail!("North is not a valid direction"),
                        Some('e') => CubeCoords::NORTH_EAST,
                        Some('w') => CubeCoords::NORTH_WEST,
                        Some(ch) => bail!("\"n{}\" is not a valid direction", ch),
                    },
                    's' => match chars.next() {
                        None => bail!("South is not a valid direction"),
                        Some('e') => CubeCoords::SOUTH_EAST,
                        Some('w') => CubeCoords::SOUTH_WEST,
                        Some(ch) => bail!("\"s{}\" is not a valid direction", ch),
                    },
                    _ => bail!("\"{}\" is not a valid direction", ch),
                });
            }
            Ok(dirs)
        })
        .ok_collect_vec()?;
    let mut pts: HashSet<_> = data
        .into_iter()
        .map(|dirs| dirs.into_iter().fold(CubeCoords::default(), Add::add))
        .counts()
        .into_iter()
        .filter_map(|(xyz, c)| (c % 2 == 1).then_some(xyz))
        .collect();
    for _ in 0..part.value(0, 100) {
        let mut new_pts: HashSet<_> = pts
            .iter()
            .flat_map(|&pt| CubeCoords::OPT_DIRS.map(|d| pt + d))
            .collect();
        new_pts.retain(|&pt| {
            let is_black = pts.contains(&pt);
            let nb_neighbors = CubeCoords::DIRS
                .into_iter()
                .filter(|&d| pts.contains(&(pt + d)))
                .count();
            matches!((is_black, nb_neighbors), (true, 1 | 2) | (false, 2))
        });
        pts = new_pts;
    }
    Ok(pts.len())
}

pub const INPUTS: [&str; 2] = [
    "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
",
    include_input!(20 24),
];

#[test]
fn solver_20_24() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 10);
    assert_eq!(solver(Part1, INPUTS[1])?, 293);
    assert_eq!(solver(Part2, INPUTS[0])?, 2208);
    assert_eq!(solver(Part2, INPUTS[1])?, 3967);
    Ok(())
}
