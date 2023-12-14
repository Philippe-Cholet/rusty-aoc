use std::{
    collections::{BinaryHeap, VecDeque},
    fmt,
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::{iproduct, Either, Itertools};

use common::prelude::*;
use utils::HeuristicItem;

use Amphipod::{Amber, Bronze, Copper, Desert};
use Loc::{Hallway, Room, RoomEntrance};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

// Hallway and Room are used to nicely index states.
#[derive(Debug, Clone, Copy)]
enum Loc {
    Hallway(usize),
    RoomEntrance(Amphipod),
    Room(Amphipod, usize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State<const N: usize> {
    hallway: [Option<Amphipod>; 7],
    rooms: [[Option<Amphipod>; N]; 4],
}

impl<const N: usize> std::hash::Hash for State<N> {
    #[allow(clippy::cast_possible_truncation)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        debug_assert!(N <= 10, "u32 does not have enough bits for this room size");
        let mut hash_amps = |slice: &[Option<Amphipod>]| {
            // 1 + 3 * slice.len() <= 32  (the biggest slice being the hallway: 7 long)
            let mut n: u32 = 1;
            for opt_amp in slice {
                n <<= 3;
                if let Some(amp) = opt_amp {
                    n |= amp.room_id() as u32;
                }
            }
            n.hash(state);
        };
        hash_amps(&self.hallway);
        for room in &self.rooms {
            hash_amps(room);
        }
    }
}

// (Precomputed) paths and distances between all locations.
type AmphipodMap = Vec<Vec<(Vec<Loc>, u32)>>;

/// Amphipod
pub fn solver(part: Part, input: &str) -> Result<u32> {
    Ok(match part {
        Part1 => input.parse::<State<2>>()?.minimize_energy()?,
        Part2 => {
            let mut lines: Vec<_> = input.lines().collect();
            lines.insert(3, "  #D#C#B#A#");
            lines.insert(4, "  #D#B#A#C#");
            let new_input = lines.join("\n");
            new_input.parse::<State<4>>()?.minimize_energy()?
        }
    })
}

impl Amphipod {
    const ALL: [Self; 4] = [Amber, Bronze, Copper, Desert];

    const fn energy(self) -> u32 {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }

    const fn room_id(self) -> usize {
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
        }
    }
}

impl Loc {
    const fn prev_in_room(self) -> Option<Self> {
        match self {
            Hallway(_) | RoomEntrance(_) | Room(_, 0) => None,
            Room(amp, i) => Some(Room(amp, i - 1)),
        }
    }

    fn valid_move(self, other: Self) -> bool {
        match (self, other) {
            (Room(a1, _), Room(a2, _)) => a1 != a2,
            (Room(_, _), Hallway(_)) | (Hallway(_), Room(_, _)) => true,
            _ => false,
        }
    }

    fn neighbors(self, room_size: usize) -> Vec<Self> {
        match self {
            Hallway(0) => vec![Hallway(1)],
            Hallway(1) => vec![Hallway(0), RoomEntrance(Amber)],
            Hallway(2) => [Amber, Bronze].map(RoomEntrance).to_vec(),
            Hallway(3) => [Bronze, Copper].map(RoomEntrance).to_vec(),
            Hallway(4) => [Copper, Desert].map(RoomEntrance).to_vec(),
            Hallway(5) => vec![RoomEntrance(Desert), Hallway(6)],
            Hallway(6) => vec![Hallway(5)],
            RoomEntrance(amphipod) => vec![
                Hallway(amphipod.room_id() + 1),
                Hallway(amphipod.room_id() + 2),
                Room(amphipod, 0),
            ],
            Room(amp, index) if index < room_size => {
                let mut res = vec![if index == 0 {
                    RoomEntrance(amp)
                } else {
                    Room(amp, index - 1)
                }];
                if index + 1 < room_size {
                    res.push(Room(amp, index + 1));
                }
                res
            }
            _ => unreachable!(),
        }
    }

    // 0..7    hallways ;
    // 7..11   room entrances ;
    // 11..15  rooms first places ;
    // 15..19  rooms second places ;
    // ...
    const fn as_usize(self) -> usize {
        match self {
            Hallway(i) => i,
            RoomEntrance(amp) => 7 + amp.room_id(),
            Room(amp, i) => 11 + (4 * i + amp.room_id()),
        }
    }

    const fn from_usize(index: usize) -> Self {
        match index {
            0..=6 => Hallway(index),
            7..=10 => RoomEntrance(Amphipod::ALL[index - 7]),
            _ => Room(Amphipod::ALL[(index - 11) % 4], (index - 11) / 4),
        }
    }
}

impl<const N: usize> State<N> {
    const GOAL: Self = Self {
        hallway: [None; 7],
        rooms: [
            [Some(Amber); N],
            [Some(Bronze); N],
            [Some(Copper); N],
            [Some(Desert); N],
        ],
    };

    #[allow(clippy::cast_possible_truncation)] // Path lengths are really small!
    fn amphipod_map() -> Result<AmphipodMap> {
        let size = 7 + 4 * (1 + N);
        (0..size)
            .map(|start| {
                let mut res = vec![None; size];
                let mut queue = VecDeque::from([(vec![], start)]);
                while let Some((path, u)) = queue.pop_front() {
                    if res[u].is_some() {
                        continue;
                    }
                    let mut new_path = path.clone();
                    new_path.pop(); // Do not consider the end of it.
                    new_path.retain(|loc| !matches!(loc, RoomEntrance(_))); // Ignore (empty) entrances.
                    res[u] = Some((new_path, path.len() as u32));
                    for neighbor in Loc::from_usize(u).neighbors(N) {
                        let v = neighbor.as_usize();
                        if res[v].is_some() {
                            continue;
                        }
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        queue.push_back((new_path, v));
                    }
                }
                res.into_iter()
                    .map(|opt| opt.context("Missing loc"))
                    .collect()
            })
            .collect()
    }

    fn energy_to_goal_lower_bound(&self, amphipod_map: &AmphipodMap) -> u32 {
        let mut wrong_locs = Vec::with_capacity(4 * N * 2 + 7);
        for (owner, index) in iproduct!(Amphipod::ALL, 0..N) {
            let loc = Room(owner, index);
            match self[loc] {
                Some(amp) if amp != owner => {
                    wrong_locs.push((amp, loc));
                    wrong_locs.push((owner, loc));
                }
                None => wrong_locs.push((owner, loc)),
                Some(_) => {}
            }
        }
        wrong_locs.extend(
            (0..7)
                .map(Hallway)
                .filter_map(|start| self[start].map(|amp| (amp, start))),
        );
        wrong_locs
            .into_iter()
            .map(|(amp, loc)| {
                amphipod_map[loc.as_usize()][RoomEntrance(amp).as_usize()].1 * amp.energy()
            })
            .sum()
    }

    #[allow(clippy::type_complexity)]
    fn possible_endpoints(&self) -> (Vec<(Loc, Amphipod)>, Vec<(Loc, Option<Amphipod>)>) {
        let (mut starts, mut ends): (Vec<_>, Vec<_>) = (0..7).map(Hallway).partition_map(|loc| {
            self[loc].map_or(Either::Right((loc, None)), |amp| Either::Left((loc, amp)))
        });
        for owner in Amphipod::ALL {
            let mut locs = (0..N).filter_map(|index| {
                let loc = Room(owner, index);
                self[loc].map(|amphipod| (loc, amphipod))
            });
            match locs.next() {
                // Nobody in this room, yet. Place an owner at the bottom of its room.
                None => ends.push((Room(owner, N - 1), Some(owner))),
                Some((start, moving_amp)) => {
                    if moving_amp != owner || locs.any(|(_, amp)| amp != owner) {
                        // Amphipod candidate to move.
                        starts.push((start, moving_amp));
                    } else if let Some(end) = start.prev_in_room() {
                        // Amphipods here are rightfully placed, but not full yet.
                        ends.push((end, Some(owner)));
                    }
                    // else the room is full of its owners.
                }
            }
        }
        (starts, ends)
    }

    fn neighbors(&self, amphipod_map: &AmphipodMap) -> Result<Vec<(Self, u32)>> {
        let (starts, ends) = self.possible_endpoints();
        let possible_moves = iproduct!(starts.iter(), ends.iter())
            .filter_map(|((start, a1), (end, a2))| {
                (a2.is_none() || a2.as_ref() == Some(a1)).then_some((*start, *end))
            })
            .filter(|(start, end)| start.valid_move(*end));
        let mut res = vec![];
        for (start, end) in possible_moves {
            let (path, distance) = &amphipod_map[start.as_usize()][end.as_usize()];
            if path.iter().all(|loc| self[*loc].is_none()) {
                // The path is clear between the start and the end.
                let mut new = self.clone();
                let amp = new[start].take().context("Taking from an empty place")?;
                let old = new[end].replace(amp);
                ensure!(old.is_none(), "Piling amphipods");
                let candidate = (new, amp.energy() * distance);
                if matches!((start, end), (Hallway(_), Room(_, _))) {
                    // Some amphipod can be ranged in its room, prioritize that!
                    return Ok(vec![candidate]);
                }
                res.push(candidate);
            }
        }
        Ok(res)
    }

    fn minimize_energy(self) -> Result<u32> {
        let amphipod_map = Self::amphipod_map()?;
        // for row in &amphipod_map {
        //     for (_, distance) in row {
        //         print!("  {distance: >2}");
        //     }
        //     println!();
        // }
        // This first heuristic is inaccurate but unused.
        let mut heap = BinaryHeap::from([HeuristicItem::rev(0, (self, 0))]);
        let mut been = HashSet::new();
        Ok(loop {
            let (state, energy_so_far) = heap.pop().context("Failed to solve!")?.item;
            if state == Self::GOAL {
                break energy_so_far;
            }
            if been.contains(&state) {
                continue;
            }
            let neighbors = state.neighbors(&amphipod_map)?;
            been.insert(state);
            for (neighbor, energy_to_neighbor) in neighbors {
                if been.contains(&neighbor) {
                    continue;
                }
                heap.push(HeuristicItem::rev(
                    energy_so_far
                        + energy_to_neighbor
                        + neighbor.energy_to_goal_lower_bound(&amphipod_map),
                    (neighbor, energy_so_far + energy_to_neighbor),
                ));
            }
        })
    }
}

impl<const N: usize> Index<Loc> for State<N> {
    type Output = Option<Amphipod>;

    fn index(&self, loc: Loc) -> &Self::Output {
        match loc {
            RoomEntrance(_) => panic!("Room entrances are empty"),
            Hallway(index) => &self.hallway[index],
            Room(amphipod, index) => &self.rooms[amphipod.room_id()][index],
        }
    }
}

impl<const N: usize> IndexMut<Loc> for State<N> {
    fn index_mut(&mut self, loc: Loc) -> &mut Self::Output {
        match loc {
            RoomEntrance(_) => panic!("Room entrances are empty"),
            Hallway(index) => &mut self.hallway[index],
            Room(amphipod, index) => &mut self.rooms[amphipod.room_id()][index],
        }
    }
}

impl FromStr for Amphipod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "A" => Amber,
            "B" => Bronze,
            "C" => Copper,
            "D" => Desert,
            _ => bail!("Wrong amphipod: {}", s),
        })
    }
}

