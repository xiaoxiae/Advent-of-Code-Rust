mod util;

macro_rules! define_days {
    ($($day_snake:ident => $day_pascal:ident),*) => {
        macro_rules! days {
            ($action:ident) => {
                $action!($($day_snake => $day_pascal),*);
            };
        }
    };
}

macro_rules! declare_days {
    ($($day_snake:ident => $day_pascal:ident),*) => {
        mod days {
            $(
                pub mod $day_snake;
            )*
        }

        $(
            use crate::days::$day_snake::$day_pascal;
        )*
    };
}

macro_rules! get_days {
    ($($day_snake:ident => $day_pascal:ident),*) => {
        vec![
            $(
                (Box::new($day_pascal), stringify!($day_snake)),
            )*
        ]
    };
}

define_days!(
    y24d1 => Y24D1,
    y24d2 => Y24D2,
    y24d3 => Y24D3,
    y24d4 => Y24D4,
    y24d5 => Y24D5,
    y24d6 => Y24D6,
    y24d7 => Y24D7,
    y24d8 => Y24D8,
    y24d9 => Y24D9,
    y24d10 => Y24D10,
    y24d11 => Y24D11,
    y24d12 => Y24D12,
    y24d13 => Y24D13,
    y24d14 => Y24D14,
    y24d15 => Y24D15,
    y24d16 => Y24D16,
    y24d17 => Y24D17,
    y24d18 => Y24D18,
    y24d19 => Y24D19,
    y24d20 => Y24D20,
    y24d21 => Y24D21,
    y24d22 => Y24D22
);

days!(declare_days);

use crate::util::Day;
use colored::Colorize;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct TimingResult {
    day: usize,
    times: HashMap<String, (String, f64)>,
    year: usize,
}

fn parse_solution_date(input: &str) -> (usize, usize) {
    let re = Regex::new(r"^y(\d+)d(\d+)$").unwrap();

    if let Some(captures) = re.captures(input) {
        let year = captures.get(1).unwrap().as_str().parse().unwrap();
        let day = captures.get(2).unwrap().as_str().parse().unwrap();

        return (year, day);
    }

    panic!("Invalid date string '{}'", input);
}

/// Gets the actual length of a string by stripping the ANSI escape codes.
fn get_length_without_colors(input: &str) -> usize {
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let clean_input = ansi_regex.replace_all(input, "");
    clean_input.len()
}

fn main() {
    let days: Vec<(Box<dyn Day>, &str)> = days!(get_days);

    let mut timing_results = Vec::new();

    let mut last_year = 0;

    let mut total_time: f64 = 0.0;
    let mut total_problems: usize = 0;

    for  (day_object, day_name) in days.iter() {
        let (year, day) = parse_solution_date(day_name);

        let input_file = format!("data/{}/input.in", day_name);
        let input = std::fs::read_to_string(&input_file)
            .expect(&format!("Failed to read input file: {}", input_file));

        let mut stars = 0;
        let mut results = TimingResult {
            day: day,
            year: year,
            times: Default::default(),
        };

        // TODO: can this be done nicer, i.e. by iterating over the functions?
        for part in 1..=3 {
            let start = Instant::now();

            let result = if part == 1 {
                day_object.solve_part1(&input)
            } else if part == 2 {
                day_object.solve_part2(&input)
            } else {
                day_object.solve_part3(&input)
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

        if year != last_year {
            println!(
                "{0}\n{1}\n{0}\n",
                "---====---".bright_black(),
                format!("   20{}   ", year).bold(),
            );

            last_year = year;
        }

        println!(
            "{0} {1} {2} {0}",
            "-".repeat(3).bright_black(),
            format!("Day {}", day).bold(),
            "*".repeat(stars).bright_yellow().bold(),
        );

        let mut sorted_keys: Vec<String> = results.times.keys().cloned().collect();
        sorted_keys.sort();

        for part in sorted_keys {
            if let Some((result, seconds)) = results.times.get(&part) {
                // skip part 3s, since those are for fun
                if part == "3" {
                    println!(
                        "{}",
                        format!(
                            "Part {}: {} (took {})",
                            part,
                            result,
                            format!("{:.2?}", Duration::from_secs_f64(*seconds))
                        ).bright_black(),
                    );
                } else {
                    total_time += seconds;
                    total_problems += 1;

                    println!(
                        "Part {}: {} (took {})",
                        part.bold(),
                        result.green(),
                        format!("{:.2?}", Duration::from_secs_f64(*seconds)).yellow()
                    );
                }
            }
        }

        println!();

        timing_results.push(results);
    }

    if total_time != 0.0 {
        let out = format!(
            "Combined time ({} problems): {}",
            total_problems.to_string().bright_yellow(),
            format!("{:.2?}", Duration::from_secs_f64(total_time)).yellow()
        );

        println!("{}\n{}", "-".repeat(get_length_without_colors(&out)).bright_black(), out);
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
        let days: Vec<(Box<dyn Day>, &str)> = days!(get_days);

        for (day, day_name) in days {
            for part in ["part1", "part2"] {
                let samples = find_samples(format!("data/{}/{}", day_name, part).as_str());

                for (input_file, output_file) in &samples {
                    let input = fs::read_to_string(input_file)
                        .expect(&format!("Failed to read input file: {}", input_file));

                    let expected_output = fs::read_to_string(output_file)
                        .expect(&format!("Failed to read output file: {}", output_file))
                        .trim()
                        .to_string(); // Convert to String

                    if part == "part1" {
                        assert_eq!(
                            day.solve_part1(&input), Option::from(expected_output),
                            "{}, {} failed", day_name, part,
                        );
                    } else {
                        assert_eq!(
                            day.solve_part2(&input), Option::from(expected_output),
                            "{}, {} failed", day_name, part,
                        );
                    }
                }
            }
        }
    }
}
