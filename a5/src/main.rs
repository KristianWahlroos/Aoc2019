use std::fs::File;
use std::io::{stdin, BufRead, BufReader, Error, ErrorKind};

struct Program {
    memory: Vec<i32>,
    index: usize,
}

impl Program {
    fn calculate_chunk(&mut self) -> (i32, usize, Option<Result<i32, Error>>) {
        let opcode = self.memory[self.index] % 100;
        let first_param = (self.memory[self.index] % 1000 - opcode) / 100;
        let second_param = (self.memory[self.index] % 10000 - first_param - opcode) / 1000;
        let third_param = (self.memory[self.index] - second_param - first_param - opcode) / 10000;
        match opcode {
            1 => (
                third_param,
                4,
                Some(Ok(self.memory[self.get_index(first_param, 1)]
                    + self.memory[self.get_index(second_param, 2)])),
            ),
            2 => (
                third_param,
                4,
                Some(Ok(self.memory[self.get_index(first_param, 1)]
                    * self.memory[self.get_index(second_param, 2)])),
            ),
            3 => (first_param, 2, Some(get_user_input())),
            4 => (first_param, 2, {
                self.print_output(first_param);
                None
            }),
            5 if self.memory[self.get_index(first_param, 1)] != 0 => {
                self.index = self.memory[self.get_index(second_param, 2)] as usize;
                (second_param, 0, None)
            }
            6 if self.memory[self.get_index(first_param, 1)] == 0 => {
                self.index = self.memory[self.get_index(second_param, 2)] as usize;
                (second_param, 0, None)
            }
            7 if self.memory[self.get_index(first_param, 1)]
                < self.memory[self.get_index(second_param, 2)] =>
            {
                (third_param, 4, Some(Ok(1)))
            }
            8 if self.memory[self.get_index(first_param, 1)]
                == self.memory[self.get_index(second_param, 2)] =>
            {
                (third_param, 4, Some(Ok(1)))
            }
            5 | 6 => (second_param, 3, None),
            7 | 8 => (third_param, 4, Some(Ok(0))),
            99 => (third_param, 0, Some(Err(Error::from(ErrorKind::Other)))),
            _ => (
                third_param,
                0,
                Some(Err(Error::from(ErrorKind::InvalidData))),
            ),
        }
    }

    fn get_index(&self, param: i32, order: usize) -> usize {
        match param {
            0 => self.memory[self.index + order] as usize,
            1 => self.index + order,
            _ => {
                println!("jou{}", param);
                ::std::process::exit(1)
            }
        }
    }

    fn print_output(&self, param: i32) {
        println!("output: {}", self.memory[self.get_index(param, 1)]);
    }
}

fn get_user_input() -> Result<i32, Error> {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .or(Err(Error::from(ErrorKind::InvalidData)));
    i32::from_str_radix(input.trim(), 10).or(Err(Error::from(ErrorKind::InvalidInput)))
}

fn main() {
    let buf_reader = BufReader::new(open_file());
    let mut program = Program {
        memory: buf_reader
            .split(",".chars().next().unwrap() as u8)
            .map(|c| i32::from_str_radix(std::str::from_utf8(&c.unwrap()).unwrap(), 10).unwrap())
            .collect::<Vec<i32>>(),
        index: 0,
    };
    let numbers_len = program.memory.len();
    loop {
        let (imm_addr, increment, result) = &program.calculate_chunk();

        match result {
            Some(a) => match a {
                Ok(v) => {
                    let save_address = program.get_index(*imm_addr, increment - 1);
                    program.memory[save_address] = *v;
                }
                Err(e) => match e.kind() {
                    ErrorKind::Other => {
                        break;
                    }
                    _ => {
                        eprintln!("index: {} + error: {:?}", program.index, e);
                        ::std::process::exit(1);
                    }
                },
            },
            None => (),
        };
        program.index = program.index + increment;
        if program.index >= numbers_len {
            eprintln!("index: {} memory[0]: {}", program.index, program.memory[0]);
            ::std::process::exit(1);
        }
    }
}

fn open_file() -> File {
    let file = match File::open("input.txt") {
        Err(_) => panic!("couldn't open the file"),
        Ok(file) => file,
    };
    file
}