impl<const N: usize> FromStr for State<N> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut lines = s.lines();
        ensure!(lines.next() == Some("#############"), "First walls");
        ensure!(lines.next() == Some("#...........#"), "Second hallway");
        let mut rooms = [[None; N]; 4];
        for j in 0..N {
            let line = lines.next().context("Missing line")?;
            if j == 0 {
                ensure!(line.len() == 13, "Line 0");
                ensure!(
                    &line[..2] == "##" && &line[11..] == "##",
                    "Start/end with ##"
                );
            } else {
                ensure!(line.len() == 11, "Line 1..N");
                ensure!(&line[..2] == "  ", "Start with two spaces");
            }
            for w in [2, 4, 6, 8, 10] {
                ensure!(&line[w..=w] == "#", "Walls");
            }
            for (i, w) in (0..4).zip([3, 5, 7, 9]) {
                rooms[i][j] = Some(line[w..=w].parse()?);
            }
        }
        ensure!(lines.next() == Some("  #########"), "Last walls");
        ensure!(lines.next().is_none(), "Nothing after last walls");
        Ok(Self {
            hallway: [None; 7],
            rooms,
        })
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Amber => 'A',
            Bronze => 'B',
            Copper => 'C',
            Desert => 'D',
        };
        write!(f, "{ch}")
    }
}

