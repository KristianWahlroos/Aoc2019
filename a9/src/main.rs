use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

struct Program {
    index: usize,
    memory: Vec<i32>,
    input: Option<i32>,
}

impl Program {
    fn create_program(memory: Vec<i32>, input: Option<i32>) -> Program {
        Program {
            index: 0,
            memory: memory,
            input: input,
        }
    }

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
        let parsed_opcode = Program::parse_opcode(opcode);
        let param_count = Program::param_count(parsed_opcode);
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

    fn run(&mut self) {
        loop {
            match self.calculate_chunk() {
                true => break,
                false => (),
            }
        }
    }

    fn calculate_chunk(&mut self) -> bool {
        let parsed_opcode = Program::parse_opcode(self.get_opcode());
        match parsed_opcode {
            1 => self.sum(),
            2 => self.multiply(),
            3 => self.get_input(),
            4 => self.print_output(),
            5 => self.jump_if_zero_is_(false),
            6 => self.jump_if_zero_is_(true),
            7 => self.store_if_less(),
            8 => self.store_if_equal(),
            99 => return true,
            _ => println!("{}", parsed_opcode),
        };
        false
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
        self.memory[operands[0] as usize] = match self.input.take() {
            Some(v) => v,
            _ => {
                let mut input = String::new();
                println!("Insert input: ");
                stdin().read_line(&mut input).unwrap();
                i32::from_str_radix(input.trim(), 10).unwrap()
            }
        };
        self.increase_index(operands.len() + 1);
    }

    //match 4:
    fn print_output(&mut self) {
        let operands = self.get_operands();
        println!("output: {}", self.memory[operands[0] as usize]);
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
    Program::create_program(memory, Some(5)).run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operand_get() {
        let program = Program {
            memory: vec![1001, 0, 20, 4, 99],
            index: 0,
            input: Some(0),
        };
        let vector = program.get_operands();
        assert_eq!(1001, vector[0]);
        assert_eq!(20, vector[1]);
        assert_eq!(99, program.memory[vector[2] as usize]);
        assert_eq!(4, vector[2]);
    }

    #[test]
    fn test_sum() {
        let mut program = Program::create_program(vec![1001, 0, 20, 1, 99], None);
        program.run();
        assert_eq!(1021, program.memory[1]);
    }
    #[test]
    fn test_multiply() {
        let mut program = Program::create_program(vec![102, 20, 0, 2, 99], None);
        program.run();
        assert_eq!(2040, program.memory[2]);
    }
    #[test]
    fn test_get_input() {
        let mut program = Program::create_program(vec![3, 2, 2, 2], Some(99));
        program.run();
        assert_eq!(99, program.memory[2]);
    }

    #[test]
    fn test_jump_if_zero_is_not_equal() {
        let mut program = Program::create_program(vec![101, -1, 5, 5, 1105, 5, 0, 99], None);
        assert_eq!(5, program.memory[5]);
        program.run();
        assert_eq!(0, program.memory[5]);
    }

    #[test]
    fn test_jump_if_zero_is_equal() {
        let mut program =
            Program::create_program(vec![1106, 0, 7, 1101, 100, 234, 0, 106, 1, 1, 99], None);
        program.run();
        assert_eq!(1106, program.memory[0]);
    }

    #[test]
    fn test_case_1_prints_1() {
        println!("Running test_case_1_prints_1");
        let mut program = Program::create_program(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            Some(1),
        );
        program.run();
    }

    #[test]
    fn test_case_2_prints_1() {
        println!("Running test_case_2_prints_1");
        let mut program = Program::create_program(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            Some(1),
        );
        program.run();
    }
}
