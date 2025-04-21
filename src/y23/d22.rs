use crate::util::Day;

pub struct D22;

#[derive(Debug, Clone, Copy)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn intersecting(&self, other: &Brick) -> bool {
        let x_overlap = self.start.0 <= other.end.0 && self.end.0 >= other.start.0;
        let y_overlap = self.start.1 <= other.end.1 && self.end.1 >= other.start.1;
        let z_overlap = self.start.2 <= other.end.2 && self.end.2 >= other.start.2;

        x_overlap && y_overlap && z_overlap
    }
}

fn fall(bricks: &mut Vec<Brick>) -> usize {
    let mut moved = 0;

    for i in 0..bricks.len() {
        let start_z = bricks[i].start.2;

        loop {
            let colliding = (0..bricks.len())
                .into_iter()
                .any(|j| j != i && bricks[j].intersecting(&bricks[i]));

            let grounded = bricks[i].start.2 == 0;

            if colliding || grounded {
                break;
            }

            bricks[i].start.2 -= 1;
            bricks[i].end.2 -= 1;
        }

        bricks[i].start.2 += 1;
        bricks[i].end.2 += 1;

        if start_z != bricks[i].start.2 {
            moved += 1;
        }
    }

    moved
}

fn solve(input: &str) -> (usize, usize) {
    let mut bricks = vec![];

    let mut moved = 0;
    let mut disintegratable: usize = 0;

    for line in input.lines() {
        let parts = line
            .split("~")
            .map(|part| {
                part.split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        // starts/ends are ordered
        for i in 0..3 {
            assert!(parts[0][i] <= parts[1][i])
        }

        bricks.push(Brick {
            start: (parts[0][0], parts[0][1], parts[0][2]),
            end: (parts[1][0], parts[1][1], parts[1][2]),
        })
    }

    bricks.sort_by(|a, b| a.start.2.cmp(&b.start.2));

    // first settle
    fall(&mut bricks);

    for i in 0..bricks.len() {
        let mut bricks = bricks[..i]
            .iter()
            .chain(bricks[i + 1..].iter())
            .cloned()
            .collect::<Vec<Brick>>();

        let fell = fall(&mut bricks);
        
        if fell == 0 {
            disintegratable += 1;
        }
        
        moved += fell;
    }

    (disintegratable, moved)
}

impl Day for D22 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        Option::from(solve(input).0.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        Option::from(solve(input).1.to_string())
    }
}
