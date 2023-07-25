use itertools::Itertools;

use common::prelude::*;

#[derive(Debug)]
struct Board([[u8; 5]; 5], [[u8; 5]; 5]); // (rows, columns)

impl Board {
    fn sum_unmarked(&self, marks: &HashSet<u8>) -> u32 {
        self.0
            .iter()
            .flatten()
            .filter_map(|n| {
                if marks.contains(n) {
                    None
                } else {
                    Some(u32::from(*n))
                }
            })
            .sum()
    }

    fn win(&self, marks: &HashSet<u8>) -> Option<u32> {
        for line in [self.0, self.1].iter().flatten() {
            if line.iter().all(|x| marks.contains(x)) {
                return Some(self.sum_unmarked(marks));
            }
        }
        None
    }
}

/// Giant Squid
pub fn solver(part: Part, input: &str) -> Result<String> {
    let mut lines = input.split("\n\n");
    let numbers: Vec<_> = lines
        .next()
        .context("Zero line")?
        .split(',')
        .map(str::parse)
        .try_collect()?;
    let mut boards = vec![];
    for grid in lines {
        let mut rows = [[0; 5]; 5];
        for (n, line) in grid.lines().enumerate() {
            let ns: Vec<u8> = line.split_whitespace().map(str::parse).try_collect()?;
            rows[n] = ns.try_into().ok().context("Not 5 integers")?;
        }
        let mut cols = [[0; 5]; 5];
        for (r, row) in rows.iter().enumerate() {
            for (c, n) in row.iter().enumerate() {
                cols[c][r] = *n;
            }
        }
        boards.push(Board(rows, cols));
    }
    let mut marks = HashSet::new();
    let mut score = numbers.into_iter().flat_map(|number| {
        marks.insert(number);
        let idx_scores: Vec<(usize, u32)> = boards
            .iter()
            .enumerate()
            .filter_map(|(idx, board)| Some((idx, board.win(&marks)? * u32::from(number))))
            .collect();
        for (idx, _) in idx_scores.iter().rev() {
            boards.remove(*idx);
        }
        idx_scores
    });
    let idx_score = match part {
        Part1 => score.next().context("No score")?.1,
        Part2 => score.last().context("No score")?.1,
    };
    Ok(idx_score.to_string())
}

pub const INPUTS: [&str; 2] = [
    "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
",
    include_str!("input.txt"),
];

#[test]
fn solver_21_04() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "4512");
    assert_eq!(solver(Part1, INPUTS[1])?, "64084");
    assert_eq!(solver(Part2, INPUTS[0])?, "1924");
    assert_eq!(solver(Part2, INPUTS[1])?, "12833");
    Ok(())
}
