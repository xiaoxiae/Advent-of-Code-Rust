use crate::util::Day;

pub struct D10;

pub enum Instruction {
    Noop,
    Add(isize),
}

const LETTERS: &str = " ##  ###   ##  ###  #### ####  ##  #  # ##### ### #  # #    #  # #  #  ##  ###   ##  ###   ### #### #  # #  # #  # #  # #   ##### |
#  # #  # #  # #  # #    #    #  # #  #   #     # # #  #    #### ## # #  # #  # #  # #  # #     #   #  # #  # #  # #  # #   #  #  |
#  # ###  #    #  # ###  ###  #    ####   #     # ##   #    #  # # ## #  # #  # #  # #  #  ##   #   #  # #  # #  #  ##   # #  #   |
#### #  # #    #  # #    #    # ## #  #   #     # # #  #    #  # #  # #  # ###  #  # ###     #  #   #  # #  # #  # #  #   #   #   |
#  # #  # #  # #  # #    #    #  # #  #   #  #  # #  # #    #  # #  # #  # #    # #  #  #    #  #   #  #  # # #### #  #   #  #    |
#  # ###   ##  ###  #### #     ### #  # ##### ##  #  # #### #  # #  #  ##  #     # # #  # ###   #    ##    #  #  # #  #   #  #### |";


impl Day for D10 {
    fn solve_part1(&self, input: &str) -> Option<String> {
        let cycles = vec![20, 60, 100, 140, 180, 220];
        let mut strength = 0;
        let mut x = 1;
        let mut instructions: Vec<Instruction> = vec![Instruction::Noop];

        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "noop" => {
                    instructions.push(Instruction::Noop);
                }
                "addx" => {
                    instructions.push(Instruction::Noop);
                    instructions.push(Instruction::Add(parts[1].parse::<isize>().unwrap()));
                }
                _ => panic!("Unknown instruction: {}", parts[0]),
            }
        }

        for (cycle, instruction) in instructions.iter().enumerate() {
            let cycle = cycle as isize;

            if cycles.contains(&cycle) {
                strength += cycle * x;
            }


            if let Instruction::Add(val) = instruction {
                x += val;
            }
        }

        Option::from(strength.to_string())
    }

    fn solve_part2(&self, input: &str) -> Option<String> {
        let mut crt = String::new();
        let mut x: isize = 1;
        let mut instructions: Vec<Instruction> = Vec::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "noop" => {
                    instructions.push(Instruction::Noop);
                }
                "addx" => {
                    instructions.push(Instruction::Noop);
                    instructions.push(Instruction::Add(parts[1].parse::<isize>().unwrap()));
                }
                _ => panic!("Unknown instruction: {}", parts[0]),
            }
        }

        for (cycle, instruction) in instructions.iter().enumerate() {
            let cycle = cycle as isize;

            if cycle % 40 == 0 {
                crt.push('\n');
            }

            if (cycle % 40 == x - 1) || (cycle % 40 == x) || (cycle % 40 == x + 1) {
                crt.push('#');
            } else {
                crt.push('.');
            }

            if let Instruction::Add(val) = instruction {
                x += val;
            }
        }

        Option::from(crt)
    }
}
