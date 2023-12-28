use itertools::Itertools;

use common::prelude::*;
use crate::utils::OkIterator;

#[derive(Debug)]
struct SandBlock([u32; 3], [u32; 3]);

impl SandBlock {
    fn new(start: [u32; 3], end: [u32; 3]) -> Result<Self> {
        start
            .iter()
            .zip(&end)
            .all(|(s, e)| s <= e)
            .then_some(Self(start, end))
            .context("Wrong block")
    }

    const fn height(&self) -> u32 {
        self.0[2]
    }

    const fn min_z_above(&self) -> u32 {
        self.1[2] + 1
    }

    fn fall_to(&mut self, z: u32) {
        debug_assert!(z <= self.0[2]);
        self.1[2] -= self.0[2] - z;
        self.0[2] = z;
    }

    const fn xy_intersect(&self, other: &Self) -> bool {
        !(self.1[0] < other.0[0]
            || other.1[0] < self.0[0]
            || self.1[1] < other.0[1]
            || other.1[1] < self.0[1])
    }

    const fn is_on_top_of(&self, other: &Self) -> bool {
        self.height() == other.min_z_above() && self.xy_intersect(other)
    }
}

/// Sand Slabs
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let mut blocks: Vec<_> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('~').context("No ~")?;
            SandBlock::new(
                start.split(',').map(str::parse).ok_collect_array()?,
                end.split(',').map(str::parse).ok_collect_array()?,
            )
        })
        .try_collect()?;
    let nb_blocks = blocks.len();
    // To process the blocks in the order of their distance to the ground.
    blocks.sort_by_key(SandBlock::height);
    for i in 0..nb_blocks {
        let z = blocks[..i]
            .iter()
            .filter(|old| blocks[i].xy_intersect(old))
            .map(SandBlock::min_z_above)
            .max()
            .unwrap_or(1);
        blocks[i].fall_to(z);
    }
    Ok(match part {
        Part1 => {
            let mut can_disintegrate = vec![true; nb_blocks];
            for (i, block) in blocks.iter().enumerate() {
                if let Ok(idx) = (0..i)
                    .filter(|j| block.is_on_top_of(&blocks[*j]))
                    .exactly_one()
                {
                    // Disintegrate this one would make `block` fall.
                    can_disintegrate[idx] = false;
                }
            }
            can_disintegrate.iter().filter(|b| **b).count()
        }
        Part2 => {
            let mut below = vec![vec![]; nb_blocks];
            let mut above = vec![vec![]; nb_blocks];
            blocks.iter().enumerate().tuple_combinations().for_each(
                |((i0, block0), (i1, block1))| {
                    if block1.is_on_top_of(block0) {
                        below[i1].push(i0);
                        above[i0].push(i1);
                    }
                },
            );
            // Remove the duplicates.
            for indexes in below.iter_mut().chain(above.iter_mut()) {
                indexes.sort_unstable();
                indexes.dedup();
            }
            let mut fallen = vec![false; nb_blocks];
            let mut stack = vec![];
            (0..nb_blocks)
                .map(|idx| {
                    debug_assert!(fallen.iter().all(|f| !f));
                    debug_assert!(stack.is_empty());
                    stack.push(idx);
                    while let Some(i) = stack.pop() {
                        if fallen[i] {
                            continue;
                        }
                        fallen[i] = true;
                        for &a in &above[i] {
                            if below[a].iter().all(|&b| fallen[b]) {
                                stack.push(a);
                            }
                        }
                    }
                    // Count fallen blocks and re-initialize `fallen`.
                    let count = fallen.iter_mut().filter(|f| **f).fold(0, |acc, f| {
                        *f = false;
                        acc + 1
                    });
                    count - 1 // Do not count the disintegrated block.
                })
                .sum()
        }
    })
}

test_solver! {
    "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
" => (5, 7),
    include_input!(23 22) => (441, 80778),
}