impl<const N: usize> fmt::Display for State<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! opamp {
            ($opt_amp:expr) => {
                match $opt_amp {
                    None => write!(f, ".")?,
                    Some(amp) => amp.fmt(f)?,
                }
            };
        }
        writeln!(f, "#############")?;
        write!(f, "#")?;
        opamp!(&self.hallway[0]);
        opamp!(&self.hallway[1]);
        write!(f, "+")?;
        opamp!(&self.hallway[2]);
        write!(f, "+")?;
        opamp!(&self.hallway[3]);
        write!(f, "+")?;
        opamp!(&self.hallway[4]);
        write!(f, "+")?;
        opamp!(&self.hallway[5]);
        opamp!(&self.hallway[6]);
        writeln!(f, "#")?;
        for i in 0..N {
            if i == 0 {
                write!(f, "##")?;
            } else {
                write!(f, "  ")?;
            };
            for j in 0..4 {
                write!(f, "#")?;
                opamp!(&self.rooms[j][i]);
            }
            if i == 0 {
                writeln!(f, "###")?;
            } else {
                writeln!(f, "#")?;
            };
        }
        write!(f, "  #########")
    }
}

pub const INPUTS: [&str; 2] = [
    "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
",
    include_input!(21 23),
];

#[test]
fn solver_21_23() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 12521);
    assert_eq!(solver(Part1, INPUTS[1])?, 16157);
    assert_eq!(solver(Part2, INPUTS[0])?, 44169);
    assert_eq!(solver(Part2, INPUTS[1])?, 43481);
    Ok(())
}
