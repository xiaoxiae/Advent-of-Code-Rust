use std::fs;
use std::time::Instant;

fn main() {
    let input_file = "data/input.in";

    let input = fs::read_to_string(input_file)
        .expect(&format!("Failed to read input file: {}", input_file));

    let start = Instant::now();
    let part1_result = solve_part1(&input);
    let duration_part1 = start.elapsed();
    println!("Part 1: {} (took {:.2?})", part1_result, duration_part1);

    let start = Instant::now();
    let part2_result = solve_part2(&input);
    let duration_part2 = start.elapsed();
    println!("Part 2: {} (took {:.2?})", part2_result, duration_part2);
}

fn is_safe(report: &Vec<i32>) -> bool {
    let sign = (report[0] - report[1]).signum();

    for i in 0..report.len() - 1 {
        let delta = report[i] - report[i + 1];

        if delta.signum() != sign {
            return false;
        } else if !(1 <= delta.abs() && delta.abs() <= 3) {
            return false;
        }
    }

    true
}

fn solve_part1(input: &str) -> String {
    let mut safe = 0;

    for line in input.trim().lines() {
        let numbers = line.split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_safe(&numbers) {
            safe += 1;
        }
    }

    safe.to_string()
}

fn is_any_safe(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let result: Vec<_> = report[..i].iter().chain(&report[i+1..]).cloned().collect();

        if is_safe(&result) {
            return true;
        }
    }

    false
}

fn solve_part2(input: &str) -> String {
    let mut safe = 0;

    for line in input.trim().lines() {
        let numbers = line.split_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_any_safe(&numbers) {
            safe += 1;
        }
    }

    safe.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn find_samples(dir: &str) -> Vec<(String, String)> {
        let mut samples = vec![];

        for entry in fs::read_dir(dir).expect(&format!("Failed to read '{}' directory", dir)) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                    if file_name.starts_with("sample") && file_name.ends_with(".in") {
                        let base_name = &file_name[..file_name.len() - 3];
                        let input_file = path.clone();
                        let output_file = path.with_file_name(format!("{}.out", base_name));

                        if output_file.exists() {
                            samples.push((
                                input_file.to_str().unwrap().to_string(),
                                output_file.to_str().unwrap().to_string(),
                            ));
                        }
                    }
                }
            }
        }

        samples
    }

    fn run_test(input_file: &str, expected_output_file: &str, solver: fn(&str) -> String) {
        let input = fs::read_to_string(input_file)
            .expect(&format!("Failed to read input file: {}", input_file));
        let expected_output = fs::read_to_string(expected_output_file)
            .expect(&format!(
                "Failed to read output file: {}",
                expected_output_file
            ))
            .trim()
            .to_string(); // Convert to String

        assert_eq!(solver(&input), expected_output);
    }

    #[test]
    fn test_part1_samples() {
        let samples = find_samples("data/part1");
        for (input_file, output_file) in &samples {
            run_test(input_file, output_file, solve_part1);
        }
    }

    #[test]
    fn test_part2_samples() {
        let samples = find_samples("data/part2");
        for (input_file, output_file) in &samples {
            run_test(input_file, output_file, solve_part2);
        }
    }
}
