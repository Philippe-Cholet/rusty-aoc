use ndarray::{s, Array2};

use common::prelude::*;
use utils::OkIterator;

#[derive(Debug)]
enum Instruction {
    TurnOn(bool),
    Toggle,
}

impl Instruction {
    fn update_light(&self, is_on: &mut bool) {
        *is_on = match self {
            Self::TurnOn(on) => *on,
            Self::Toggle => !*is_on,
        };
    }

    fn update_brightness(&self, brightness: &mut u8) {
        match self {
            Self::TurnOn(true) => *brightness += 1,
            Self::TurnOn(false) => *brightness = brightness.saturating_sub(1),
            Self::Toggle => *brightness += 2,
        };
    }
}

/// Probably a Fire Hazard
pub fn solver(part: Part, input: &str) -> Result<String> {
    let changes = input
        .lines()
        .map(|line| {
            let items: Vec<_> = line.rsplitn(4, ' ').collect();
            ensure!(items.len() == 4 && items[1] == "through", "{:?}", items);
            let (x0, y0) = items[2].split_once(',').context("No comma")?;
            let (x1, y1) = items[0].split_once(',').context("No comma")?;
            let [x0, y0, x1, y1]: [usize; 4] = [x0.parse()?, y0.parse()?, x1.parse()?, y1.parse()?];
            Ok((items[3].parse::<Instruction>()?, s![x0..=x1, y0..=y1]))
        })
        .ok_collect_vec()?;
    Ok(match part {
        Part1 => {
            let mut grid = Array2::<bool>::default((1000, 1000));
            for (instruction, slice) in changes {
                grid.slice_mut(slice)
                    .map_mut(|on| instruction.update_light(on));
            }
            grid.into_raw_vec().into_iter().filter(|&on| on).count()
        }
        Part2 => {
            let mut grid = Array2::<u8>::zeros((1000, 1000));
            for (instruction, slice) in changes {
                grid.slice_mut(slice)
                    .map_mut(|brightness| instruction.update_brightness(brightness));
            }
            grid.into_raw_vec().into_iter().map(usize::from).sum()
        }
    }
    .to_string())
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "turn on" => Self::TurnOn(true),
            "turn off" => Self::TurnOn(false),
            "toggle" => Self::Toggle,
            _ => bail!("Wrong instruction: {}", s),
        })
    }
}

pub const INPUTS: [&str; 1] = [include_str!("input.txt")];

#[test]
fn solver_15_06() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "400410");
    assert_eq!(solver(Part2, INPUTS[0])?, "15343601");
    Ok(())
}
