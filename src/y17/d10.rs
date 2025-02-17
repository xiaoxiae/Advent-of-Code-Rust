use crate::util::Day;

pub struct D10;

static ELEMENTS: usize = 256;


fn reverse(numbers: &mut Vec<usize>, index: usize, length: u8) {
    let n = numbers.len();

    for i in 0..length / 2 {
        numbers.swap(
            (index + i as usize) % n,
            (index + length as usize - 1 - i as usize) % n,
        )
    }
}

fn knot_hash(numbers: &mut Vec<usize>, lengths: &Vec<u8>, rounds: usize) {
    let mut current_position = 0;
    let mut skip_size = 0;

    for _ in 0..rounds {
        for &length in lengths {
            reverse(numbers, current_position, length);
            current_position += length as usize + skip_size;
            skip_size += 1;
        }
    }
}


impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let lengths = input.trim().split(',').map(|x| x.parse::<u8>().unwrap()).collect::<Vec<_>>();
        let mut numbers = (0..ELEMENTS).collect::<Vec<usize>>();

        knot_hash(&mut numbers, &lengths, 1);

        Option::from((numbers[0] * numbers[1]).to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut lengths = input.trim().chars().map(|c| c as u8).collect::<Vec<_>>();
        let mut numbers = (0..ELEMENTS).collect::<Vec<usize>>();

        for additional in [17, 31, 73, 47, 23] {
            lengths.push(additional as u8);
        }

        knot_hash(&mut numbers, &lengths, 64);

        let mut dense_hash = vec![];
        let stride = 16;

        for i in 0..numbers.len() / stride {
            let xor = numbers[i * stride..(i + 1) * stride].iter().fold(0, |acc, &x| acc ^ x);
            dense_hash.push(xor);
        }

        let result = dense_hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        Option::from(result)
    }
}
