use itertools::Itertools;

use common::prelude::*;

// I thought I would need more than the first position.
fn find_z_positions<'a, F: Fn(usize) -> bool + 'a>(
    mut node: usize,
    is_z: F,
    lr_instructions: &'a [bool],
    network: &'a [[usize; 2]],
) -> impl Iterator<Item = usize> + 'a {
    lr_instructions
        .iter()
        .cycle()
        .positions(move |&lr| {
            node = network[node][usize::from(lr)];
            is_z(node)
        })
        .map(|pos| pos + 1)
}

/// Haunted Wasteland
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let (lr_instructions, network) = input.split_once("\n\n").context("No empty line")?;
    let lr_instructions: Vec<_> = lr_instructions
        .chars()
        .map(|ch| match ch {
            'L' => Ok(false),
            'R' => Ok(true),
            _ => bail!("Wrong LR instruction: {:?}", ch),
        })
        .try_collect()?;
    let network: Vec<_> = network
        .lines()
        .map(|line| {
            ensure!(
                line.len() == 16
                    && &line[3..7] == " = ("
                    && &line[10..12] == ", "
                    && &line[15..] == ")"
            );
            Ok((&line[..3], [&line[7..10], &line[12..15]]))
        })
        .try_collect()?;
    // Replace `&str` nodes by their line index.
    let nodes = network.iter().map(|(node, _)| *node).collect_vec();
    let node_index = |node| nodes.iter().position(|s| s == node);
    let network: Vec<_> = network
        .iter()
        .map(|(_, [left, right])| {
            let left = node_index(left).context("left")?;
            node_index(right)
                .context("right")
                .map(|right| [left, right])
        })
        .try_collect()?;
    #[allow(clippy::expect_used)]
    Ok(match part {
        Part1 => {
            let start = node_index(&"AAA").context("AAA")?;
            let end = node_index(&"ZZZ").context("ZZZ")?;
            find_z_positions(start, move |n| n == end, &lr_instructions, &network)
                .next()
                .expect("Endless loop")
        }
        Part2 => {
            let starts = nodes.iter().positions(|s| &s[2..] == "A").collect_vec();
            let ends = nodes.iter().positions(|s| &s[2..] == "Z").collect_vec();
            ensure!(starts.len() == ends.len());
            // After seeing multiple Z positions, I noticed it is simply periodic.
            // I don't think it would always be the case but I assume it is here:
            starts
                .into_iter()
                .map(|start| {
                    find_z_positions(start, |n| ends.contains(&n), &lr_instructions, &network)
                        .next()
                        .expect("Endless loop")
                })
                .reduce(num_integer::lcm)
                .context("No start")?
        }
    })
}

pub const INPUTS: [&str; 4] = [
    "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
",
    "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
",
    "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
",
    include_input!(23 08),
];

#[test]
fn solver_23_08() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, 2);
    assert_eq!(solver(Part1, INPUTS[1])?, 6);
    assert_eq!(solver(Part1, INPUTS[3])?, 12737);
    assert_eq!(solver(Part2, INPUTS[2])?, 6);
    assert_eq!(solver(Part2, INPUTS[3])?, 9064949303801);
    Ok(())
}
