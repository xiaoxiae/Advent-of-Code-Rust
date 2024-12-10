use std::cmp::Reverse;
use crate::util::Day;
use std::collections::{BinaryHeap};
use rustc_hash::{FxHashMap as HashMap};

pub struct Day9;

fn build_memory(
    input: &str,
) -> (
    Vec<i64>,
    HashMap<usize, DataField>,
    HashMap<usize, BinaryHeap<Reverse<usize>>>,
) {
    let chars = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let mut memory: Vec<i64> = Vec::new();
    let mut datas: HashMap<usize, DataField> = HashMap::default();
    let mut spaces: HashMap<usize, BinaryHeap<Reverse<usize>>> = HashMap::default();

    // Build memory
    for i in 0..chars.len() {
        let index = i as i64 / 2;
        let size = chars[i];

        let char = match i {
            k if k % 2 == 0 => {
                datas.insert(
                    index as usize,
                    DataField {
                        index: memory.len(),
                        size: size as usize,
                    },
                );

                index
            }
            _ => {
                spaces
                    .entry(size as usize)
                    .or_insert(BinaryHeap::new())
                    .push(Reverse(memory.len()));

                -1
            }
        };

        for _ in 0..size {
            memory.push(char);
        }
    }

    (memory, datas, spaces)
}

#[derive(Debug)]
struct DataField {
    index: usize,
    size: usize,
}

impl Day for Day9 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut memory, _, _) = build_memory(input);

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
        let (_, mut datas, mut spaces) = build_memory(input);

        let mut data_id = datas.len() - 1;

        loop {
            // Find where this could fit
            let data_size = datas[&data_id].size;
            let data_index = datas[&data_id].index;

            // Go through the spaces that could fit this and find the leftmost one that will,
            // as long as it is more left than the current data
            let mut space_size = data_size;

            let mut best_space_size = data_size;
            let mut best_space_index: usize = usize::MAX;

            let largest_space = *spaces.keys().max().unwrap();
            loop {
                if let Some(space_indexes) = spaces.get(&space_size) {
                    if let Some(Reverse(element)) = space_indexes.peek() {
                        if element < &data_index
                            && element < &best_space_index
                        {
                            best_space_index = *element;
                            best_space_size = space_size;
                        }
                    }

                }

                space_size += 1;

                if space_size > largest_space {
                    break;
                }
            }

            // If we found somewhere to place the data
            if best_space_index != usize::MAX {
                datas.get_mut(&data_id).unwrap().index = best_space_index; // Move data
                spaces
                    .get_mut(&best_space_size)
                    .unwrap()
                    .pop();

                // Possibly create new space after the inserted data
                let remainder = best_space_size - data_size;
                if remainder > 0 {
                    spaces
                        .entry(remainder)
                        .or_insert(BinaryHeap::new())
                        .push(Reverse(best_space_index + data_size));
                }

                // We actually don't need to take care of the space we just created after data move,
                // since we are moving stuff from left to right :)
            }

            data_id -= 1;

            if data_id == 0 {
                break;
            }
        }

        let mut total = 0;
        for (id, field) in datas {
            for i in 0..field.size {
                total += (field.index + i) * id;
            }
        }

        Option::from(total.to_string())
    }
}
