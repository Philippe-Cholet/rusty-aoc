// I would simplify things if I did not already spent too much time on it
// (mostly because of a stupid error in `neighbors` method of `State`).
#![allow(clippy::expect_used)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use itertools::{iproduct, Itertools};

use common::{bail, Context, Part, Part1, Part2, Result};
use utils::{neighbors, FromIterStr};

type Location = (usize, usize);

const fn manhattan(a: Location, b: Location) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    const fn energy(self) -> usize {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Cell {
    Wall,
    Hallway { entry: bool },
    Room { target: Amphipod },
}

impl Cell {
    fn is_room(&self, amphipod: Amphipod) -> bool {
        matches!(self, Self::Room { target } if target == &amphipod)
    }
}

#[derive(Debug)]
struct Grid {
    inner: Vec<Vec<Cell>>,
    rooms: HashMap<Amphipod, Vec<Location>>,
    distances: HashMap<(Location, Location), usize>,
}

impl Grid {
    fn new(grid: Vec<Vec<Cell>>, amphipods_per_room: usize) -> Self {
        let ncols = grid.iter().map(Vec::len).max().expect("Empty grid");
        let grid = {
            let mut rect = grid;
            // Ensure the grid is rectangular.
            for row in &mut rect {
                for _ in 0..ncols - row.len() {
                    row.push(Cell::Wall);
                }
            }
            rect
        };
        let rooms = HashMap::from([
            (
                Amphipod::Amber,
                (0..amphipods_per_room).map(|r| (r + 2, 3)).collect_vec(),
            ),
            (
                Amphipod::Bronze,
                (0..amphipods_per_room).map(|r| (r + 2, 5)).collect_vec(),
            ),
            (
                Amphipod::Copper,
                (0..amphipods_per_room).map(|r| (r + 2, 7)).collect_vec(),
            ),
            (
                Amphipod::Desert,
                (0..amphipods_per_room).map(|r| (r + 2, 9)).collect_vec(),
            ),
        ]);
        let cells = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(c, cell)| match cell {
                        Cell::Wall => None,
                        _ => Some(((r, c), cell)),
                    })
            })
            .collect_vec();
        let distances = iproduct!(cells.iter(), cells.iter())
            .map(|((loc1, cell1), (loc2, cell2))| {
                let dist = match (cell1, cell2) {
                    (Cell::Room { target: a }, Cell::Room { target: b }) if a != b => {
                        let (r1, c1) = loc1;
                        let (r2, c2) = loc2;
                        r1 - 1 + c1.abs_diff(*c2) + r2 - 1
                        // Move up, move left or right, move down
                    }
                    _ => manhattan(*loc1, *loc2),
                };
                ((*loc1, *loc2), dist)
            })
            .collect();
        Self {
            inner: grid,
            rooms,
            distances,
        }
    }

    fn shape(&self) -> Location {
        (self.inner.len(), self.inner[0].len())
    }

    fn get(&self, loc: &Location) -> &Cell {
        &self.inner[loc.0][loc.1]
    }
}

#[derive(Debug, Clone)]
struct State {
    places: HashMap<Location, Amphipod>,
    energy: usize,
    heuristic: usize, // energy + "estimation of the distance to the goal"
}

// Note the use of reverse in below implementations, to make the heap a min-heap.
// Only consider heuristic to order those states.
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.heuristic.eq(&other.heuristic)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.heuristic.partial_cmp(&other.heuristic)?.reverse())
    }
}
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
    }
}

impl State {
    fn energy_lower_bound(&self, grid: &Grid) -> usize {
        let mut amp2locs = HashMap::new();
        for (loc, amphi) in &self.places {
            amp2locs.entry(amphi).or_insert_with(Vec::new).push(*loc);
        }
        // If an amphipod could go through an occupied cell, the solution would be
        // obvious, the energy it would take is an obvious lower bound.
        let mut total = 0;
        for (amphi, room) in &grid.rooms {
            let locs = amp2locs.get(amphi).expect("Missing amphipod?!");
            assert_eq!(room.len(), locs.len());
            let min_dist: usize = locs
                .iter()
                .permutations(locs.len())
                .map(|v| {
                    room.iter()
                        .zip(v.into_iter())
                        .map(|(a, b)| grid.distances.get(&(*a, *b)).expect("Missing distance?!"))
                        .sum()
                })
                .min()
                .expect("Missing amphipod?!");
            total += min_dist * amphi.energy();
        }
        total
    }

    fn moving(&self, src: Location, dst: Location, nb_moves: usize, grid: &Grid) -> Self {
        let mut new = self.clone();
        let amphipod = new
            .places
            .remove(&src)
            .expect("Can not move from an empty place");
        new.energy += amphipod.energy() * nb_moves;
        assert!(!new.places.contains_key(&dst));
        new.places.insert(dst, amphipod);
        new.heuristic = new.energy + new.energy_lower_bound(grid);
        new
    }

