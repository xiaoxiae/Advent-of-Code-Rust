use crate::util::Day;
use std::collections::HashMap;

pub struct D8;

type ConditionFn = Box<dyn Fn(i32, i32) -> bool>;

fn parse_condition_functions() -> HashMap<&'static str, ConditionFn> {
    vec![
        (">", Box::new(|a, b| a > b) as ConditionFn),
        ("<", Box::new(|a, b| a < b)),
        (">=", Box::new(|a, b| a >= b)),
        ("<=", Box::new(|a, b| a <= b)),
        ("!=", Box::new(|a, b| a != b)),
        ("==", Box::new(|a, b| a == b)),
    ]
        .into_iter()
        .collect()
}

fn process_instructions(input: &str) -> (HashMap<String, i32>, i32) {
    let condition_functions = parse_condition_functions();
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut highest_held_value = i32::MIN;

    for instruction in input.lines() {
        let parts: Vec<&str> = instruction.split(" if ").collect();
        let modi: Vec<&str> = parts[0].split_whitespace().collect();
        let condi: Vec<&str> = parts[1].split_whitespace().collect();

        // Ensure registers exist
        registers.entry(modi[0].to_string()).or_insert(0);
        registers.entry(condi[0].to_string()).or_insert(0);

        // Check condition
        if condition_functions[condi[1]](registers[condi[0]], condi[2].parse().unwrap()) {
            let entry = registers.get_mut(modi[0]).unwrap();
            let delta = modi[2].parse::<i32>().unwrap();
            if modi[1] == "inc" {
                *entry += delta;
            } else {
                *entry -= delta;
            }
        }

        // Track highest value ever held
        highest_held_value = highest_held_value.max(*registers.values().max().unwrap_or(&0));
    }

    (registers, highest_held_value)
}

impl Day for D8 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (registers, _) = process_instructions(input);
        Some(registers.values().cloned().max().unwrap_or(0).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (_, highest_held_value) = process_instructions(input);
        Some(highest_held_value.to_string())
    }
}
