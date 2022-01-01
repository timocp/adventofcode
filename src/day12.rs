use crate::Part;
use serde_json::Value;

pub fn run(input: &str, part: Part) -> String {
    let json = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => sum(&json, ""),
            Part::Two => sum(&json, "red"),
        }
    )
}

fn parse_input(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn sum(json: &Value, except: &str) -> i64 {
    match json {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(vec) => vec.iter().map(|v| sum(v, except)).sum(),
        Value::Object(map) => {
            if map.iter().any(|(_, v)| v.is_string() && v == except) {
                0
            } else {
                map.iter().map(|(_, v)| sum(v, except)).sum()
            }
        }
        _ => 0, // Null, Bool, String
    }
}

#[test]
fn test() {
    assert_eq!(6, sum(&parse_input("[1,2,3]"), ""));
    assert_eq!(6, sum(&parse_input(r#"{"a":2,"b":4}"#), ""));
    assert_eq!(3, sum(&parse_input("[[[3]]]"), ""));
    assert_eq!(3, sum(&parse_input(r#"{"a":{"b":4},"c":-1}"#), ""));
    assert_eq!(0, sum(&parse_input(r#"{"a":[-1,1]}"#), ""));
    assert_eq!(0, sum(&parse_input(r#"[-1,{"a":1}]"#), ""));
    assert_eq!(0, sum(&parse_input("[]"), ""));
    assert_eq!(0, sum(&parse_input("{}"), ""));

    assert_eq!(6, sum(&parse_input("[1,2,3]"), "red"));
    assert_eq!(4, sum(&parse_input(r#"[1,{"c":"red","b":2},3]"#), "red"));
    assert_eq!(
        0,
        sum(&parse_input(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), "red")
    );
    assert_eq!(6, sum(&parse_input(r#"[1,"red",5]"#), "red"));
}