    fn neighbors(&self, grid: &Grid) -> Vec<Self> {
        let (nrows, ncols) = grid.shape();
        let mut result = vec![];
        for (src, amphipod) in &self.places {
            let in_my_room = grid.get(src).is_room(*amphipod);
            let my_room_has_no_stranger = grid
                .rooms
                .get(amphipod)
                .expect("Missing room?!")
                .iter()
                .all(|loc| self.places.get(loc).map_or(true, |amp| amp == amphipod));
            let last_free_place_in_my_room = grid.rooms[amphipod]
                .iter()
                .filter(|loc| !self.places.contains_key(*loc))
                .max_by_key(|loc| loc.0);
            let can_move_to_hallway = match grid.get(src) {
                Cell::Wall => unreachable!("You are in a wall really?!"),
                Cell::Hallway { .. } => false,
                Cell::Room { target } if target == amphipod => !my_room_has_no_stranger,
                Cell::Room { .. } => true,
            };
            let mut queue = VecDeque::from([(0, *src)]);
            let mut been = HashSet::new();
            while let Some((nb_moves, loc)) = queue.pop_front() {
                if been.contains(&loc) {
                    continue; // already visited
                }
                if &loc != src && self.places.contains_key(&loc) {
                    continue; // another amphipod occupies this location.
                }
                been.insert(loc);
                for loc2 in neighbors(loc.0, loc.1, nrows, ncols, false) {
                    if been.contains(&loc2) {
                        continue; // already visited
                    }
                    if self.places.contains_key(&loc2) {
                        continue; // already occupied
                    }
                    let can_stop_at = match &grid.get(&loc2) {
                        Cell::Wall => continue,
                        Cell::Hallway { entry } => !entry && can_move_to_hallway,
                        // Stop if it is my room, if I am not already in it,
                        // if it has no stranger and if I would not block the way in my room.
                        Cell::Room { target } => {
                            target == amphipod
                                && !in_my_room
                                && my_room_has_no_stranger
                                && last_free_place_in_my_room == Some(&loc2)
                        }
                    };
                    if can_stop_at {
                        result.push(self.moving(*src, loc2, nb_moves + 1, grid));
                    }
                    queue.push_back((nb_moves + 1, loc2));
                }
            }
        }
        result
    }

    fn is_organized(&self, grid: &Grid) -> bool {
        self.places
            .iter()
            .all(|(loc, amphipod)| grid.get(loc).is_room(*amphipod))
    }
}

/// Amphipod
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut lines = input.lines().collect_vec();
    let amphipods_per_room = match part {
        Part1 => 2,
        Part2 => {
            lines.insert(3, "  #D#C#B#A#");
            lines.insert(4, "  #D#B#A#C#");
            4
        }
    };
    let column2amphipod = |c| match c {
        3 => Some(Amphipod::Amber),
        5 => Some(Amphipod::Bronze),
        7 => Some(Amphipod::Copper),
        9 => Some(Amphipod::Desert),
        _ => None,
    };
    let mut init_places = HashMap::new();
    let grid = lines.into_iter().parse_to_grid_with_loc(|(r, c), ch| {
        Ok(match ch {
            '#' | ' ' => Cell::Wall,
            '.' => Cell::Hallway {
                entry: column2amphipod(c).is_some(),
            },
            'A' | 'B' | 'C' | 'D' => {
                let amphipod = match ch {
                    'A' => Amphipod::Amber,
                    'B' => Amphipod::Bronze,
                    'C' => Amphipod::Copper,
                    'D' => Amphipod::Desert,
                    _ => bail!("Not an amphipod: {}", ch),
                };
                init_places.insert((r, c), amphipod);
                Cell::Room {
                    target: column2amphipod(c).context("Amphipod in hallway")?,
                }
            }
            _ => bail!("Wrong char: {}", ch),
        })
    })?;
    let grid = Grid::new(grid, amphipods_per_room);
    let mut been = HashSet::from([AmphipodLocations::from(&init_places)]);
    let mut pqueue = BinaryHeap::from([State {
        energy: 0,
        heuristic: 0,
        places: init_places,
    }]);
    Ok(loop {
        let state = pqueue.pop().context("Failed to find a way")?;
        if state.is_organized(&grid) {
            break state.energy;
        }
        for next in state.neighbors(&grid) {
            let locs = AmphipodLocations::from(&next.places);
            if !been.contains(&locs) {
                been.insert(locs);
                pqueue.push(next);
            }
        }
    }
    .to_string())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct AmphipodLocations(Vec<(Location, Amphipod)>);

impl AmphipodLocations {
    fn from(places: &HashMap<Location, Amphipod>) -> Self {
        let mut locs = places.iter().map(|(&loc, &amp)| (loc, amp)).collect_vec();
        locs.sort_by(|(a, _), (b, _)| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        Self(locs)
    }
}

pub const INPUTS: [&str; 2] = [
    "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_23() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "12521");
    assert_eq!(solver(Part1, INPUTS[1])?, "16157");
    assert_eq!(solver(Part2, INPUTS[0])?, "44169");
    assert_eq!(solver(Part2, INPUTS[1])?, "43481");
    Ok(())
}
