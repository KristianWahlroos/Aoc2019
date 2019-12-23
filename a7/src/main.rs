use std::fs::File;
use std::io::{BufRead, BufReader};

struct Program {
    amplifiers: Vec<Amplifier>,
    current_amplifier_index: usize,
}

struct Amplifier {
    index: usize,
    memory: Vec<i32>,
    phase_stack: Option<i32>,
    output: Option<i32>,
    last_amplifier_output: i32,
}

impl Program {
    fn create_program(memory: Vec<i32>, permutation: Vec<i32>) -> Program {
        let mut amplifiers: Vec<Amplifier> = Vec::new();
        for phase in permutation {
            amplifiers.push(Amplifier {
                index: 0,
                memory: memory.clone(),
                phase_stack: Some(phase),
                output: None,
                last_amplifier_output: 0,
            })
        }
        Program {
            amplifiers: amplifiers,
            current_amplifier_index: 0,
        }
    }

    fn start_program(&mut self) -> i32 {
        self.amplifiers[self.current_amplifier_index].run(0);
        let mut program_output = 0;
        loop {
            match self.go_to_next_amplifier() {
                false => (),
                true => {
                    program_output = self.amplifiers[self.current_amplifier_index]
                        .output
                        .unwrap_or(0);
                    break;
                }
            }
        }
        program_output
    }

    fn go_to_next_amplifier(&mut self) -> bool {
        let last_amplifier_output = self.amplifiers[self.current_amplifier_index]
            .output
            .unwrap_or(0);
        let mut last_amplifier = false;
        if self.current_amplifier_index + 1 == self.amplifiers.len() {
            // self.current_amplifier_index = 0;
            last_amplifier = true;
        } else {
            self.current_amplifier_index = self.current_amplifier_index + 1;
            self.amplifiers[self.current_amplifier_index].run(last_amplifier_output);
        }
        last_amplifier
    }
}

impl Amplifier {
    fn parse_opcode(opcode: i32) -> i32 {
        opcode % 100
    }

    fn param_count(parsed_opcode: i32) -> usize {
        match parsed_opcode {
            1 | 2 | 7 | 8 => 3,
            5 | 6 => 2,
            3 | 4 => 1,
            _ => 0,
        }
    }

    fn get_opcode(&self) -> i32 {
        self.memory[self.index]
    }

    fn get_operands(&self) -> Vec<i32> {
        let opcode = self.get_opcode();
        let parsed_opcode = Amplifier::parse_opcode(opcode);
        let param_count = Amplifier::param_count(parsed_opcode);
        let mut operands: Vec<i32> = Vec::new();
        for i in 0..param_count {
            operands.push(
                match (opcode % (1000 * (10i32.pow(i as u32)))) / (100 * (10i32.pow(i as u32))) {
                    1 if param_count == i + 1 => (self.index + i + 1) as i32,
                    0 if param_count == i + 1 => self.memory[(self.index + i + 1)],
                    1 => self.memory[(self.index + i + 1)],
                    0 => self.memory[self.memory[self.index + i + 1] as usize],
                    _ => panic!("Invalid opcode parameter"),
                },
            );
        }
        operands
    }

    fn run(&mut self, last_amplifier_output: i32) {
        self.last_amplifier_output = last_amplifier_output;
        self.index = 0;
        self.output = None;
        loop {
            match self.calculate_chunk() {
                true => break,
                false => (),
            }
        }
    }

    fn calculate_chunk(&mut self) -> bool {
        let parsed_opcode = Amplifier::parse_opcode(self.get_opcode());
        match parsed_opcode {
            1 => self.sum(),
            2 => self.multiply(),
            3 => self.get_input(),
            4 => self.set_output(),
            5 => self.jump_if_zero_is_(false),
            6 => self.jump_if_zero_is_(true),
            7 => self.store_if_less(),
            8 => self.store_if_equal(),
            99 => (),
            _ => println!("{}", parsed_opcode),
        };
        match parsed_opcode {
            99 => true,
            _ => false,
        }
    }

    fn store(&mut self, a: i32, to: usize) {
        self.memory[to] = a;
    }

    fn increase_index(&mut self, amount: usize) {
        self.index = self.index + amount;
    }

    fn jump_to_index(&mut self, index: usize) {
        self.index = index;
    }

    //match 1:
    fn sum(&mut self) {
        let operands = self.get_operands();
        self.store(operands[0] + operands[1], operands[2] as usize);
        self.increase_index(operands.len() + 1)
    }

    //match 2:
    fn multiply(&mut self) {
        let operands = self.get_operands();
        self.store(operands[0] * operands[1], operands[2] as usize);
        self.increase_index(operands.len() + 1)
    }
    //match 3:
    fn get_input(&mut self) {
        let operands = self.get_operands();
        self.memory[operands[0] as usize] = match self.phase_stack {
            Some(v) => {
                self.phase_stack = None;
                v
            }
            _ => self.last_amplifier_output,
        };
        self.increase_index(operands.len() + 1)
    }

    //match 4:
    fn set_output(&mut self) {
        let operands = self.get_operands();
        self.output = Some(self.memory[operands[0] as usize]);
        self.increase_index(operands.len() + 1)
    }

