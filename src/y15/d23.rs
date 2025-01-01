use crate::util::Day;

pub struct D23;

fn index(r: &str) -> usize {
    match r {
        "a" => 0,
        "b" => 1,
        _ => panic!("Wrong register {}!", r),
    }
}

fn jmp(ip: &mut usize, offset: isize) {
    *ip = ((*ip as isize) + offset) as usize;
}

fn solve(input: &str, registers: &mut Vec<usize>) {
    let commands = input
        .trim()
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect::<Vec<_>>();
    let mut ip: usize = 0;

    while ip < commands.len() {
        let command: &Vec<_> = &commands[ip];

        match command[..] {
            ["hlf", r] => registers[index(r)] /= 2,
            ["tpl", r] => registers[index(r)] *= 3,
            ["inc", r] => registers[index(r)] += 1,
            ["jmp", offset] => jmp(&mut ip, offset.parse::<isize>().unwrap() - 1),
            ["jie", r_with_comma, offset] => {
                if registers[index(&r_with_comma[..r_with_comma.len() - 1])] % 2 == 0 {
                    jmp(&mut ip, offset.parse::<isize>().unwrap() - 1);
                }
            }
            ["jio", r_with_comma, offset] => {
                if registers[index(&r_with_comma[..r_with_comma.len() - 1])] == 1 {
                    jmp(&mut ip, offset.parse::<isize>().unwrap() - 1);
                }
            }
            _ => panic!("Invalid command {:?}!", command),
        }

        ip += 1;
    }
}

impl Day for D23 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let mut registers = vec![0, 0];

        solve(input, &mut registers);

        Option::from(registers[index("b")].to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut registers = vec![1, 0];

        solve(input, &mut registers);

        Option::from(registers[index("b")].to_string())
    }
}
