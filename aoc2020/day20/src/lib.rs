use std::collections::VecDeque;

use itertools::{iproduct, Itertools};

use common::prelude::*;
use utils::{parse_to_grid, OkIterator};

use Rotation::{Rot0, Rot180, Rot270, Rot90};

struct TileCollection {
    id2tile: HashMap<u32, Tile>,
    common_border: HashMap<(u32, u32), Border>,
}

struct Tile {
    grid: Vec<Vec<bool>>,
    borders: [Border; 4], // [North, South, West, East]
}

// NOTE: [North, South, West, East] too!!
const NEIGHBORS_4: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Clone, Copy)]
/// Represents the 10 bits of a tile border, equal to its flip (bits in reverse).
struct Border(u16);

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

/// ```
/// +--------------------+
/// |                  # |
/// |#    ##    ##    ###|
/// | #  #  #  #  #  #   |
/// +--------------------+
/// ```
#[rustfmt::skip]
const PATTERN: [(usize, usize); 15] = [
                                                                                                   (0,18),
(1,0),                   (1,5),(1,6),                    (1,11),(1,12),                     (1,17),(1,18),(1,19),
     (2,1),         (2,4),          (2,7),         (2,10),            (2,13),         (2,16),
];
const P_HEIGHT: usize = 3;
const P_WIDTH: usize = 20;

/// Jurassic Jigsaw
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut tile_collection: TileCollection = input.parse()?;
    Ok(match part {
        Part1 => {
            let ids = tile_collection.sure_corner_ids();
            (ids.len() == 4)
                .then(|| ids.into_iter().map(u64::from).product::<u64>())
                .context("Not sure about some corner(s)")?
                .to_string()
        }
        Part2 => {
            let mut image = tile_collection.merge_tiles()?;
            let (nrows, ncols) = (image.len(), image[0].len());
            let nb_cells = 'find_patterns: {
                for _ns_flip in 0..2 {
                    for _rot in 0..4 {
                        let cells = iproduct!(0..=nrows - P_HEIGHT, 0..=ncols - P_WIDTH)
                            .filter_map(|(r, c)| {
                                let places = PATTERN.map(|(dr, dc)| (r + dr, c + dc));
                                places.iter().all(|(i, j)| image[*i][*j]).then_some(places)
                            })
                            .flatten()
                            .unique();
                        let nb = if cfg!(debug_assertions) {
                            let vec_cells = cells.collect_vec();
                            if !vec_cells.is_empty() {
                                for (r, row) in image.iter().enumerate() {
                                    for (c, black) in row.iter().enumerate() {
                                        match (*black, vec_cells.contains(&(r, c))) {
                                            (true, true) => print!("█"),
                                            (true, false) => print!("#"),
                                            (false, false) => print!("░"),
                                            (false, true) => bail!("Pattern on '░'"),
                                        }
                                    }
                                    println!();
                                }
                            }
                            vec_cells.len()
                        } else {
                            cells.count()
                        };
                        if nb > 0 {
                            break 'find_patterns nb;
                        }
                        // Rotate by 90
                        Rot90.mutate_grid(&mut image);
                    }
                    // Flip North-South
                    image.reverse();
                }
                bail!("Did not find any pattern");
            };
            let nb_blacks = image.iter().flatten().filter(|black| **black).count();
            (nb_blacks - nb_cells).to_string()
        }
    })
}

impl Rotation {
    fn mutate_grid(self, grid: &mut Vec<Vec<bool>>) {
        let (nrows, ncols) = (grid.len(), grid[0].len());
        match self {
            Rot0 => {}
            Rot90 => {
                let mut new = vec![vec![false; nrows]; ncols];
                for (r, row) in grid.iter().enumerate() {
                    for (c, black) in row.iter().enumerate() {
                        new[ncols - 1 - c][r] = *black;
                    }
                }
                *grid = new;
            }
            Rot180 => {
                grid.reverse();
                grid.iter_mut().for_each(|row| row.reverse());
            }
            Rot270 => {
                let mut new = vec![vec![false; nrows]; ncols];
                for (r, row) in grid.iter().enumerate() {
                    for (c, black) in row.iter().enumerate() {
                        new[c][nrows - 1 - r] = *black;
                    }
                }
                *grid = new;
            }
        }
    }
}

impl Border {
    const BITS: u32 = 10;

    fn from_line(line: &[bool]) -> Self {
        Self(line.iter().fold(0, |n, black| (n << 1) | u16::from(*black)))
    }

