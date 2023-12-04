use itertools::{iproduct, Itertools};

use common::{prelude::*, Ok};
use utils::OkIterator;

#[derive(Debug, Clone)]
struct Unit {
    hit_points: u16,
    damage: u16,
    armor: u16,
}

#[derive(Debug, Default, Clone)]
struct Item {
    cost: u16,
    damage: u16,
    armor: u16,
}

/// RPG Simulator 20XX
pub fn solver(part: Part, input: &str) -> Result<u16> {
    let boss: Unit = input.parse()?;
    match part {
        Part1 => Item::shopping_possibilities()
            .sorted_by_key(|items| items.cost)
            .find_map(|items| {
                let mut player = Unit::default();
                player.equip(&items);
                player.fight(&mut boss.clone()).then_some(items.cost)
            })
            .context("Can't win against the boss, even well armed"),
        Part2 => Item::shopping_possibilities()
            .sorted_by_key(|items| items.cost)
            .rev()
            .find_map(|items| {
                let mut player = Unit::default();
                player.equip(&items);
                (!player.fight(&mut boss.clone())).then_some(items.cost)
            })
            .context("Can't lose against the boss, even badly armed"),
    }
}

impl Item {
    const fn new(cost: u16, damage: u16, armor: u16) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }

    const WEAPONS: [Self; 5] = [
        Self::new(8, 4, 0),
        Self::new(10, 5, 0),
        Self::new(25, 6, 0),
        Self::new(40, 7, 0),
        Self::new(74, 8, 0),
    ];
    const ARMORS: [Self; 5] = [
        Self::new(13, 0, 1),
        Self::new(31, 0, 2),
        Self::new(53, 0, 3),
        Self::new(75, 0, 4),
        Self::new(102, 0, 5),
    ];
    const RINGS: [Self; 6] = [
        Self::new(25, 1, 0),
        Self::new(50, 2, 0),
        Self::new(100, 3, 0),
        Self::new(20, 0, 1),
        Self::new(40, 0, 2),
        Self::new(80, 0, 3),
    ];

    fn shopping_possibilities() -> impl Iterator<Item = Self> {
        iproduct!(0..=1, 0..=2).flat_map(|(a, r)| {
            iproduct!(
                Self::WEAPONS,
                Self::ARMORS.into_iter().combinations(a),
                Self::RINGS.into_iter().combinations(r)
            )
            .map(|(weapon, armors, rings)| {
                weapon + armors.into_iter().sum() + rings.into_iter().sum()
            })
        })
    }
}

impl Unit {
    const fn is_alive(&self) -> bool {
        self.hit_points != 0
    }

    fn equip(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armor += item.armor;
    }

    fn take_hit(&mut self, damage: u16) {
        let real_damage = damage.saturating_sub(self.armor).max(1);
        self.hit_points = self.hit_points.saturating_sub(real_damage);
    }

    fn fight(&mut self, other: &mut Self) -> bool {
        while self.is_alive() && other.is_alive() {
            other.take_hit(self.damage);
            if other.is_alive() {
                self.take_hit(other.damage);
            }
        }
        self.is_alive()
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            hit_points: 100,
            damage: 0,
            armor: 0,
        }
    }
}

impl std::str::FromStr for Unit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let [hit_points, damage, armor] = s
            .lines()
            .zip(["Hit Points", "Damage", "Armor"])
            .map(|(line, s)| {
                Ok(line
                    .strip_prefix(&format!("{s}: "))
                    .context("Wrong prefix")?
                    .parse::<u16>()?)
            })
            .ok_collect_array()?;
        Ok(Self {
            hit_points,
            damage,
            armor,
        })
    }
}

impl std::ops::Add for Item {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cost: self.cost + other.cost,
            damage: self.damage + other.damage,
            armor: self.armor + other.armor,
        }
    }
}

impl std::iter::Sum for Item {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(std::ops::Add::add).unwrap_or_default()
    }
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_15_21() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 91);
    assert_eq!(solver(Part2, INPUTS[0])?, 158);
    Ok(())
}
