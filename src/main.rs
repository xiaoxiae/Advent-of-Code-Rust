#![feature(portable_simd)]

mod util;

// Define a macro to declare days for multiple years.
macro_rules! define_years {
    ($($year:ident => { $($day_snake:ident => $day_pascal:ident),* }),*) => {
        $(
            mod $year {
                $(
                    pub mod $day_snake;
                )*

                $(
                    pub use crate::$year::$day_snake::$day_pascal;
                )*
            }
        )*

        macro_rules! days_vector {
            () => {
                vec![
                    $($(
                        (Box::new($year::$day_pascal), stringify!($year), stringify!($day_snake)),
                    )*)*
                ]
            }
        }
    };
}

define_years!(
    y15 => {
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
    },
    y16 => {
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
    },
    y22 => {
        d1 => D1,
        d2 => D2,
        d3 => D3,
        d4 => D4,
        d5 => D5,
        d6 => D6,
        d7 => D7,
        d8 => D8,
        d9 => D9,
        d10 => D10
    },
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

use crate::util::Day;
use colored::Colorize;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};
use clap::Parser;

#[derive(Serialize)]
struct TimingResult {
    day: usize,
    times: HashMap<String, (String, f64)>,
    year: usize,
}

/// Gets the actual length of a string by stripping the ANSI escape codes.
fn get_length_without_colors(input: &str) -> usize {
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    let clean_input = ansi_regex.replace_all(input, "");
    clean_input.len()
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    year: usize,

    #[arg(short, long, default_value_t = 0)]
    day: usize,
}

fn main() {
    let args = Args::parse();

    let days: Vec<(Box<dyn Day>, &str, &str)> = days_vector!();

    let mut timing_results = Vec::new();

    let mut last_year = 0;
    let mut last_stars = 0;

    let mut total_time: f64 = 0.0;
    let mut total_valid_stars: usize = 0;

    let mut year_times: HashMap<usize, (usize, f64)> = HashMap::new();

    for (day_object, year_name, day_name) in days {
        let year = year_name[1..].parse::<usize>().unwrap();
        let day = day_name[1..].parse::<usize>().unwrap();

        if args.year != 0 && args.year - 2000 != year && args.year != year {
            continue;
        }

        if args.day != 0 && args.day != day {
            continue;
        }

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

        if day_stars > 0 {
            year_times
                .entry(year)
                .and_modify(|(count, time)| {
                    *count += day_stars.min(2);  // no part 3s
                    *time += results
                        .times
                        .values()
                        .filter(|&&(ref _res, t)| t > 0.0)
                        .map(|&(_, t)| t)
                        .sum::<f64>();
                })
                .or_insert((
                    day_stars.min(2),  // no part 3s
                    results
                        .times
                        .values()
                        .filter(|&&(ref _res, t)| t > 0.0)
                        .map(|&(_, t)| t)
                        .sum(),
                ));
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
                        )
                            .bright_black(),
                    );
                } else {
                    let time_string = format!("{:.2?}", Duration::from_secs_f64(*seconds));

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

    println!(
        "{0}\n{1}\n{0}\n",
        "---=======---".bright_black(),
        format!("   Summary   ").bold(),
    );

    if total_time != 0.0 {
        let mut sorted_keys: Vec<usize> = year_times.keys().cloned().collect::<Vec<_>>();
        sorted_keys.sort();

        for key in sorted_keys {
            if let Some(&(count, total)) = year_times.get(&key) {
                println!(
                    "20{}: {} ({})",
                    key,
                    format!("{}", count).bright_yellow().bold(),
                    format!("{:.2?}", Duration::from_secs_f64(total)).yellow()
                );
            }
        }

        let out = format!(
            " all: {} ({})",
            total_valid_stars.to_string().bright_yellow().bold(),
            format!("{:.2?}", Duration::from_secs_f64(total_time)).yellow()
        );

        println!(
            "{}\n{}",
            "-".repeat(get_length_without_colors(&out)).bright_black(),
            out
        );
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
        let days: Vec<(Box<dyn Day>, &str, &str)> = days_vector!();

        for (day_object, year_name, day_name) in days {
            let year = year_name[1..].parse::<usize>().unwrap();
            let day = day_name[1..].parse::<usize>().unwrap();

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
                            day_object.solve_part1(&input),
                            Option::from(expected_output),
                            "{}, {} failed",
                            day_name,
                            part,
                        );
                    } else {
                        assert_eq!(
                            day_object.solve_part2(&input),
                            Option::from(expected_output),
                            "{}, {} failed",
                            day_name,
                            part,
                        );
                    }
                }
            }
        }
    }
}