    const fn rev(self) -> Self {
        Self(self.0.reverse_bits() >> (u16::BITS - Self::BITS))
    }
}

impl PartialEq for Border {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 || self.0 == other.rev().0
    }
}

impl Tile {
    fn satisfy_constraints(&mut self, nswe_borders: [Option<Border>; 4]) -> Result<()> {
        let flip_rot = 'orientation: {
            for ns_flip in [false, true] {
                for rot in [Rot0, Rot90, Rot180, Rot270] {
                    if nswe_borders
                        .iter()
                        .zip(self.borders)
                        .all(|(opt_b, b)| opt_b.map_or(true, |nwse_b| nwse_b == b))
                    {
                        break 'orientation Some((ns_flip, rot));
                    }
                    // Rotate borders by 90
                    self.borders = [
                        self.borders[3],
                        self.borders[2],
                        self.borders[0],
                        self.borders[1],
                    ];
                }
                // Flip borders North-South
                self.borders = [
                    self.borders[1],
                    self.borders[0],
                    self.borders[2].rev(),
                    self.borders[3].rev(),
                ];
            }
            None
        };
        let (ns_flip, rot) = flip_rot.context("Does not match constraints")?;
        if ns_flip {
            self.grid.reverse();
        }
        rot.mutate_grid(&mut self.grid);
        Ok(())
    }
}

fn distances_from(start: u32, graph: &HashMap<u32, Vec<u32>>) -> Result<HashMap<u32, usize>> {
    let mut distances = HashMap::new();
    distances.insert(start, 0);
    let mut been = HashSet::new();
    let mut queue = VecDeque::from([start]);
    while let Some(id) = queue.pop_front() {
        if !been.insert(id) {
            continue;
        }
        let dist = *distances
            .get(&id)
            .context("Was on the queue but without distance?!")?;
        for next_id in graph.get(&id).context("Graph pointing outside")? {
            if been.contains(next_id) {
                continue;
            }
            distances.insert(*next_id, dist + 1);
            queue.push_back(*next_id);
        }
    }
    Ok(distances)
}

impl TileCollection {
    fn sure_corner_ids(&self) -> Vec<u32> {
        self.common_border
            .keys()
            .flat_map(|&(i0, i1)| [i0, i1])
            .counts()
            .into_iter()
            .filter_map(|(id, count)| (count == 2).then_some(id))
            .collect_vec()
    }

