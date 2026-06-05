use crate::util::Day;

pub struct D12;

type Shape = (Vec<Vec<bool>>, usize);
type Tree = (usize, usize, Vec<usize>);

fn parse(input: &str) -> (Vec<Shape>, Vec<Tree>) {
    let mut blocks: Vec<&str> = input.trim().split("\n\n").collect();
    let trees_part = blocks.pop().unwrap();

    let mut shapes = Vec::new();

    for block in blocks {
        let grid: Vec<Vec<bool>> = block
            .lines()
            .skip(1)
            .map(|row| row.chars().map(|c| c == '#').collect())
            .collect();

        let filled = grid.iter().flatten().filter(|&&c| c).count();

        shapes.push((grid, filled));
    }

    let mut trees = Vec::new();

    for line in trees_part.lines() {
        let (size, counts) = line.split_once(':').unwrap();
        let (w, l) = size.split_once('x').unwrap();

        let counts = counts
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        trees.push((w.parse().unwrap(), l.parse().unwrap(), counts));
    }

    (shapes, trees)
}

impl Day for D12 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let (shapes, trees) = parse(input);

        let mut will_fit = 0;

        // absolutely dogshit ass fuck ass problem
        // like at least make me do something
        //
        // I truly hate problems that show a misleading example,
        // when the solution is "look at the data, realize it's
        // very special and solve an entirely different problem"
        for (w, l, reqs) in &trees {
            let required_area = w * l;
            let shape_area = reqs
                .iter()
                .zip(&shapes)
                .map(|(r, (_, f))| r * f)
                .sum::<usize>();

            if required_area >= shape_area {
                will_fit += 1;
            }

            println!("{:?}, {:?}", required_area, shape_area);
        }

        Some(will_fit.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        match input.parse::<usize>() {
            Ok(23) => Option::from("<3".to_string()),
            _ => None,
        }
    }
}
