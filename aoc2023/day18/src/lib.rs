#![allow(clippy::expect_used)]
use itertools::Itertools;

use common::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

#[derive(Debug)]
struct Rect([i32; 4]);

type Point = (i32, i32);
type Segment = [Point; 2];
type RectsGrid = Vec<Vec<Rect>>;

/// Lavaduct Lagoon
pub fn solver(part: Part, input: &str) -> Result<u64> {
    let data: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let udlr = parts.next().context("Empty line")?;
            let n = parts.next().context("No number in the line")?;
            let last = parts.next().context("No (#......) in the line")?;
            ensure!(parts.next().is_none(), "Too many spaces in the line");
            Ok(match part {
                Part1 => {
                    let dir = match udlr {
                        "R" => Direction::E,
                        "D" => Direction::S,
                        "L" => Direction::W,
                        "U" => Direction::N,
                        _ => bail!("Not R D L U but: {}", udlr),
                    };
                    (dir, n.parse()?)
                }
                Part2 => {
                    let last = last
                        .strip_prefix("(#")
                        .context("Wrong last prefix")?
                        .strip_suffix(')')
                        .context("Wrong last suffix")?;
                    ensure!(last.len() == 6, "Not 6-long");
                    let (n, udlr) = last.split_at(5);
                    let dir = match udlr {
                        "0" => Direction::E,
                        "1" => Direction::S,
                        "2" => Direction::W,
                        "3" => Direction::N,
                        _ => bail!("Wrong 0-3 direction: {}", udlr),
                    };
                    (dir, i32::from_str_radix(n, 16)?)
                }
            })
        })
        .try_collect()?;
    if data.is_empty() {
        return Ok(0);
    }
    let pts = read_ngon(&data)?;
    let (rects_grid, segments) = rectangular_parts(&pts);
    let outside = get_outside(&rects_grid, &segments);
    Ok(get_inside_area(&rects_grid, &outside))
}

/// Starting at `(0, 0)`, get positions of the polygon's points.
fn read_ngon(data: &[(Direction, i32)]) -> Result<Vec<Point>> {
    let mut pts = Vec::with_capacity(data.len());
    let end = data.iter().fold((0, 0), |(r, c), (dir, n)| {
        pts.push((r, c));
        match dir {
            Direction::N => (r - n, c),
            Direction::S => (r + n, c),
            Direction::W => (r, c - n),
            Direction::E => (r, c + n),
        }
    });
    (end == (0, 0))
        .then_some(pts)
        .context("The polygon does not end where it started!")
}

/// Split the ground into (big) rectangles and cut polygon segments on border accordingly.
fn rectangular_parts(pts: &[Point]) -> (RectsGrid, HashSet<Segment>) {
    // Both `rs` and `cs` are reasonably small, leading to a not too big 2D grid.
    let mut rs = pts.iter().map(|(r, _c)| *r).sorted().dedup().collect_vec();
    let mut cs = pts.iter().map(|(_r, c)| *c).sorted().dedup().collect_vec();
    // Add rects on the outside.
    rs.insert(0, rs[0] - 1);
    rs.push(*rs.last().expect("Empty data") + 1);
    cs.insert(0, cs[0] - 1);
    cs.push(*cs.last().expect("Empty data") + 1);
    let segments = pts
        .iter()
        .copied()
        .circular_tuple_windows()
        .flat_map(|((mut r0, mut c0), (mut r1, mut c1))| {
            assert!(r0 == r1 || c0 == c1, "Diagonal?!");
            if r0 == r1 {
                if c0 > c1 {
                    (c0, c1) = (c1, c0);
                }
                cs.iter()
                    .copied()
                    .filter(|&c| c0 <= c && c <= c1)
                    .tuple_windows()
                    .map(|(u, v)| [(r0, u), (r0, v)])
                    .collect_vec()
            } else {
                if r0 > r1 {
                    (r0, r1) = (r1, r0);
                }
                rs.iter()
                    .copied()
                    .filter(|&r| r0 <= r && r <= r1)
                    .tuple_windows()
                    .map(|(u, v)| [(u, c0), (v, c0)])
                    .collect_vec()
            }
        })
        .collect();
    let rects_grid = rs
        .iter()
        .tuple_windows()
        .map(|(r0, r1)| {
            cs.iter()
                .tuple_windows()
                .map(|(c0, c1)| Rect([*r0, *r1, *c0, *c1]))
                .collect()
        })
        .collect();
    (rects_grid, segments)
}

