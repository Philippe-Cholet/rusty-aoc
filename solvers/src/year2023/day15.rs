use common::prelude::*;

fn hash(s: &str) -> u8 {
    s.bytes()
        .fold(0, |acc, b| acc.wrapping_add(b).wrapping_mul(17))
}

/// Lens Library
pub fn solver(part: Part, input: &str) -> Result<u32> {
    let parts = input.trim_end().split(',');
    Ok(match part {
        Part1 => parts.map(hash).map(u32::from).sum::<u32>(),
        Part2 => {
            let mut boxes: [Vec<(&str, u8)>; 256] = core::array::from_fn(|_| Vec::new());
            for cmd in parts {
                if let Some(label) = cmd.strip_suffix('-') {
                    boxes[hash(label) as usize].retain(|(s, _)| s != &label);
                } else {
                    let (label, focal) = cmd.split_once('=').context("Wrong command")?;
                    let focal = focal.parse()?;
                    ensure!(matches!(focal, 1..=9));
                    let lenses = &mut boxes[hash(label) as usize];
                    if let Some((_, old_focal)) = lenses.iter_mut().find(|(s, _)| s == &label) {
                        *old_focal = focal;
                    } else {
                        lenses.push((label, focal));
                    }
                }
            }
            // println!("{boxes:?}");
            #[allow(clippy::cast_possible_truncation)]
            boxes
                .iter()
                .enumerate()
                .flat_map(|(idx, v)| {
                    v.iter().enumerate().map(move |(i, (_, focal))| {
                        u32::from(*focal) * (i as u32 + 1) * (idx as u32 + 1)
                    })
                })
                .sum()
        }
    })
}

#[test]
#[ignore]
fn hash_hash() {
    assert_eq!(hash("HASH"), 52);
}

test_solver! {
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7" => (1320, 145),
    include_input!(23 15) => (506869, 271384),
}
