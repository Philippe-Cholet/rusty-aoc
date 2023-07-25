use common::prelude::*;
use utils::OkIterator;

#[derive(Debug, PartialEq)]
enum Rule {
    Char(char),
    SubRules(Vec<Vec<usize>>),
}

fn backtrack(mut chars: std::str::Chars, indexes: &mut Vec<usize>, rules: &Vec<Rule>) -> bool {
    // NOTE: At the end of it, `indexes` are unchanged.
    let Some(idx) = indexes.pop() else {
        return chars.next().is_none(); // Reached the end?!
    };
    let success = match &rules[idx] {
        Rule::Char(ch) => chars.next().as_ref() == Some(ch) && backtrack(chars, indexes, rules),
        Rule::SubRules(paths) => paths.iter().any(|path| {
            let len = indexes.len();
            indexes.extend(path.iter().rev());
            let res = backtrack(chars.clone(), indexes, rules);
            indexes.truncate(len);
            res
        }),
    };
    indexes.push(idx);
    success
}

/// Monster Messages
pub fn solver(part: Part, input: &str) -> Result<String> {
    let (rules, messages) = input.split_once("\n\n").context("No blank line")?;
    let mut rules = rules
        .lines()
        .map(|line| {
            let (id, s) = line.split_once(": ").context("No colon")?;
            let id = id.parse()?;
            let mut rule = s.parse()?;
            if part.two() {
                if id == 8 {
                    ensure!(rule == Rule::SubRules(vec![vec![42]]), "Wrong rule 8");
                    rule = Rule::SubRules(vec![vec![42], vec![42, 8]]);
                } else if id == 11 {
                    ensure!(rule == Rule::SubRules(vec![vec![42, 31]]), "Wrong rule 11");
                    rule = Rule::SubRules(vec![vec![42, 31], vec![42, 11, 31]]);
                }
            }
            Ok((id, rule))
        })
        .ok_collect_vec()?;
    rules.sort_by_key(|x| x.0);
    let table: HashMap<_, _> = rules
        .iter()
        .enumerate()
        .map(|(idx, (id, _))| (*id, idx))
        .collect();
    let rules = rules
        .into_iter()
        .map(|(_, mut rule)| rule.translate(&table).map(|_| rule))
        .ok_collect_vec()?;
    Ok(messages
        .lines()
        .filter(|s| backtrack(s.chars(), &mut vec![0], &rules)) // Must match rule 0.
        .count()
        .to_string())
}

impl Rule {
    // On the 2nd example, rule numbers are not contiguous.
    // Therefore, to get rules in `Vec<Rule>`, I translate indexes.
    // That was that or use slow HashMap or `Vec<Option<Rule>>` and test if it's "some" later.
    fn translate(&mut self, table: &HashMap<usize, usize>) -> Result<()> {
        if let Self::SubRules(paths) = self {
            for path in paths {
                for idx in path {
                    *idx = *table.get(idx).context("Missing idx")?;
                }
            }
        }
        Ok(())
    }
}

impl std::str::FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with('"') {
            ensure!(s.len() == 3 && s.ends_with('"'), "Not \"s\"");
            s.chars().nth(1).context("Weirdly empty").map(Self::Char)
        } else {
            s.split(" | ")
                .map(|t| t.split_whitespace().map(str::parse).collect())
                .ok_collect()
                .map(Self::SubRules)
        }
    }
}

pub const INPUTS: [&str; 3] = [
    r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#,
    r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#,
    include_str!("input.txt"),
];

#[test]
fn solver_20_19() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "2");
    assert_eq!(solver(Part1, INPUTS[2])?, "208");
    assert_eq!(solver(Part2, INPUTS[1])?, "12");
    assert_eq!(solver(Part2, INPUTS[2])?, "316");
    Ok(())
}
