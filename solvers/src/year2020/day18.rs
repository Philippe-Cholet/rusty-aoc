use itertools::Itertools;
use pest::{iterators::Pair, Parser};

use common::prelude::*;

/// Operation Order
pub fn solver(part: Part, input: &str) -> Result<u64> {
    input
        .lines()
        .map(|line| {
            let pair = ExprParser::parse(Rule::expr, line)?
                .exactly_one()
                .ok()
                .context("Not ONE expression")?;
            eval(part, pair)
        })
        .sum()
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

test_solver! {
    "1 + 2 * 3 + 4 * 5 + 6" => (71, 231),
    "1 + (2 * 3) + (4 * (5 + 6))" => (51, 51),
    "2 * 3 + (4 * 5)" => (26, 46),
    "5 + (8 * 3 + 9 + 3 * 4 * 3)" => (437, 1445),
    "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))" => (12240, 669060),
    "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2" => (13632, 23340),
    include_input!(20 18) => (280014646144, 9966990988262),
}
