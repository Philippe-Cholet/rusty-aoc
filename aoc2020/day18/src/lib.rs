use itertools::Itertools;
use pest::{iterators::Pair, Parser};

use common::{Context, Part, Part1, Part2, Result};
use utils::OkIterator;

/// Operation Order
pub fn solver(part: Part, input: &str) -> Result<String> {
    Ok(input
        .lines()
        .map(|line| {
            let pair = ExprParser::parse(Rule::expr, line)?
                .exactly_one()
                .ok()
                .context("Not ONE expression")?;
            eval(part, pair)
        })
        .ok_sum::<u64>()?
        .to_string())
}

#[derive(pest_derive::Parser)] // It implements the above `Parser` trait.
#[grammar_inline = r#"
WHITESPACE =  _{ " " }

number   = @{ (ASCII_NONZERO_DIGIT ~ ASCII_DIGIT+ | ASCII_DIGIT) }
operator =  { "+" | "*" }
expr     =  { term ~ (operator ~ term )* }
term     = _{ number | "(" ~ expr ~ ")" }
"#] // It generates an enum named `Rule`.
struct ExprParser;

fn eval(part: Part, pair: Pair<Rule>) -> Result<u64> {
    Ok(match pair.as_rule() {
        Rule::expr => {
            let mut it = pair.into_inner();
            let Some(first) = it.next() else {
                unreachable!("An expression is defined non-empty");
            };
            let mut result = eval(part, first)?;
            match part {
                Part1 => {
                    for (op, next) in it.tuples() {
                        let next = eval(part, next)?;
                        match op.as_str() {
                            "+" => result += next,
                            "*" => result *= next,
                            _ => unreachable!("Defined operations are only + and *"),
                        }
                    }
                }
                Part2 => {
                    let mut prod_result = 1;
                    for (op, next) in it.tuples() {
                        let next = eval(part, next)?;
                        match op.as_str() {
                            "+" => result += next,
                            "*" => {
                                prod_result *= result;
                                result = next;
                            }
                            _ => unreachable!("Defined operations are only + and *"),
                        }
                    }
                    result *= prod_result;
                }
            }
            result
        }
        Rule::number => pair.as_str().parse()?,
        Rule::operator => unreachable!("An operator is not evaluable alone"),
        Rule::term | Rule::WHITESPACE => unreachable!("Silent rule"),
    })
}

pub const INPUTS: [&str; 7] = [
    "1 + 2 * 3 + 4 * 5 + 6",
    "1 + (2 * 3) + (4 * (5 + 6))",
    "2 * 3 + (4 * 5)",
    "5 + (8 * 3 + 9 + 3 * 4 * 3)",
    "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
    "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
    include_str!("input.txt"),
];

#[test]
fn solver_20_18() -> Result<()> {
    assert_eq!(solver(Part1, INPUTS[0])?, "71");
    assert_eq!(solver(Part1, INPUTS[1])?, "51");
    assert_eq!(solver(Part1, INPUTS[2])?, "26");
    assert_eq!(solver(Part1, INPUTS[3])?, "437");
    assert_eq!(solver(Part1, INPUTS[4])?, "12240");
    assert_eq!(solver(Part1, INPUTS[5])?, "13632");
    assert_eq!(solver(Part1, INPUTS[6])?, "280014646144");

    assert_eq!(solver(Part2, INPUTS[0])?, "231");
    assert_eq!(solver(Part2, INPUTS[1])?, "51");
    assert_eq!(solver(Part2, INPUTS[2])?, "46");
    assert_eq!(solver(Part2, INPUTS[3])?, "1445");
    assert_eq!(solver(Part2, INPUTS[4])?, "669060");
    assert_eq!(solver(Part2, INPUTS[5])?, "23340");
    assert_eq!(solver(Part2, INPUTS[6])?, "9966990988262");

    Ok(())
}