impl Rect {
    const fn border(&self, dir: Direction) -> Segment {
        let a = &self.0;
        match dir {
            Direction::N => [(a[0], a[2]), (a[0], a[3])],
            Direction::S => [(a[1], a[2]), (a[1], a[3])],
            Direction::W => [(a[0], a[2]), (a[1], a[2])],
            Direction::E => [(a[0], a[3]), (a[1], a[3])],
        }
    }

    fn area(&self) -> u64 {
        let a = &self.0;
        u64::try_from(a[1] - a[0]).expect("Positive length")
            * u64::try_from(a[3] - a[2]).expect("Positive length")
    }
}

fn get_outside(rects_grid: &RectsGrid, segments: &HashSet<Segment>) -> HashSet<(usize, usize)> {
    let nrows = rects_grid.len();
    let ncols = rects_grid[0].len();
    // Since I previously added some space around the polygon,
    // (0, 0) is outside and all the outside is accessible from it.
    let mut stack = vec![(0usize, 0usize)];
    let mut outside = HashSet::new();
    while let Some((r, c)) = stack.pop() {
        if !outside.insert((r, c)) {
            continue; // Visited already.
        }
        for dir in [Direction::N, Direction::S, Direction::W, Direction::E] {
            let next_loc = match dir {
                Direction::N => r.checked_sub(1).map(|i| (i, c)),
                Direction::S => (r + 1 < nrows).then_some((r + 1, c)),
                Direction::W => c.checked_sub(1).map(|i| (r, i)),
                Direction::E => (c + 1 < ncols).then_some((r, c + 1)),
            };
            let Some(loc) = next_loc else {
                continue; // Outside the grid.
            };
            if segments.contains(&rects_grid[r][c].border(dir)) {
                continue; // Inside the digged zone.
            }
            stack.push(loc);
        }
    }
    outside
}

/// Compute the area inside the polygon.
///
/// Note that big (inside) rectangles have areas that intersect with the neighboring ones.
fn get_inside_area(rects_grid: &RectsGrid, outside: &HashSet<(usize, usize)>) -> u64 {
    let nrows = rects_grid.len();
    let ncols = rects_grid[0].len();
    let mut total = 0;
    for (r, c) in itertools::iproduct!(0..nrows, 0..ncols) {
        if outside.contains(&(r, c)) {
            continue;
        }
        let rect = &rects_grid[r][c];
        total += rect.area();
        let mut south_east_corner: u8 = 0;
        for (dir, (r0, c0)) in [(Direction::S, (r + 1, c)), (Direction::E, (r, c + 1))] {
            if r0 < nrows && c0 < ncols && outside.contains(&(r0, c0)) {
                let [p0, p1] = rect.border(dir);
                let segment_length = p1.0 - p0.0 + p1.1 - p0.1;
                total += u64::try_from(segment_length).expect("Positive length");
                if dir == Direction::E && !outside.contains(&(r - 1, c + 1)) {
                    total -= 1; // Counted twice.
                }
                south_east_corner += 1;
            }
        }
        if south_east_corner == 2 && outside.contains(&(r + 1, c + 1)) {
            total += 1; // Not counted yet.
        }
    }
    total
}

pub const INPUTS: [&str; 2] = [
    "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
",
    include_input!(23 18),
];

#[test]
fn solver_23_18() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 62);
    assert_eq!(solver(Part1, INPUTS[1])?, 50465);
    assert_eq!(solver(Part2, INPUTS[0])?, 952408144115);
    assert_eq!(solver(Part2, INPUTS[1])?, 82712746433310);
    Ok(())
}
