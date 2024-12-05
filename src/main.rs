mod util;

mod days { pub mod day1; pub mod day2; pub mod day3; pub mod day4; pub mod day5; }

use std::time::Instant;
use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::util::Day;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
    ];

    for (i, day) in days.iter().enumerate() {
        let day_number = i + 1;
        let input_file = format!("data/day{}/input.in", day_number);
        let input = std::fs::read_to_string(&input_file)
            .expect(&format!("Failed to read input file: {}", input_file));

        println!("--- Day {} ---", day_number);
        let start = Instant::now();
        let part1_result = day.solve_part1(&input);
        let duration_part1 = start.elapsed();
        println!("Part 1: {} (took {:.2?})", part1_result, duration_part1);

        let start = Instant::now();
        let part2_result = day.solve_part2(&input);
        let duration_part2 = start.elapsed();
        println!("Part 2: {} (took {:.2?})\n", part2_result, duration_part2);
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

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
        ];

        for (day, day_name) in days {
            for part in ["part1", "part2"] {
                let samples = find_samples(format!("data/{}/{}", day_name, part).as_str());

                println!("Testing {}/{}", day_name, part);

                for (input_file, output_file) in &samples {
                    let input = fs::read_to_string(input_file)
                        .expect(&format!("Failed to read input file: {}", input_file));

                    let expected_output = fs::read_to_string(output_file)
                        .expect(&format!(
                            "Failed to read output file: {}",
                            output_file
                        ))
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
