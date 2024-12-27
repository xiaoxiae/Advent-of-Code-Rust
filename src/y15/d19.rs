use crate::util::Day;
use std::collections::{HashMap, HashSet};

pub struct D19;

fn neighbours(molecule: &String, replacements: &Vec<(String, String)>) -> Vec<String> {
    let mut molecules = HashSet::new();

    for (from, to) in replacements {
        let mut start_pos = 0;

        while let Some(pos) = molecule[start_pos..].find(&*from) {
            let pos = pos + start_pos;
            let mut new_molecule = molecule.clone();
            new_molecule.replace_range(pos..pos + from.len(), &*to);

            molecules.insert(new_molecule);
            start_pos = pos + 1;
        }
    }

    molecules.into_iter().collect()
}

fn parse(input: &str) -> (Vec<(String, String)>, String) {
    let (replacement_str, molecule) = input.trim().split_once("\n\n").unwrap();
    let molecule = molecule.to_string();

    let mut replacements: Vec<(String, String)> = Vec::new();
    for replacement in replacement_str.lines() {
        let (left, right) = replacement.split_once(" => ").unwrap();
        replacements.push((left.to_string(), right.to_string()));
    }

    (replacements, molecule)
}


fn devolve(molecule: String, replacements: &Vec<(String, String)>, cache: &mut HashMap<String, usize>) -> usize {
    if molecule == "e" {
        return 0;
    }

    if cache.contains_key(&molecule) {
        return cache[&molecule];
    }

    let mut min_depth = usize::MAX;
    for neighbour in neighbours(&molecule, &replacements) {
        let result = devolve(neighbour, &replacements, cache);

        // looking through the input, it looks pretty linear so we just assume it's unique
        if result != usize::MAX {
            min_depth = 1 + min_depth.min(result);
            break;
        }
    }

    if min_depth != usize::MAX {
        cache.insert(molecule, min_depth);
    }

    min_depth
}


impl Day for D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (replacements, molecule) = parse(input);

        let molecules = neighbours(&molecule, &replacements);

        Option::from(molecules.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        None

        // TODO: this sometimes go into an infinite loop -- fix me!

        // let (replacements, molecule) = parse(input);

        // let mut inverse_replacements: Vec<(String, String)> = vec![];
        // for (from, to) in &replacements {
        //     inverse_replacements.push((to.to_string(), from.to_string()));
        // }

        // let mut cache = HashMap::new();

        // let distance: usize = devolve(molecule, &inverse_replacements, &mut cache);

        // Option::from(distance.to_string())
    }
}
