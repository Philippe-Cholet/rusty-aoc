use itertools::{iproduct, Itertools};

use common::prelude::*;
use utils::OkIterator;

/// Trench Map
pub fn solver(part: Part, input: &str) -> Result<String> {
    let char2bool = |ch: char| -> Result<bool> {
        Ok(match ch {
            '#' => true,
            '.' => false,
            _ => bail!("Wrong char detected: {}", ch),
        })
    };
    let bool2char = |b| if b { '#' } else { '.' };
    let display_image = |image: &Vec<Vec<bool>>| {
        if !cfg!(debug_assertions) {
            return;
        }
        for row in image {
            println!("{}", row.iter().map(|p| bool2char(*p)).join(""));
        }
        println!();
    };
    let (enhancement, input_image) = input
        .split_once("\n\n")
        .context("no empty line between sections")?;
    let enhancement: [bool; 512] = enhancement.chars().map(char2bool).ok_collect_array()?;
    let nrows = input_image.lines().count();
    let ncols = input_image
        .lines()
        .map(str::len)
        .max()
        .context("No image")?;
    let steps = part.value(2, 50);
    let n = steps + 1;
    let mut fill_value = false;
    let mut image = vec![vec![fill_value; ncols + 2 * n]; nrows + 2 * n];
    for (r, line) in input_image.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if char2bool(ch)? {
                image[r + n][c + n] = true;
            }
        }
    }
    display_image(&image);
    for i in 1..=steps {
        // So with a fill value changing each time, the image is quite flashy, weird!
        fill_value = if fill_value {
            enhancement[511]
        } else {
            enhancement[0]
        };
        let mut new_image = vec![vec![fill_value; ncols + 2 * n]; nrows + 2 * n];
        for (r, c) in iproduct!(n - i..nrows + n + i, n - i..ncols + n + i) {
            let bin = iproduct!(r - 1..=r + 1, c - 1..=c + 1)
                .fold(0, |res, (r, c)| (res << 1) | usize::from(image[r][c]));
            debug_assert!(bin < 512);
            new_image[r][c] = enhancement[bin];
        }
        image = new_image;
        display_image(&image);
    }
    let result = image.iter().flatten().filter(|&b| *b).count();
    Ok(result.to_string())
}

pub const INPUTS: [&str; 2] = [
    "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_20() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "35");
    assert_eq!(solver(Part1, INPUTS[1])?, "4917");
    assert_eq!(solver(Part2, INPUTS[0])?, "3351");
    assert_eq!(solver(Part2, INPUTS[1])?, "16389");
    Ok(())
}
