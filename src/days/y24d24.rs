use crate::util::Day;
use itertools::Itertools;
use rand::RngCore;
use regex::Regex;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub struct Y24D24;

#[derive(Debug, Clone)]
struct Command {
    left: String,
    command: String,
    right: String,
}

// fn string_to_identifier(str: String) -> u32 {
//     // TODO: string is at most 4 ASCII characters, convert them into bytes
// }

fn parse(input: &str) -> (HashMap<String, bool>, HashMap<String, Command>) {
    let (initial, connections) = input.split_once("\n\n").unwrap();

    let initial_re = Regex::new(r"(\w+): (\d+)").unwrap();
    let connections_re = Regex::new(r"(\w+) (OR|XOR|AND) (\w+) -> (\w+)").unwrap();

    let mut wires: HashMap<String, bool> = HashMap::default();

    initial.lines().for_each(|line| {
        let captures = initial_re.captures(line).unwrap();
        wires.insert(
            captures[1].to_string(),
            captures[2].parse::<u8>().unwrap() != 0,
        );
    });

    let mut commands: HashMap<String, Command> = HashMap::default();

    connections.lines().for_each(|line| {
        let captures = connections_re.captures(line).unwrap();

        let command = Command {
            left: captures[1].to_string(),
            command: captures[2].to_string(),
            right: captures[3].to_string(),
        };

        commands.insert(captures[4].to_string(), command);
    });

    (wires, commands)
}

fn _simulate(
    wire: &String,
    wires: &mut HashMap<String, bool>,
    commands: &HashMap<String, Command>,
) {
    if wires.contains_key(wire) {
        return;
    }

    let command = commands.get(wire).unwrap();

    // this is to prevent infinite cycles
    // for part 1, this doesn't matter
    // for part 2, we're doing heuristic shit anyway so it doesn't matter
    wires.insert(wire.clone(), false);

    _simulate(&command.left, wires, commands);
    _simulate(&command.right, wires, commands);

    let result = match command.command.as_str() {
        "AND" => wires[&command.left] && wires[&command.right],
        "OR" => wires[&command.left] || wires[&command.right],
        "XOR" => wires[&command.left] != wires[&command.right],
        _ => panic!("Unknown command '{}'!", command.command),
    };

    wires.insert(wire.clone(), result);
}

fn simulate(wires: &mut HashMap<String, bool>, commands: &HashMap<String, Command>) {
    for wire in commands.keys() {
        _simulate(wire, wires, &commands);
    }
}

fn combine(prefix: char, wires: &HashMap<String, bool>) -> usize {
    let mut number: usize = 0;

    let mut w = wires.keys().cloned().collect::<Vec<_>>();
    w.sort_unstable();
    w.reverse();

    for key in w {
        if key.chars().nth(0).unwrap() == prefix {
            // println!("{} -> {}", key, wires[&key]);
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

        let mut number: usize = 0;
        for key in keys {
            if key.chars().nth(0).unwrap() == 'z' {
                number = (number << 1) | (if wires[&key] { 1 } else { 0 });
            }
        }

        Option::from(number.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut wires, commands) = parse(input);

        // nullify all inputs
        for wire in wires.values_mut() {
            *wire = false;
        }

        let mut swaps: HashMap<(String, String), usize> = HashMap::default();

        for prefix in ['x'] {
            let mut w = wires.keys().cloned().collect::<Vec<_>>();
            w.sort_unstable();

            let mut new_wires = wires.clone();
            for wire in w {
                if wire.chars().nth(0).unwrap() != prefix {
                    continue;
                }

                // first check that there even was an error
                let mut tmp = new_wires.clone();
                tmp.insert(wire.to_string(), true);

                simulate(&mut tmp, &commands);

                // println!("--- {} ---", wire);

                if combine(prefix, &tmp) == combine('z', &tmp) {
                    continue;
                }

                // println!("{} {}", combine(prefix, &new_wires), combine('z', &new_wires));
                // panic!();

                let mut keys = commands.keys().cloned().collect::<Vec<_>>();
                keys.sort_unstable();

                let mut total = 0;

                for k1 in &keys {
                    for k2 in &keys {
                        if !(k1 < k2) {
                            continue;
                        }

                        // only swap wires that make a difference
                        if tmp[k1] == tmp[k2] {
                            continue;
                        }

                        let mut new_commands = commands.clone();

                        if let (Some(value1), Some(value2)) =
                            (new_commands.get(k1).cloned(), new_commands.get(k2).cloned())
                        {
                            new_commands.insert(k1.to_string(), value2);
                            new_commands.insert(k2.to_string(), value1);
                        }

                        let mut newest_wires = new_wires.clone();
                        newest_wires.insert(wire.to_string(), true);

                        simulate(&mut newest_wires, &new_commands);

                        if combine(prefix, &newest_wires) == combine('z', &newest_wires) {
                            *swaps.entry((k1.to_string(), k2.to_string())).or_insert(0) += 1;
                            total += 1;
                        }
                    }
                }
            }
        }

        let mut values: Vec<_> = swaps.iter().map(|(_, v)| v).collect();
        values.sort_unstable();

        // this now presumably gives candidates for swaps
        let candidates = swaps
            .iter()
            .filter_map(|(k, v)| {
                if v == *values.iter().max().unwrap() {
                    Some(k)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for combinations in candidates.iter().combinations(4) {
            let mut seen_values = HashSet::default();

            let unique = combinations
                .iter()
                .flat_map(|&(a, b)| vec![a, b])
                .all(|value| seen_values.insert(value));

            if !unique {
                continue;
            }

            let mut rng = rand::thread_rng();
            let mut error = false;

            for _ in 0..100 {
                let mut x: usize = rng.next_u64() as usize;
                let mut y: usize = rng.next_u64() as usize;

                x &= (1 << 44) - 1;
                y &= (1 << 44) - 1;

                let orig_z = x + y;

                let mut newest_wires = wires.clone();

                let mut w = wires.keys().cloned().collect::<Vec<_>>();
                w.sort_unstable();

                for key in w {
                    if key.chars().nth(0).unwrap() == 'x' {
                        newest_wires.insert(key, (x & 1) == 1);
                        x >>= 1;
                    } else if key.chars().nth(0).unwrap() == 'y' {
                        newest_wires.insert(key, (y & 1) == 1);
                        y >>= 1;
                    }
                }

                let mut new_commands = commands.clone();
                for (k1, k2) in &combinations {
                    if let (Some(value1), Some(value2)) =
                        (new_commands.get(k1).cloned(), new_commands.get(k2).cloned())
                    {
                        new_commands.insert(k1.to_string(), value2);
                        new_commands.insert(k2.to_string(), value1);
                    }
                }

                simulate(&mut newest_wires, &new_commands);

                let mut w = newest_wires.keys().cloned().collect::<Vec<_>>();
                w.sort_unstable();
                w.reverse();

                let mut z = 0;

                for key in w {
                    if key.chars().nth(0).unwrap() == 'z' {
                        z = (z << 1) | (if newest_wires[&key] { 1 } else { 0 });
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
                    .flat_map(|(a, b)| [a.clone(), b.clone()]) // Flatten each tuple into individual strings
                    .collect::<Vec<_>>();

                parts.sort_unstable();

                return Option::from(parts.join(","));
            }
        }

        None
    }
}
