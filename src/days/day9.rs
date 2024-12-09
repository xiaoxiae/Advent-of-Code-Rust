use crate::util::Day;
use std::collections::HashMap;

pub struct Day9;

fn build_memory(input: &str) -> (Vec<i64>, HashMap<usize, usize>) {
    let chars = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let mut memory: Vec<i64> = Vec::new();
    let mut sizes: HashMap<usize, usize> = HashMap::new();

    // Build memory
    for i in 0..chars.len() {
        let index = i as i64 / 2;
        let size = chars[i];

        let char = match i {
            k if k % 2 == 0 => {
                sizes.insert(index as usize, size as usize);

                index
            }
            _ => -1,
        };

        for _ in 0..size {
            memory.push(char);
        }
    }

    (memory, sizes)
}

impl Day for Day9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut memory, _) = build_memory(input);

        // Compact
        let mut space_idx = 0;
        let mut data_idx = memory.len() - 1;

        loop {
            // Find first empty space
            while space_idx <= data_idx && memory[space_idx] != -1 {
                space_idx += 1;
            }

            // And last non-empty space
            while space_idx <= data_idx && memory[data_idx] == -1 {
                data_idx -= 1;
            }

            if space_idx > data_idx {
                break;
            }

            memory.swap(space_idx, data_idx);
        }

        let mut total = 0;
        for (i, v) in memory.iter().enumerate() {
            if *v == -1 {
                break;
            }

            total += i as i64 * v;
        }

        Option::from(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut memory, sizes) = build_memory(input);

        // Compact
        let mut data_id = sizes.keys().len() - 1;
        let mut data_idx = memory.len() - 1;

        loop {
            // Find end of the next data segment
            while memory[data_idx] as usize != data_id {
                data_idx -= 1;
            }

            // Look for space
            let data_size = sizes[&data_id];
            let mut space_idx = 0;
            loop {
                match memory.get(space_idx) {
                    None => break,
                    Some(-1) => {
                        let mut space_size = 0;

                        while space_idx < data_idx && memory[space_idx] == -1 {
                            space_size += 1;
                            space_idx += 1;
                        }

                        if space_idx >= data_idx {
                            break;
                        }

                        // Swap all elements
                        if space_size >= data_size {
                            for i in 0..data_size {
                                memory.swap(
                                    data_idx - data_size + i + 1, // non-inclusive
                                    space_idx - space_size + i,
                                );
                            }

                            break;
                        }
                    }
                    Some(value) => {
                        space_idx += sizes[&(*value as usize)];
                        continue;
                    },
                }
            }

            data_id -= 1;
            if data_id == 0 {
                break;
            }
        }

        let mut total = 0;
        for (i, v) in memory.iter().enumerate() {
            if *v == -1 {
                continue;
            }

            total += i as i64 * v;
        }

        Option::from(total.to_string())
    }
}