    fn grid_ids(&self) -> Result<Vec<Vec<u32>>> {
        let graph = self
            .common_border
            .keys()
            .copied()
            .flat_map(|(u, v)| [(u, v), (v, u)])
            .into_group_map();
        let graph = graph.into_iter().collect(); // RandomState -> FxHasher
        let first_corner_id = *self
            .sure_corner_ids()
            .first()
            .context("No known corner id")?;
        let distances = distances_from(first_corner_id, &graph)?;
        // After grouping ids by distances from an arbitrary corner
        // 01234
        // 1234     The two neighbors of the corner are arbitrary positioned aside the corner's position.
        // 234      THen positions "n+1" are based on the previous one (or two) "n".
        // 34
        // 4
        // ...
        let &dist_max = distances
            .values()
            .max()
            .context("The distance to the first corner is there!")?;
        let mut dist2ids = vec![vec![]; dist_max + 1];
        for (id, dist) in distances {
            dist2ids[dist].push(id);
        }
        let mut id2loc = HashMap::new();
        id2loc.insert(first_corner_id, (0, 0));
        let mut pairs = dist2ids.iter().tuple_windows();
        let (_, ones) = pairs.next().context("The corner is alone?!")?;
        let &[id0, id1] = &ones[..] else {
            bail!("{} cells at distance 1 from the corner", ones.len());
        };
        id2loc.insert(id0, (0, 1));
        id2loc.insert(id1, (1, 0));
        for (old, new) in pairs {
            for id in new {
                let prev_locs = old
                    .iter()
                    .copied()
                    .filter_map(|prev_id| graph[id].contains(&prev_id).then(|| id2loc[&prev_id]))
                    .collect_vec();
                let loc: (usize, usize) = match prev_locs[..] {
                    [(r, 0)] => (r + 1, 0),
                    [(0, c)] => (0, c + 1),
                    [(r, c), (x, y)] if r.abs_diff(x) == 1 && c.abs_diff(y) == 1 => {
                        (r.max(x), c.max(y))
                    }
                    _ => bail!("Wrong previous locations for {}: {:?}", id, prev_locs),
                };
                id2loc.insert(*id, loc);
            }
        }
        let nrows = id2loc
            .values()
            .map(|rc| rc.0)
            .max()
            .context("(0, 0) is at least there")?
            + 1;
        let ncols = id2loc
            .values()
            .map(|rc| rc.1)
            .max()
            .context("(0, 0) is at least there")?
            + 1;
        let mut opt_grid = vec![vec![None; ncols]; nrows];
        for (id, (r, c)) in id2loc {
            let old = opt_grid[r][c].replace(id);
            ensure!(old.is_none(), "Piling tiles on the grid");
        }
        opt_grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|id| id.context("Missing id in the grid"))
                    .collect()
            })
            .collect()
    }

    fn merge_tiles(&mut self) -> Result<Vec<Vec<bool>>> {
        const SIZE: usize = 10;
        let grid_ids = self.grid_ids()?;
        debug_assert!(grid_ids
            .iter()
            .flatten()
            .all(|id| self.id2tile.contains_key(id)));
        let (id_nrows, id_ncols) = (grid_ids.len(), grid_ids[0].len());
        for r in 0..id_nrows {
            for c in 0..id_ncols {
                let id = grid_ids[r][c];
                let neighboring_ids = NEIGHBORS_4.map(|(dr, dc)| {
                    let (r0, c0) = (r.checked_add_signed(dr)?, c.checked_add_signed(dc)?);
                    let id2 = *grid_ids.get(r0)?.get(c0)?;
                    self.common_border
                        .get(&(id, id2))
                        .or_else(|| self.common_border.get(&(id2, id)))
                        .copied()
                });
                self.id2tile
                    .get_mut(&id)
                    .context("Missing tile id")?
                    .satisfy_constraints(neighboring_ids)?;
            }
        }
        let mut big_grid = vec![vec![false; (SIZE - 2) * id_ncols]; (SIZE - 2) * id_nrows];
        for (r, c, i, j) in iproduct!(0..id_nrows, 0..id_ncols, 1..SIZE - 1, 1..SIZE - 1) {
            let id = grid_ids[r][c];
            big_grid[(SIZE - 2) * r + i - 1][(SIZE - 2) * c + j - 1] = self.id2tile[&id].grid[i][j];
        }
        Ok(big_grid)
    }
}

impl std::str::FromStr for TileCollection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let id2tile = s
            .trim_end()
            .split("\n\n")
            .map(|block| {
                let (t, s) = block.split_once('\n').context("Not multiple lines")?;
                let id: u32 = t
                    .strip_prefix("Tile ")
                    .context("Wrong prefix")?
                    .strip_suffix(':')
                    .context("No colon")?
                    .parse()?;
                s.parse::<Tile>().map(|tile| (id, tile))
            })
            .ok_collect_hmap()?;
        let id_borders = id2tile
            .iter()
            .flat_map(|(id, tile)| tile.borders.map(|b| (*id, b)))
            .collect_vec();
        let common_border = id_borders
            .iter()
            .tuple_combinations()
            .filter_map(|(&(i0, b0), &(i1, b1))| (b0 == b1).then_some(((i0, i1), b0)))
            .collect();
        Ok(Self {
            id2tile,
            common_border,
        })
    }
}

impl std::str::FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let grid = parse_to_grid(s.lines(), |ch| match ch {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => bail!("Wrong char: {}", ch),
        })?;
        ensure!(grid.len() == 10, "Not 10 rows");
        ensure!(grid[0].len() == 10, "Not 10 columns");
        let col0 = grid.iter().map(|row| row[0]).collect_vec();
        let col9 = grid.iter().map(|row| row[9]).collect_vec();
        let borders = [
            Border::from_line(&grid[0]),
            Border::from_line(&grid[9]),
            Border::from_line(&col0),
            Border::from_line(&col9),
        ];
        Ok(Self { grid, borders })
    }
}

pub const INPUTS: [&str; 2] = [
    "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
",
    include_str!("input.txt"),
];

#[test]
fn solver_20_20() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "20899048083289"); // 1951 * 3079 * 2971 * 1171
    assert_eq!(solver(Part1, INPUTS[1])?, "21599955909991"); // 1061 * 2521 * 2633 * 3067
    assert_eq!(solver(Part2, INPUTS[0])?, "273");
    assert_eq!(solver(Part2, INPUTS[1])?, "2495");
    Ok(())
}
