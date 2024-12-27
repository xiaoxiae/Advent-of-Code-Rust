use crate::util::Day;
use regex::Regex;
use serde_json::Value;

pub struct D12;

fn count_non_red_numbers(value: &Value) -> isize {
    match value {
        Value::Array(arr) => {
            let mut total = 0;

            for elem in arr {
                total += count_non_red_numbers(elem);
            }

            total
        }
        Value::Object(obj) => {
            let mut total = 0;

            for val in obj.values() {
                match val {
                    Value::String(v) if v == "red" => return 0,
                    _ => {},
                }
            }

            for val in obj.values() {
                total += count_non_red_numbers(val);
            }

            total
        }
        Value::Number(n) => {
            n.as_i64().unwrap() as isize
        }
        _ => 0
    }
}


impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let re = Regex::new(r"[-+]?\d+").unwrap();

        let total = re.find_iter(input.trim())
            .filter_map(|mat| mat.as_str().parse::<isize>().ok())
            .sum::<isize>();

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let json: Value = match serde_json::from_str(input) {
            Ok(value) => value,
            Err(_) => return None, // Return None if the input is not valid JSON
        };

        let total = count_non_red_numbers(&json);

        Option::from(total.to_string())
    }
}
