use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file = "data/input.in";

    let input = fs::read_to_string(input_file)
        .expect(&format!("Failed to read input file: {}", input_file));

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let parts: Vec<Vec<i32>> = input
        .trim()
        .split('\n')
        .map(|s| s.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();

    (0..parts[0].len())
        .map(|col| parts.iter().map(|row| row[col]).collect())
        .collect()
}

fn solve_part1(input: &str) -> String {
    let mut lists = parse_input(input);

    lists[0].sort();
    lists[1].sort();

    let differences: i32 = lists[0]
        .iter()
        .zip(lists[1].iter())
        .map(|(x, y)| (x - y).abs())
        .sum();

    differences.to_string()
}

fn solve_part2(input: &str) -> String {
    let lists = parse_input(input);

    let mut occurrences: HashMap<i32, i32> = HashMap::new();

    for &item in &lists[1] {
        *occurrences.entry(item).or_insert(0) += 1;
    }

    lists[0].iter()
        .map(|x| x * *occurrences.get(x).unwrap_or(&0))
        .sum::<i32>()
        .to_string()
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
