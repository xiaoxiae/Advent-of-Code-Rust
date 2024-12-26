use rayon::prelude::*;
use crate::util::Day;
use itertools::Itertools;
use rand::RngCore;
use regex::Regex;
use rustc_hash::FxHashMap as HashMap;
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use dashmap::DashMap;

pub struct Y24D24;

#[derive(Debug, Clone)]
enum CommandType {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Command {
    left: u32,
    command: CommandType,
    right: u32,
}

fn string_to_identifier(str: String) -> u32 {
    let bytes = str.as_bytes();
    let mut result = 0u32;

    for (i, &byte) in bytes.iter().take(4).enumerate() {
        result |= (byte as u32) << (8 * (3 - i));
    }

    result
}

fn identifier_to_string(id: u32) -> String {
    let mut chars = Vec::new();

    for i in (0..4).rev() {
        let byte = ((id >> (8 * i)) & 0xFF) as u8;

        if byte != 0 {
            chars.push(byte as char);
        }
    }

    chars.iter().collect()
}

fn parse(input: &str) -> (HashMap<u32, bool>, HashMap<u32, Command>) {
    let (initial, connections) = input.split_once("\n\n").unwrap();

    let initial_re = Regex::new(r"(\w+): (\d+)").unwrap();
    let connections_re = Regex::new(r"(\w+) (OR|XOR|AND) (\w+) -> (\w+)").unwrap();

    let mut wires: HashMap<u32, bool> = HashMap::default();

    initial.lines().for_each(|line| {
        let captures = initial_re.captures(line).unwrap();
        wires.insert(
            string_to_identifier(captures[1].to_string()),
            captures[2].parse::<u8>().unwrap() != 0,
        );
    });

    let mut commands: HashMap<u32, Command> = HashMap::default();

    connections.lines().for_each(|line| {
        let captures = connections_re.captures(line).unwrap();

        let left = string_to_identifier(captures[1].to_string());
        let right = string_to_identifier(captures[3].to_string());

        let command = match captures[2].to_string().as_str() {
            "AND" => CommandType::AND,
            "OR" => CommandType::OR,
            "XOR" => CommandType::XOR,
            _ => panic!("Unknown command!"),
        };

        let command = Command {
            left,
            command,
            right,
        };

        commands.insert(string_to_identifier(captures[4].to_string()), command);
    });

    (wires, commands)
}

fn _simulate(wire: u32, wires: &mut HashMap<u32, bool>, commands: &HashMap<u32, Command>) {
    if wires.contains_key(&wire) {
        return;
    }

    let command = commands.get(&wire).unwrap();

    // this is to prevent infinite cycles when rewiring
    //
    // for part 1, this doesn't matter
    // for part 2, we're doing heuristic shit anyway so it doesn't matter either
    wires.insert(wire, false);

    _simulate(command.left, wires, commands);
    _simulate(command.right, wires, commands);

    let result = match command.command {
        CommandType::AND => wires[&command.left] && wires[&command.right],
        CommandType::OR => wires[&command.left] || wires[&command.right],
        CommandType::XOR => wires[&command.left] != wires[&command.right],
    };

    wires.insert(wire, result);
}

fn simulate(wires: &mut HashMap<u32, bool>, commands: &HashMap<u32, Command>) {
    for wire in commands.keys() {
        _simulate(*wire, wires, &commands);
    }
}

fn combine(prefix: char, wires: &HashMap<u32, bool>) -> usize {
    let mut number: usize = 0;

    let mut w = wires.keys().cloned().collect::<Vec<_>>();
    w.sort_unstable();
    w.reverse();

    for key in w {
        if (key >> 24) as u8 == prefix as u8 {
            number = (number << 1) | (if wires[&key] { 1 } else { 0 });
        }
    }

    number
}

impl Day for Y24D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (mut wires, commands) = parse(input);

        simulate(&mut wires, &commands);

        let mut keys = wires.keys().cloned().collect::<Vec<_>>();
        keys.sort_unstable();
        keys.reverse();

        let number = combine('z', &wires);

        Option::from(number.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut wires, mut commands) = parse(input);

        // nullify all inputs
        for wire in wires.values_mut() {
            *wire = false;
        }

        // remember swaps that we did to fix the circuit
        let swaps = DashMap::new(); // Thread-safe HashMap

