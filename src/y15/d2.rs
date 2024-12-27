use crate::util::Day;

pub struct D2;

impl Day for D2 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut total = 0;
        for line in input.lines() {
            let dimensions: Vec<usize> = line
                .split('x')
                .filter_map(|x| x.parse().ok())
                .collect();

            if let [a, b, c] = dimensions.as_slice() {
                let surface_area = 2 * (a * b + b * c + c * a);
                let slack = (a * b).min((b * c)).min((c * a));
                total += surface_area + slack;
            }
        }
        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut total = 0;
        for line in input.lines() {
            let dimensions: Vec<usize> = line
                .split('x')
                .filter_map(|x| x.parse().ok())
                .collect();

            if let [a, b, c] = dimensions.as_slice() {
                let perimeter = 2 * (a + b).min(b + c).min(c + a);
                let volume = a * b * c;
                total += perimeter + volume;
            }
        }
        Some(total.to_string())
    }
}
