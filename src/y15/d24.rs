use crate::util::Day;

pub struct D24;

fn has_balanced_sum(
    numbers: &Vec<usize>,
    selected: &Vec<bool>,
    current_sum: usize,
    desired_sum: usize,
) -> bool {
    if current_sum == desired_sum {
        return true;
    }

    for (i, number) in numbers.iter().enumerate() {
        if !selected[i] && current_sum + number <= desired_sum {
            let mut new_selected = selected.clone();
            new_selected[i] = true;

            if has_balanced_sum(numbers, &new_selected, current_sum + number, desired_sum) {
                return true;
            }
        }
    }

    false
}

fn get_entanglement_group(
    numbers: &Vec<usize>,
    selected: &Vec<bool>,
    current_sum: usize,
    current_count: usize,
    current_entanglement: usize,
    desired_sum: usize,
    result: &mut (usize, usize),
) {
    // if we're over best score, skip
    if current_count > result.0 {
        return;
    }

    if current_sum == desired_sum {
        // got better result
        if has_balanced_sum(numbers, selected, 0, desired_sum) {
            if result.0 > current_count
                || (result.0 == current_count && result.1 > current_entanglement)
            {
                result.0 = current_count;
                result.1 = current_entanglement;
            }
        }

        return;
    }

    // try all options that fit
    for (i, number) in numbers.iter().enumerate() {
        if selected[i] {
            continue;
        }

        if current_sum + number > desired_sum {
            continue;
        }

        // since the numbers are sorted, if we were to use the last number to fill the
        // remaining spots and still be under the sum, skip too
        if (result.0 - current_count) * number + current_sum < desired_sum {
            continue;
        }

        let mut new_selected = selected.clone();
        new_selected[i] = true;

        get_entanglement_group(
            numbers,
            &new_selected,
            current_sum + number,
            current_count + 1,
            current_entanglement * number,
            desired_sum,
            result,
        )
    }
}

fn solve(input: &str, group_size: usize) -> usize {
    let mut weights = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // do in reverse to prioritize large groups
    weights.sort_unstable();
    weights.reverse();

    // (group size, entanglement size), carried though the recursive function
    let mut result = (usize::MAX, usize::MAX);

    get_entanglement_group(
        &weights,
        &vec![false; weights.len()],
        0,
        0,
        1,
        weights.iter().sum::<usize>() / group_size,
        &mut result,
    );

    result.1
}

impl Day for D24 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 3).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input, 4).to_string())
    }
}
