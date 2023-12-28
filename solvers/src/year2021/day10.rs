use common::prelude::*;

#[derive(Debug)]
enum Chunks {
    Valid,
    Incomplete(String),
    Corrupted(String),
}

/// Syntax Scoring
pub fn solver(part: Part, input: &str) -> Result<usize> {
    let forbidden_chars: String = input
        .chars()
        .collect::<HashSet<_>>()
        .difference(&"()[]{}<>\n".chars().collect())
        .collect();
    ensure!(
        forbidden_chars.is_empty(),
        "forbidden brackets: \"{}\"",
        forbidden_chars
    );
    let mut scores: Vec<usize> = input
        .lines()
        .map(|line| {
            let mut sline = line.to_owned();
            let mut changes = true;
            while changes {
                changes = false;
                for s in ["()", "[]", "{}", "<>"] {
                    if sline.contains(s) {
                        sline = sline.replace(s, "");
                        changes = true;
                    }
                }
            }
            if sline.is_empty() {
                Chunks::Valid
            } else if sline.trim_start_matches(['(', '[', '{', '<']).is_empty() {
                Chunks::Incomplete(sline)
            } else {
                Chunks::Corrupted(sline)
            }
        })
        .filter_map(|chunk| match (part, chunk) {
            (Part1, Chunks::Corrupted(s)) => s.chars().find_map(|c| match c {
                ')' => Some(3),
                ']' => Some(57),
                '}' => Some(1197),
                '>' => Some(25137),
                _ => None,
            }),
            (Part2, Chunks::Incomplete(s)) => Some(
                s.chars()
                    .rev()
                    .map(|c| match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!("Remains non-opening brackets: {:?}", c),
                    })
                    .fold(0, |prev, new| 5 * prev + new),
            ),
            _ => None,
        })
        .collect();
    Ok(match part {
        Part1 => scores.iter().sum(),
        Part2 => {
            ensure!(scores.len() % 2 == 1, "Even number of scores");
            scores.sort_unstable();
            scores[scores.len() / 2]
        }
    })
}

test_solver! {
    "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
" => (26397, 288957),
    include_input!(21 10) => (341823, 2801302861),
}
