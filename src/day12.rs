use crate::Part;
use serde_json::Value;

pub fn run(input: &str, part: Part) -> String {
    let json = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => sum_numbers(&json),
            Part::Two => 0,
        }
    )
}

fn parse_input(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn sum_numbers(json: &Value) -> i64 {
    match json {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(vec) => vec.iter().map(|v| sum_numbers(v)).sum(),
        Value::Object(map) => map.iter().map(|(_k, v)| sum_numbers(v)).sum(),
        _ => 0, // Null, Bool, String
    }
}

#[test]
fn test() {
    assert_eq!(6, sum_numbers(&parse_input("[1,2,3]")));
    assert_eq!(6, sum_numbers(&parse_input(r#"{"a":2,"b":4}"#)));
    assert_eq!(3, sum_numbers(&parse_input("[[[3]]]")));
    assert_eq!(3, sum_numbers(&parse_input(r#"{"a":{"b":4},"c":-1}"#)));
    assert_eq!(0, sum_numbers(&parse_input(r#"{"a":[-1,1]}"#)));
    assert_eq!(0, sum_numbers(&parse_input(r#"[-1,{"a":1}]"#)));
    assert_eq!(0, sum_numbers(&parse_input("[]")));
    assert_eq!(0, sum_numbers(&parse_input("{}")));
}
