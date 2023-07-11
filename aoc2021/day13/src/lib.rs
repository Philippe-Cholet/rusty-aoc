use itertools::Itertools;

use common::{prelude::*, Ok};
use utils::OkIterator;

/// Transparent Origami
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (coords, fold_alongs) = input
        .split_once("\n\n")
        .context("No empty line after coords")?;
    let mut coords = coords
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').context("no comma")?;
            Ok((x.parse()?, y.parse()?))
        })
        .ok_collect_hset()?;
    let mut fold_alongs = fold_alongs
        .lines()
        .map(|line| {
            let (xy, n) = line
                .rsplit_once(' ')
                .context("no space")?
                .1
                .split_once('=')
                .context("no equal")?;
            Ok((xy == "x", n.parse::<u32>()?))
        })
        .ok_collect_vec()?;
    if part == Part1 {
        fold_alongs = fold_alongs.into_iter().take(1).collect();
    }
    for (vertical, n) in fold_alongs {
        coords = coords
            .into_iter()
            .map(|(x, y)| {
                (
                    if vertical && n < x {
                        n - n.abs_diff(x)
                    } else {
                        x
                    },
                    if !vertical && n < y {
                        n - n.abs_diff(y)
                    } else {
                        y
                    },
                )
            })
            .collect();
    }
    Ok(match part {
        Part1 => coords.len().to_string(),
        Part2 => {
            let (xs, ys): (Vec<_>, Vec<_>) = coords.into_iter().unzip();
            let (&x0, &x1) = xs
                .iter()
                .minmax()
                .into_option()
                .context("No point left?!")?;
            let (&y0, &y1) = ys
                .iter()
                .minmax()
                .into_option()
                .context("No point left?!")?;
            let mut grid = vec![vec!['░'; (x1 - x0 + 1) as usize]; (y1 - y0 + 1) as usize];
            for (x, y) in xs.into_iter().zip(ys.into_iter()) {
                grid[(y - y0) as usize][(x - x0) as usize] = '█';
            }
            grid.into_iter().map(|line| line.iter().join("")).join("\n")
        }
    })
}

pub const INPUTS: [&str; 2] = [
    "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_13() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "17");
    assert_eq!(solver(Part1, INPUTS[1])?, "607");
    let answers = [
        "\
█████
█░░░█
█░░░█
█░░░█
█████",
        "\
░██░░███░░████░█░░░░███░░████░████░█░░░
█░░█░█░░█░░░░█░█░░░░█░░█░█░░░░░░░█░█░░░
█░░░░█░░█░░░█░░█░░░░█░░█░███░░░░█░░█░░░
█░░░░███░░░█░░░█░░░░███░░█░░░░░█░░░█░░░
█░░█░█░░░░█░░░░█░░░░█░░░░█░░░░█░░░░█░░░
░██░░█░░░░████░████░█░░░░█░░░░████░████",
    ];
    assert_eq!(solver(Part2, INPUTS[0])?, answers[0]); // Squared O
    assert_eq!(solver(Part2, INPUTS[1])?, answers[1]); // CPZLPFZL
    Ok(())
}
