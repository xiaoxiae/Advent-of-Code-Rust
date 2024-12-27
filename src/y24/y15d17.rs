use crate::util::Day;

pub struct Y15D17;

static LITERS: usize = 150;

fn min_required_containers(
    remaining: usize,
    container_count: usize,
    index: usize,
    containers: &Vec<usize>,
) -> usize {
    if remaining == 0 {
        return container_count;
    }

    let mut min = usize::MAX;

    for i in index..containers.len() {
        let container = containers[i];

        if remaining >= container {
            min = min.min(min_required_containers(
                remaining - container,
                container_count + 1,
                i + 1,
                containers,
            ));
        }
    }

    min
}

fn count_with_containers(
    remaining: usize,
    container_count: usize,
    index: usize,
    containers: &Vec<usize>,
    max_container_count: Option<usize>,
) -> usize {
    if remaining == 0 {
        return match max_container_count {
            Some(c) if c == container_count => 1,
            None => 1,
            _ => 0,
        };
    }

    let mut total = 0;

    for i in index..containers.len() {
        let container = containers[i];

        if remaining >= container {
            total += count_with_containers(
                remaining - container,
                container_count + 1,
                i + 1,
                containers,
                max_container_count,
            );
        }
    }

    total
}

impl Day for Y15D17 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let containers: Vec<_> = input
            .lines()
            .map(|line| line.trim().parse::<usize>().unwrap())
            .collect();

        let combinations = count_with_containers(LITERS, 0, 0, &containers, None);

        Option::from(combinations.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let containers: Vec<_> = input
            .lines()
            .map(|line| line.trim().parse::<usize>().unwrap())
            .collect();

        let min_containers = min_required_containers(LITERS, 0, 0, &containers);

        let combinations =
            count_with_containers(LITERS, 0, 0, &containers, Option::from(min_containers));

        Option::from(combinations.to_string())
    }
}
