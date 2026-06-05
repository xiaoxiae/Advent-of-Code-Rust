//! Auto-translated from the original Python solution by an LLM (Claude Code).
//! Not yet hand-reviewed or rewritten for idiomatic Rust / performance.
//! Original: https://github.com/xiaoxiae/Advent-of-Code-2020/tree/master/21
use crate::util::Day;
use rustc_hash::FxHashSet;

pub struct D21;

fn parse(input: &str) -> (Vec<(Vec<String>, Vec<String>)>, FxHashSet<String>, FxHashSet<String>) {
    let mut pairings: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    let mut foods: FxHashSet<String> = FxHashSet::default();
    let mut alergens: FxHashSet<String> = FxHashSet::default();

    for line in input.trim().lines() {
        let (ingredients, alg) = line.split_once(" (contains ").unwrap();
        let ing: Vec<String> = ingredients.split_whitespace().map(|s| s.to_string()).collect();
        let alg: Vec<String> = alg
            .trim_end_matches(')')
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        for f in &ing {
            foods.insert(f.clone());
        }
        for a in &alg {
            alergens.insert(a.clone());
        }
        pairings.push((ing, alg));
    }

    (pairings, foods, alergens)
}

impl Day for D21 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (pairings, foods, alergens) = parse(input);

        let mut impossible_foods: FxHashSet<String> = foods.clone();

        for alergen in &alergens {
            let mut possible_foods: FxHashSet<String> = foods.clone();

            for (f, a) in &pairings {
                if a.contains(alergen) {
                    for food in &foods {
                        if !f.contains(food) {
                            possible_foods.remove(food);
                        }
                    }
                }
            }

            for food in &possible_foods {
                impossible_foods.remove(food);
            }
        }

        let mut total = 0usize;
        for food in &impossible_foods {
            for (f, _) in &pairings {
                total += f.iter().filter(|x| *x == food).count();
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (pairings, mut foods, mut alergens) = parse(input);

        let mut canonical: Vec<(String, String)> = Vec::new();

        while !alergens.is_empty() {
            let mut to_process: Option<String> = None;
            let mut found_food: Option<String> = None;

            for alergen in &alergens {
                let mut possible_foods: FxHashSet<String> = foods.clone();

                for (f, a) in &pairings {
                    if a.contains(alergen) {
                        for food in &foods {
                            if !f.contains(food) {
                                possible_foods.remove(food);
                            }
                        }
                    }
                }

                if possible_foods.len() == 1 {
                    to_process = Some(alergen.clone());
                    found_food = Some(possible_foods.into_iter().next().unwrap());
                    break;
                }
            }

            if let (Some(alergen), Some(food)) = (to_process, found_food) {
                canonical.push((food.clone(), alergen.clone()));
                alergens.remove(&alergen);
                foods.remove(&food);
            } else {
                break;
            }
        }

        canonical.sort_by(|x, y| x.1.cmp(&y.1));

        Some(
            canonical
                .iter()
                .map(|(a, _)| a.as_str())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}
