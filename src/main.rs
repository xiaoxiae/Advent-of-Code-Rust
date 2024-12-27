mod util;

// Define a macro to declare days for multiple years.
macro_rules! define_years {
    ($($year:ident => { $($day_snake:ident => $day_pascal:ident),* }),*) => {
        macro_rules! days_by_year {
            ($action:ident) => {
                $(
                    $action!($year, $($day_snake => $day_pascal),*);
                )*
            };
        }
    };
}

// Declare modules and imports for each year.
macro_rules! declare_days {
    ($year:ident, $($day_snake:ident => $day_pascal:ident),*) => {
        mod $year {
            $(
                pub mod $day_snake;
            )*

            $(
                pub use crate::$year::$day_snake::$day_pascal;
            )*
        }
    };
}

// Collect all days into a single vector.
macro_rules! get_days {
    ($year:ident, $($day_snake:ident => $day_pascal:ident),*) => {
        vec![
            $(
                (Box::new($year::$day_pascal), concat!(stringify!($year), "::", stringify!($day_snake))),
            )*
        ]
    };
}

// Define all years and their respective days.
define_years!(
    y24 => {
        d1 => D1,
        d2 => D2,
        d3 => D3,
        d4 => D4,
        d5 => D5,
        d6 => D6,
        d7 => D7,
        d8 => D8,
        d9 => D9,
        d10 => D10,
        d11 => D11,
        d12 => D12,
        d13 => D13,
        d14 => D14,
        d15 => D15,
        d16 => D16,
        d17 => D17,
        d18 => D18,
        d19 => D19,
        d20 => D20,
        d21 => D21,
        d22 => D22,
        d23 => D23,
        d24 => D24,
        d25 => D25
    }
);

// Execute actions for each year and day.
days_by_year!(declare_days);

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
    let re = Regex::new(r"^y(\d+)::d(\d+)$").unwrap();

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
    let days: Vec<(Box<dyn Day>, &str)> = days_by_year!(get_days);

    let mut timing_results = Vec::new();

    let mut last_year = 0;
    let mut last_stars = 0;

    let mut total_time: f64 = 0.0;
    let mut total_valid_stars: usize = 0;

    for  (day_object, day_name) in days.iter() {
        let (year, day) = parse_solution_date(day_name);

        if year != last_year {
            println!(
                "{0}\n{1}\n{0}\n",
                "---====---".bright_black(),
                format!("   20{}   ", year).bold(),
            );

            last_year = year;
            last_stars = total_valid_stars;
        }

        let input_file = format!("data/y{}/d{}/input.in", year, day);
        let input = std::fs::read_to_string(&input_file)
            .expect(&format!("Failed to read input file: {}", input_file));

        let mut day_stars = 0;
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
                // day 25 part 2 is always special, since it only requires you to do the previous 49 challenges
                if day == 25 {
                    day_object.solve_part2(&*(total_valid_stars - last_stars).to_string())
                } else {
                    day_object.solve_part2(&input)
                }

            } else {
                day_object.solve_part3(&input)
            };

            let duration = start.elapsed();

            match result {
                Some(value) => {
                    day_stars += 1;

                    if part <= 2 {
                        total_valid_stars += 1;
                    }

                    results
                        .times
                        .insert(part.to_string(), (value, duration.as_secs_f64()));
                }
                None => {}
            }
        }

        println!(
            "{0} {1} {2} {0}",
            "-".repeat(3).bright_black(),
            format!("Day {}", day).bold(),
            "*".repeat(day_stars).bright_yellow().bold(),
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
                    let mut time_string = format!("{:.2?}", Duration::from_secs_f64(*seconds));

                    total_time += seconds;
                    println!(
                        "Part {}: {} (took {})",
                        part.bold(),
                        result.bright_blue(),
                        match seconds {
                            &s if s * 1000.0 < 1.0 => time_string.green(),
                            &s if s * 1000.0 < 10.0 => time_string.yellow(),
                            _ => time_string.red(),
                        },
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
            total_valid_stars.to_string().bright_yellow(),
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
        let days: Vec<(Box<dyn Day>, &str)> = days_by_year!(get_days);

        for (day_box, day_name) in days {
            let (year, day) = parse_solution_date(day_name);

            for part in ["part1", "part2"] {
                let samples = find_samples(format!("data/y{}/d{}/{}", year, day, part).as_str());

                for (input_file, output_file) in &samples {
                    let input = fs::read_to_string(input_file)
                        .expect(&format!("Failed to read input file: {}", input_file));

                    let expected_output = fs::read_to_string(output_file)
                        .expect(&format!("Failed to read output file: {}", output_file))
                        .trim()
                        .to_string(); // Convert to String

                    if part == "part1" {
                        assert_eq!(
                            day_box.solve_part1(&input), Option::from(expected_output),
                            "{}, {} failed", day_name, part,
                        );
                    } else {
                        assert_eq!(
                            day_box.solve_part2(&input), Option::from(expected_output),
                            "{}, {} failed", day_name, part,
                        );
                    }
                }
            }
        }
    }
}
