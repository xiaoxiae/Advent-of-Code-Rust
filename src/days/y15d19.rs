use crate::util::Day;
use std::collections::HashSet;

pub struct Y15D19;

// fn generate_replacements(
//     current: String,
//     replacements: &HashMap<String, String>,
//     generated: &mut HashSet<String>,
// ) {
//     for (from, to) in replacements.iter() {
//         if current.starts_with(from) {
//             j
//         }
//     }
// }

fn neighbours(molecule: String, replacements: &Vec<(String, String)>) -> Vec<String> {
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


fn devolve(molecule: String, depth: usize, replacements: &Vec<(String, String)>) -> usize {
    if molecule == "e" {
        return depth;
    }

    let mut min_depth = usize::MAX;
    for neighbour in neighbours(molecule, &replacements) {
        min_depth = min_depth.min(devolve(neighbour, depth + 1, &replacements));

        // looking through the input, it looks pretty linear so we just assume it's unique
        if min_depth != usize::MAX {
            break;
        }
    }

    min_depth
}


impl Day for Y15D19 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (replacements, molecule) = parse(input);

        let molecules = neighbours(molecule, &replacements);

        Option::from(molecules.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (replacements, molecule) = parse(input);

        let mut inverse_replacements: Vec<(String, String)> = vec![];
        for (from, to) in &replacements {
            inverse_replacements.push((to.to_string(), from.to_string()));
        }

        let distance: usize = devolve(molecule, 0, &inverse_replacements);

        Option::from(distance.to_string())
    }
}
