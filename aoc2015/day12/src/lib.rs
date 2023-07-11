use serde_json::Value;

use common::prelude::*;

/// JSAbacusFramework.io
pub fn solver(part: Part, input: &str) -> Result<String> {
    Ok(sum_ints(&input.parse()?, part == Part2)?.to_string())
}

fn sum_ints(data: &Value, not_reds: bool) -> Result<i64> {
    match data {
        Value::Number(n) => n.as_i64().context("Not i64"),
        Value::Array(arr) => arr.iter().map(|v| sum_ints(v, not_reds)).sum(),
        Value::Object(obj) => {
            if not_reds
                && obj
                    .values()
                    .any(|v| matches!(v, Value::String(s) if s == "red"))
            {
                Ok(0)
            } else {
                obj.values().map(|v| sum_ints(v, not_reds)).sum()
            }
        }
        Value::Null | Value::String(_) | Value::Bool(_) => Ok(0),
    }
}

pub const INPUTS: [&str; 12] = [
    "[1,2,3]",
    r#"{"a":2,"b":4}"#,
    "[[[3]]]",
    r#"{"a":{"b":4},"c":-1}"#,
    r#"{"a":[-1,1]}"#,
    r#"[-1,{"a":1}]"#,
    "[]",
    "{}",
    r#"[1,{"c":"red","b":2},3]"#,
    r#"{"d":"red","e":[1,2,3,4],"f":5}"#,
    r#"[1,"red",5]"#,
    include_str!("input.txt"),
];

#[test]
fn solver_15_12() -> Result<()> {
    for (input, answer) in INPUTS.into_iter().zip([6, 6, 3, 3, 0, 0, 0, 0]) {
        assert_eq!(solver(Part1, input)?, answer.to_string());
    }
    assert_eq!(solver(Part1, INPUTS[11])?, "156366");
    for (idx, answer) in [0, 8, 9, 10].into_iter().zip([6, 4, 0, 6]) {
        assert_eq!(solver(Part2, INPUTS[idx])?, answer.to_string());
    }
    assert_eq!(solver(Part2, INPUTS[11])?, "96852");
    Ok(())
}
