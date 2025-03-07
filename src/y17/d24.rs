use crate::util::Day;

pub struct D24;

fn get_strong(bridges: &Vec<(usize, usize)>, collected: usize, last_number: usize) -> usize {
    let mut best = 0;

    // try to connect all unused bridges
    for (i, bridge) in bridges.iter().enumerate() {
        // used...
        if (collected & (1 << i)) != 0 {
            continue;
        }

        // either this is the last one, or we connect more
        let mut result = 0;
        if bridge.0 == last_number {
            result = bridge.0 + bridge.1 + get_strong(bridges, collected | (1 << i), bridge.1);
        } else if bridge.1 == last_number {
            result = bridge.0 + bridge.1 + get_strong(bridges, collected | (1 << i), bridge.0);
        }

        if result > best {
            best = result;
        }
    }

    best
}

fn get_long(bridges: &Vec<(usize, usize)>, collected: usize, last_number: usize) -> (usize, usize) {
    let mut best = (0, 0);

    for (i, bridge) in bridges.iter().enumerate() {
        // used...
        if (collected & (1 << i)) != 0 {
            continue;
        }

        let mut result = (0, 0);

        if bridge.0 == last_number {
            result = get_long(bridges, collected | (1 << i), bridge.1);

            result.0 += 1;
            result.1 += bridge.0 + bridge.1;
        } else if bridge.1 == last_number {
            result = get_long(bridges, collected | (1 << i), bridge.0);

            result.0 += 1;
            result.1 += bridge.0 + bridge.1;
        }

        if result.0 > best.0 || (result.0 == best.0 && result.1 > best.1) {
            best = result;
        }
    }

    best
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut bridges = vec![];

    for line in input.lines() {
        let parts = line.split('/').collect::<Vec<_>>();

        bridges.push((
            parts[0].parse::<usize>().unwrap(),
            parts[1].parse::<usize>().unwrap(),
        ))
    }
    
    bridges
}

impl Day for D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let bridges = parse(input);

        Option::from(get_strong(&bridges, 0, 0).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let bridges = parse(input);

        Option::from(get_long(&bridges, 0, 0).1.to_string())
    }
}
