use serde_json::Value;

use common::prelude::*;

/// JSAbacusFramework.io
pub fn solver(part: Part, input: &str) -> Result<i64> {
    sum_ints(&input.parse()?, part.two())
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

test_solver! {
    "[1,2,3]" => (6, 6),
    r#"{"a":2,"b":4}"# => 6,
    "[[[3]]]" => 3,
    r#"{"a":{"b":4},"c":-1}"# => 3,
    r#"{"a":[-1,1]}"# => 0,
    r#"[-1,{"a":1}]"# => 0,
    "[]" => 0,
    "{}" => 0,
    r#"[1,{"c":"red","b":2},3]"# => ((), 4),
    r#"{"d":"red","e":[1,2,3,4],"f":5}"# => ((), 0),
    r#"[1,"red",5]"# => ((), 6),
    include_input!(15 12) => (156366, 96852),
}
