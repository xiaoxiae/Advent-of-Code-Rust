//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2019/tree/master/14
use crate::util::Day;
use rustc_hash::FxHashMap;

pub struct D14;

struct Resource {
    quantity: i64,
    requires: Vec<(String, i64)>,
}

fn parse_value(value: &str) -> (String, i64) {
    let mut it = value.trim().split(' ');
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    (b.to_string(), a.parse::<i64>().unwrap())
}

fn parse(input: &str) -> FxHashMap<String, Resource> {
    let mut resources: FxHashMap<String, Resource> = FxHashMap::default();

    for line in input.trim().lines() {
        let mut parts = line.split(" => ");
        let l = parts.next().unwrap();
        let r = parts.next().unwrap();

        let (chemical, quantity) = parse_value(r);
        let requires = l.split(", ").map(parse_value).collect();
        resources.insert(chemical, Resource { quantity, requires });
    }

    resources
}

fn get_cost(
    resources: &FxHashMap<String, Resource>,
    byproducts: &mut FxHashMap<String, i64>,
    chemical: &str,
    quantity: i64,
) -> i64 {
    let resource = &resources[chemical];

    // calculate the minimum allowed quantity + the leftovers
    let quantity_coefficient = (quantity + resource.quantity - 1) / resource.quantity;
    *byproducts.entry(chemical.to_string()).or_insert(0) +=
        quantity_coefficient * resource.quantity - quantity;

    // base condition -- ores
    if resource.requires[0].0 == "ORE" {
        return resource.requires[0].1 * quantity_coefficient;
    }

    let mut cost = 0;
    for (r_chemical, r_quantity) in &resource.requires {
        let needed = r_quantity * quantity_coefficient;
        let available = *byproducts.get(r_chemical).unwrap_or(&0);
        if available >= needed {
            *byproducts.entry(r_chemical.clone()).or_insert(0) -= needed;
        } else {
            let r_quantity = needed - available;
            byproducts.insert(r_chemical.clone(), 0);
            cost += get_cost(resources, byproducts, r_chemical, r_quantity);
        }
    }

    cost
}

impl Day for D14 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let resources = parse(input);
        let mut byproducts: FxHashMap<String, i64> = FxHashMap::default();
        let answer = get_cost(&resources, &mut byproducts, "FUEL", 1);
        Some(answer.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let resources = parse(input);

        let input_size: i64 = 1_000_000_000_000;
        let mut lo: i64 = 0;
        let mut hi: i64 = input_size;

        while lo < hi {
            let avg = (lo + hi) / 2;
            let mut byproducts: FxHashMap<String, i64> = FxHashMap::default();
            if get_cost(&resources, &mut byproducts, "FUEL", avg) < input_size {
                lo = avg + 1;
            } else {
                hi = avg;
            }
        }

        let answer = lo - 1;
        Some(answer.to_string())
    }
}
