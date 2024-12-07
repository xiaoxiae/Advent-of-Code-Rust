mod util;

mod days {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
}

use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::util::Day;
use colored::Colorize;
use serde::Serialize;
use std::io::Write;
use std::time::Instant;

#[derive(Serialize)]
struct TimingResult {
    day: usize,
    part1_time: f64, // Duration in seconds
    part2_time: f64, // Duration in seconds
}

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
        Box::new(Day6),
    ];

    let mut timing_results = Vec::new();

    for (i, day) in days.iter().enumerate() {
        let day_number = i + 1;
        let input_file = format!("data/day{}/input.in", day_number);
        let input = std::fs::read_to_string(&input_file)
            .expect(&format!("Failed to read input file: {}", input_file));

        println!(
            "{0} {1} {2}{2} {0}",
            "---".bright_black(),
            format!("Day {}", day_number.to_string()).bold(),
            "*".bright_yellow().bold(),
        );

        let start = Instant::now();
        let part1_result = day.solve_part1(&input);
        let duration_part1 = start.elapsed();
        println!(
            "Part 1: {} (took {})",
            part1_result.green(),
            format!("{:.2?}", duration_part1).yellow()
        );

        let start = Instant::now();
        let part2_result = day.solve_part2(&input);
        let duration_part2 = start.elapsed();
        println!(
            "Part 2: {} (took {})\n",
            part2_result.green(),
            format!("{:.2?}", duration_part2).yellow()
        );

        timing_results.push(TimingResult {
            day: day_number,
            part1_time: duration_part1.as_secs_f64(),
            part2_time: duration_part2.as_secs_f64(),
        });
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

    #[test]
    fn test_all_days() {
        let days: Vec<(Box<dyn Day>, &str)> = vec![
            (Box::new(Day1), "day1"),
            (Box::new(Day2), "day2"),
            (Box::new(Day3), "day3"),
            (Box::new(Day4), "day4"),
            (Box::new(Day5), "day5"),
            (Box::new(Day6), "day6"),
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
                        assert_eq!(day.solve_part1(&input), expected_output);
                    } else {
                        assert_eq!(day.solve_part2(&input), expected_output);
                    }
                }
            }
        }
    }
}
