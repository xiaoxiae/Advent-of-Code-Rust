use crate::util::Day;
use regex::Regex;

pub struct D15;

static TEASPOONS: usize = 100;
static REQUIRED_CALORIES: usize = 500;


fn parse(input: &str) -> Vec<Vec<isize>> {
    let re = Regex::new(r"([+-]?\d+)").unwrap();

    input
        .lines()
        .map(|line| {
            re.find_iter(line)
                .filter_map(|mat| mat.as_str().parse::<isize>().ok())
                .collect::<Vec<isize>>()
        })
        .collect::<Vec<Vec<isize>>>()
}

fn get_best_cookie(
    teaspoons: usize,
    ingredient_index: usize,
    ingredient_split: &mut Vec<usize>,
    ingredients: &Vec<Vec<isize>>,
    required_calories: Option<usize>,
) -> isize {
    if ingredient_index == ingredients.len() {
        if teaspoons != 0 {
            return 0;
        }

        let mut total = 1;

        for i in 0..ingredients[0].len() {
            let mut current = 0;

            for j in 0..ingredients.len() {
                current += ingredients[j][i] * (ingredient_split[j] as isize);
            }

            // skip calories
            if i == ingredients[0].len() - 1 {
                // possibly require a specific amount
                if let Some(c) = required_calories {
                    if current != (c as isize) {
                        return 0;
                    }
                }

                continue;
            }

            if current <= 0 {
                return 0;
            }

            total *= current;
        }

        return total;
    }

    let mut best = 0;
    for t in 0..=teaspoons {
        ingredient_split[ingredient_index] = t;

        best = best.max(get_best_cookie(
            teaspoons - t,
            ingredient_index + 1,
            ingredient_split,
            ingredients,
            required_calories,
        ));
    }

    best
}

impl Day for D15 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let ingredients = parse(input);

        let mut ingredient_splits = vec![0; ingredients.len()];
        let max_ingredients = get_best_cookie(TEASPOONS, 0, &mut ingredient_splits, &ingredients, None);

        Option::from(max_ingredients.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let ingredients = parse(input);

        let mut ingredient_splits = vec![0; ingredients.len()];
        let max_ingredients = get_best_cookie(
            TEASPOONS,
            0,
            &mut ingredient_splits,
            &ingredients,
            Option::from(REQUIRED_CALORIES),
        );

        Option::from(max_ingredients.to_string())
    }
}