    //match 5|6:
    fn jump_if_zero_is_(&mut self, equal: bool) {
        let operands = self.get_operands();
        match equal {
            true if operands[0] == 0 => {
                self.jump_to_index(self.memory[operands[1] as usize] as usize)
            }
            false if operands[0] != 0 => {
                self.jump_to_index(self.memory[operands[1] as usize] as usize)
            }
            _ => self.increase_index(operands.len() + 1),
        }
    }

    //match 7:
    fn store_if_less(&mut self) {
        let operands = self.get_operands();
        match operands[0] < operands[1] {
            true => self.store(1, operands[2] as usize),
            false => self.store(0, operands[2] as usize),
        };
        self.increase_index(operands.len() + 1);
    }

    //match 8:
    fn store_if_equal(&mut self) {
        let operands = self.get_operands();
        match operands[0] == operands[1] {
            true => self.store(1, operands[2] as usize),
            false => self.store(0, operands[2] as usize),
        };
        self.increase_index(operands.len() + 1);
    }
}

fn main() {
    let buf_reader = BufReader::new(File::open("input.txt").unwrap());
    let memory = buf_reader
        .split(b',')
        .map(|c| i32::from_str_radix(std::str::from_utf8(&c.unwrap()).unwrap(), 10).unwrap())
        .collect::<Vec<i32>>();
    let permutations = get_all_permutations(1, 5);
    let mut highest_output = 0;
    for permutation in permutations {
        highest_output = Program::create_program(memory.clone(), permutation).start_program();
    }
    println!("highest output: {}", highest_output)
}

fn get_all_permutations(start: i32, size: i32) -> Vec<Vec<i32>> {
    let mut permutations: Vec<Vec<i32>> = Vec::new();
    let depth = size;
    let mut permutation: Vec<i32> = Vec::new();
    iterate_permutations(start, size, depth, &mut permutations, &mut permutation);
    permutations
}

fn iterate_permutations(
    start: i32,
    size: i32,
    depth: i32,
    permutations: &mut Vec<Vec<i32>>,
    permutation: &mut Vec<i32>,
) {
    if depth == 0 {
        permutations.push(permutation.clone());
    } else {
        'please: for id in start..start + size {
            for number in permutation.clone() {
                if number == id {
                    continue 'please;
                }
            }
            permutation.push(id);
            iterate_permutations(start, size, depth - 1, permutations, permutation);
            permutation.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_all_permutations() {
        assert_eq!(120, get_all_permutations(1, 5).len());
        assert_eq!(120, get_all_permutations(0, 5).len());
        assert_eq!(120, get_all_permutations(-6, 5).len());
        assert_eq!(40320, get_all_permutations(-6, 8).len());
        assert_eq!(vec!(5, 4, 3, 2, 1, 0), get_all_permutations(0, 6)[719]);
    }

    #[test]
    fn test_operand_get() {
        let amplifier = Amplifier {
            memory: vec![1001, 0, 20, 4, 99],
            index: 0,
            output: Some(0),
            last_amplifier_output: 0,
            phase_stack: Some(1),
        };
        let vector = amplifier.get_operands();
        assert_eq!(1001, vector[0]);
        assert_eq!(20, vector[1]);
        assert_eq!(99, amplifier.memory[vector[2] as usize]);
        assert_eq!(4, vector[2]);
    }

    #[test]
    fn test_sum() {
        let mut program = Program::create_program(vec![1001, 0, 20, 1, 99], vec![1]);
        program.start_program();
        assert_eq!(1021, program.amplifiers[0].memory[1]);
    }
    #[test]
    fn test_multiply() {
        let mut program = Program::create_program(vec![102, 20, 0, 2, 99], vec![1]);
        program.start_program();
        assert_eq!(2040, program.amplifiers[0].memory[2]);
    }
    #[test]
    fn test_get_input() {
        let mut program = Program::create_program(vec![3, 1, 3, 3, 99], vec![1]);
        program.start_program();
        assert_eq!(1, program.amplifiers[0].memory[1]);
        assert_eq!(0, program.amplifiers[0].memory[3]);
    }

    #[test]
    fn test_get_output() {
        let mut program = Program::create_program(vec![4, 2, 99], vec![1]);
        program.start_program();
        assert_eq!(99, program.amplifiers[0].output.unwrap());
    }

    #[test]
    fn test_jump_if_zero_is_not_equal() {
        let mut program = Program::create_program(vec![101, -1, 5, 5, 1105, 5, 0, 99], vec![1]);
        assert_eq!(5, program.amplifiers[0].memory[5]);
        program.start_program();
        assert_eq!(0, program.amplifiers[0].memory[5]);
    }

    #[test]
    fn test_jump_if_zero_is_equal() {
        let mut program =
            Program::create_program(vec![1106, 0, 7, 1101, 100, 234, 0, 106, 1, 1, 99], vec![1]);
        program.start_program();
        assert_eq!(1106, program.amplifiers[0].memory[0]);
    }

    #[test]
    fn test_program_1() {
        let mut program = Program::create_program(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            vec![4, 3, 2, 1, 0],
        );
        assert_eq!(43210, program.start_program());
    }

    #[test]
    fn test_program_2() {
        let mut program = Program::create_program(
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            vec![0, 1, 2, 3, 4],
        );
        assert_eq!(54321, program.start_program());
    }

    #[test]
    fn test_program_3() {
        let mut program = Program::create_program(
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            vec![1, 0, 4, 3, 2],
        );
        assert_eq!(65210, program.start_program());
    }
}
