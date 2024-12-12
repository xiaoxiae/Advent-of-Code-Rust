mod util;

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
    pub mod day7;
    pub mod day8;
    pub mod day9;
    pub mod day10;
    pub mod day11;
    pub mod day12;
}

use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::days::day7::Day7;
use crate::days::day8::Day8;
use crate::days::day9::Day9;
use crate::days::day10::Day10;
use crate::days::day11::Day11;
use crate::days::day12::Day12;
use crate::util::Day;
use colored::Colorize;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct TimingResult {
    day: usize,
    times: HashMap<String, (String, f64)>,
}

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
        Box::new(Day6),
        Box::new(Day7),
        Box::new(Day8),
        Box::new(Day9),
        Box::new(Day10),
        Box::new(Day11),
        Box::new(Day12),
    ];

    let mut timing_results = Vec::new();

    for (i, day) in days.iter().enumerate() {
        let day_number = i + 1;
        let input_file = format!("data/day{}/input.in", day_number);
        let input = std::fs::read_to_string(&input_file)
            .expect(&format!("Failed to read input file: {}", input_file));

        let mut stars = 0;
        let mut results = TimingResult {
            day: 0,
            times: Default::default(),
        };

        // TODO: can this be done nicer, i.e. by iterating over the functions?
        for part in 1..=3 {
            let start = Instant::now();

            let result = if part == 1 {
                day.solve_part1(&input)
            } else if part == 2 {
                day.solve_part2(&input)
            } else {
                day.solve_part3(&input)
            };

            let duration = start.elapsed();

            match result {
                Some(value) => {
                    stars += 1;
                    results
                        .times
                        .insert(part.to_string(), (value, duration.as_secs_f64()));
                }
                None => {}
            }
        }

        println!(
            "{0} {1} {2} {0}",
            "---".bright_black(),
            format!("Day {}", day_number.to_string()).bold(),
            "*".repeat(stars).bright_yellow().bold(),
        );

        let mut sorted_keys: Vec<String> = results.times.keys().cloned().collect();
        sorted_keys.sort();

        for part in sorted_keys {
            if let Some((result, seconds)) = results.times.get(&part) {
                println!(
                    "Part {}: {} (took {})",
                    part.bold(),
                    result.green(),
                    format!("{:.2?}", Duration::from_secs_f64(*seconds)).yellow()
                );
            }
        }

        println!();

        timing_results.push(results);
    }

    let json_file = "timing_results.json";
    let json_data = serde_json::to_string_pretty(&timing_results)
        .expect("Failed to serialize timing results to JSON");
    let mut file = std::fs::File::create(json_file).expect("Failed to create JSON file");
    file.write_all(json_data.as_bytes())
        .expect("Failed to write JSON data to file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::option::Option;

    fn find_samples(dir: &str) -> Vec<(String, String)> {
        let mut samples = vec![];

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
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
        }

        samples
    }

    #[test]
    fn test_all_days() {
        let days: Vec<(Box<dyn Day>, &str)> = vec![
            (Box::new(Day1), "day1"),
            (Box::new(Day2), "day2"),
            (Box::new(Day3), "day3"),
            (Box::new(Day4), "day4"),
            (Box::new(Day5), "day5"),
            (Box::new(Day6), "day6"),
            (Box::new(Day7), "day7"),
            (Box::new(Day8), "day8"),
            (Box::new(Day9), "day9"),
            (Box::new(Day10), "day10"),
            (Box::new(Day11), "day11"),
            (Box::new(Day12), "day12"),
        ];

        for (day, day_name) in days {
            for part in ["part1", "part2"] {
                let samples = find_samples(format!("data/{}/{}", day_name, part).as_str());

                println!("Testing {}/{}", day_name, part);

                for (input_file, output_file) in &samples {
                    let input = fs::read_to_string(input_file)
                        .expect(&format!("Failed to read input file: {}", input_file));

                    let expected_output = fs::read_to_string(output_file)
                        .expect(&format!("Failed to read output file: {}", output_file))
                        .trim()
                        .to_string(); // Convert to String

                    if part == "part1" {
                        assert_eq!(day.solve_part1(&input), Option::from(expected_output));
                    } else {
                        assert_eq!(day.solve_part2(&input), Option::from(expected_output));
                    }
                }
            }
        }
    }
}