        let wire_keys: Vec<_> = wires.keys().cloned().collect();

        // try adding only individual bits of x with zeroed y
        wire_keys.par_iter().for_each(|wire| {
            if (*wire >> 24) as u8 != 'x' as u8 {
                return;
            }

            // Check that there was an error
            let mut single_wires = wires.clone();
            single_wires.insert(*wire, true);

            simulate(&mut single_wires, &commands);

            if combine('x', &single_wires) == combine('z', &single_wires) {
                return;
            }

            let keys = commands.keys().cloned().collect::<Vec<_>>();

            for k1 in &keys {
                for k2 in &keys {
                    if !(k1 < k2) {
                        continue;
                    }

                    // Only swap wires that make a difference
                    if single_wires[k1] == single_wires[k2] {
                        continue;
                    }

                    let mut swapped_commands = commands.clone();

                    let a = swapped_commands.get_mut(k1).unwrap() as *mut Command;
                    let b = swapped_commands.get_mut(k2).unwrap() as *mut Command;
                    unsafe {
                        std::ptr::swap(a, b);
                    }

                    let mut swapped_wires = wires.clone();
                    swapped_wires.insert(*wire, true);

                    simulate(&mut swapped_wires, &swapped_commands);

                    if combine('x', &swapped_wires) == combine('z', &swapped_wires) {
                        swaps.entry((*k1, *k2)).or_insert_with(|| AtomicUsize::new(0)).fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        });

        let swaps: HashMap<_, _> = swaps
            .into_iter()
            .map(|(key, value)| (key, value.load(Ordering::Relaxed)))
            .collect();

        let mut values: Vec<_> = swaps.iter().map(|(_, v)| v).collect();
        values.sort_unstable();

        // this now presumably gives candidates for swaps
        let candidates = swaps
            .iter()
            .filter_map(|((i, j), v)| {
                if v == *values.iter().max().unwrap() {
                    Some((*i, *j))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        candidates.into_iter().combinations(4)
            .collect::<Vec<Vec<_>>>()
            .par_iter().find_map_any(|combinations|
        {
            let mut seen_values: HashSet<u32> = HashSet::default();

            let unique = combinations
                .iter()
                .flat_map(|&(a, b)| vec![a, b])
                .all(|value| seen_values.insert(value));

            if !unique {
                return None;
            }

            let mut rng = rand::thread_rng();
            let mut error = false;

            let mut swapped_commands = commands.clone();

            for (k1, k2) in combinations {
                let a = swapped_commands.get_mut(k1).unwrap() as *mut Command;
                let b = swapped_commands.get_mut(k2).unwrap() as *mut Command;
                unsafe {
                    std::ptr::swap(a, b);
                }
            }

            for _ in 0..100 {
                let mut x: usize = rng.next_u64() as usize;
                let mut y: usize = rng.next_u64() as usize;

                x &= (1 << 44) - 1;
                y &= (1 << 44) - 1;

                let orig_z = x + y;

                let mut swapped_wires = wires.clone();

                let mut w = wires.keys().cloned().collect::<Vec<_>>();
                w.sort_unstable();

                for wire in w {
                    if (wire >> 24) as u8 == 'x' as u8 {
                        swapped_wires.insert(wire, (x & 1) == 1);
                        x >>= 1;
                    } else if (wire >> 24) as u8 == 'y' as u8 {
                        swapped_wires.insert(wire, (y & 1) == 1);
                        y >>= 1;
                    }
                }

                simulate(&mut swapped_wires, &swapped_commands);

                let mut w = swapped_wires.keys().cloned().collect::<Vec<_>>();
                w.sort_unstable();
                w.reverse();

                let mut z = 0;

                for wire in w {
                    if (wire >> 24) as u8 == 'z' as u8 {
                        z = (z << 1) | (if swapped_wires[&wire] { 1 } else { 0 });
                    }
                }

                if orig_z != z {
                    error = true;
                    break;
                }
            }

            if !error {
                let mut parts = combinations
                    .iter()
                    .flat_map(|(a, b)| [a, b]) // Flatten each tuple into individual strings
                    .collect::<Vec<_>>();

                parts.sort_unstable();

                return Option::from(parts.into_iter().map(|v| identifier_to_string(*v)).join(","));
            }
            
            None
        })
    }
}
