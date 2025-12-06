use crate::util::Day;

pub struct D5;

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let mut ingredients = ingredients_str
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let ranges = ranges_str
        .lines()
        .map(|l| {
            let (from, to) = l.split_once("-").unwrap();
            (from.parse::<usize>().unwrap(), to.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    return (ranges, ingredients);
}

impl Day for D5 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (ranges, ingredients) = parse(input);

        let mut fresh = 0;

        for ingredient in ingredients.iter() {
            for (from, to) in ranges.iter() {
                if from <= ingredient && ingredient <= to {
                    fresh += 1;
                    break;
                }
            }
        }

        Option::from(fresh.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let (mut ranges, _) = parse(input);

        let mut merged = true;
        while merged {
            let mut i = 0;
            merged = false;

            while i < ranges.len() {
                let mut j = i + 1;

                while j < ranges.len() {
                    let (a, b) = ranges[i];
                    let (an, bn) = ranges[j];

                    if (a <= an && an <= b)
                        || (a <= bn && bn <= b)
                        || (an <= a && b <= bn)
                        || (a <= an && bn <= b)
                    {
                        ranges[i] = (a.min(an), b.max(bn));
                        ranges.remove(j);
                        merged = true;
                    }

                    j += 1;
                }

                i += 1;
            }
        }

        let mut fresh = 0;

        for (a, b) in ranges.iter() {
            fresh += b - a + 1;
        }

        Option::from(fresh.to_string())
    }
}
