use crate::util::Day;

pub struct D3;

impl Day for D3 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let triangles = input.trim().lines().collect::<Vec<&str>>();
        let mut total = 0;

        for triangle in triangles {
            let sides: Vec<i32> = triangle
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            if sides.len() == 3 {
                let (a, b, c) = (sides[0], sides[1], sides[2]);
                if a + b > c && a + c > b && b + c > a {
                    total += 1;
                }
            }
        }

        Some(total.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let triangles: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut total = 0;

        for i in 0..(triangles.len() / 9) {
            for j in 0..3 {
                let a = triangles[i * 9 + j];
                let b = triangles[i * 9 + 3 + j];
                let c = triangles[i * 9 + 6 + j];

                if a + b > c && a + c > b && b + c > a {
                    total += 1;
                }
            }
        }

        Some(total.to_string())
    }
}
