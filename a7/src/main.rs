use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

struct Program {
    memories: [Vec<i64>; 5],
    index: usize,
    phase_index: usize,
    phases: [i64; 5],
    input: i64,
    output: i64,
    output_check: bool,
    phase_phase: bool,
    all_phases_inputed: bool,
}

impl Program {
    fn calculate_chunk(&mut self) -> (i64, usize, Option<Result<i64, Error>>) {
        let opcode = self.memories[self.phase_index][self.index] % 100;
        let first_param = (self.memories[self.phase_index][self.index] % 1000 - opcode) / 100;
        let second_param =
            (self.memories[self.phase_index][self.index] % 10000 - first_param - opcode) / 1000;
        let third_param =
            (self.memories[self.phase_index][self.index] - second_param - first_param - opcode)
                / 10000;
        match opcode {
            1 => (
                third_param,
                4,
                Some(Ok(self.memories[self.phase_index]
                    [self.get_index(first_param, 1)]
                    + self.memories[self.phase_index]
                        [self.get_index(second_param, 2)])),
            ),
            2 => (
                third_param,
                4,
                Some(Ok(self.memories[self.phase_index]
                    [self.get_index(first_param, 1)]
                    * self.memories[self.phase_index]
                        [self.get_index(second_param, 2)])),
            ),
            3 => (first_param, 2, Some(self.get_input())),
            4 => (first_param, 2, {
                self.store_output(first_param);
                None
            }),
            5 if self.memories[self.phase_index][self.get_index(first_param, 1)] != 0 => {
                self.index =
                    self.memories[self.phase_index][self.get_index(second_param, 2)] as usize;
                (second_param, 0, None)
            }
            6 if self.memories[self.phase_index][self.get_index(first_param, 1)] == 0 => {
                self.index =
                    self.memories[self.phase_index][self.get_index(second_param, 2)] as usize;
                (second_param, 0, None)
            }
            7 if self.memories[self.phase_index][self.get_index(first_param, 1)]
                < self.memories[self.phase_index][self.get_index(second_param, 2)] =>
            {
                (third_param, 4, Some(Ok(1)))
            }
            8 if self.memories[self.phase_index][self.get_index(first_param, 1)]
                == self.memories[self.phase_index][self.get_index(second_param, 2)] =>
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

    fn get_index(&self, param: i64, order: usize) -> usize {
        match param {
            0 => self.memories[self.phase_index][self.index + order] as usize,
            1 => self.index + order,
            _ => {
                println!("jou{}", param);
                ::std::process::exit(1)
            }
        }
    }

    fn store_output(&mut self, param: i64) {
        self.output_check = true;
        self.output = self.memories[self.phase_index][self.get_index(param, 1)];
    }

    fn get_input(&mut self) -> Result<i64, Error> {
        match self.phase_phase && !self.all_phases_inputed {
            true => {
                self.phase_phase = false;
                Ok(self.phases[self.phase_index])
            }
            false => Ok(self.input),
        }
    }
}

fn run(memory: Vec<i64>, phases: [i64; 5]) -> i64 {
    let mut program = Program {
        memories: [
            memory.clone(),
            memory.clone(),
            memory.clone(),
            memory.clone(),
            memory.clone(),
        ],
        index: 0,
        phase_index: 0,
        phases: phases,
        input: 0,
        output: 0,
        output_check: false,
        phase_phase: true,
        all_phases_inputed: false,
    };
    let numbers_len = program.memories[program.phase_index].len();
    loop {
        loop {
            let (imm_addr, increment, result) = &program.calculate_chunk();

            match result {
                Some(a) => match a {
                    Ok(v) => {
                        let save_address = program.get_index(*imm_addr, increment - 1);
                        program.memories[program.phase_index][save_address] = *v;
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
                eprintln!(
                    "index: {} memory[0]: {}",
                    program.index, program.memories[program.phase_index][0]
                );
                ::std::process::exit(1);
            }
        }

        program.phase_phase = true;
        program.memories[program.phase_index] = memory.clone();
        if !program.output_check {
            break;
        }
        if program.phase_index >= 4 {
            program.all_phases_inputed = true;
            program.phase_index = 0;
        } else {
            program.phase_index = program.phase_index + 1;
        }
        program.input = program.output;
        program.output_check = false;
        program.index = 0;
    }
    program.output
}

fn main() {
    let buf_reader = BufReader::new(open_file());
    let memory = buf_reader
        .split(",".chars().next().unwrap() as u8)
        .map(|c| i64::from_str_radix(std::str::from_utf8(&c.unwrap()).unwrap(), 10).unwrap())
        .collect::<Vec<i64>>();
    let mut highest_output = 0;
    // for permutation in get_all_permutations() {
    //     let output = run(memory.clone(), permutation);
    //     if output > highest_output {
    //         highest_output = output;
    //     }
    // }
    highest_output = run(memory.clone(), [5, 6, 7, 8, 9]);
    println!("Highest output is: {}", highest_output);
}

fn get_all_permutations() -> Vec<[i64; 5]> {
    let mut permutations: Vec<[i64; 5]> = Vec::new();
    for a in 0..5 {
        for b in 0..5 {
            if b == a {
                continue;
            }
            for c in 0..5 {
                if c == a {
                    continue;
                }
                if c == b {
                    continue;
                }
                for d in 0..5 {
                    if d == a {
                        continue;
                    }
                    if d == b {
                        continue;
                    }
                    if d == c {
                        continue;
                    }
                    for e in 0..5 {
                        if e == a {
                            continue;
                        }
                        if e == b {
                            continue;
                        }
                        if e == c {
                            continue;
                        }
                        if e == d {
                            continue;
                        }
                        println!("Permutataion foudn: {}{}{}{}{}", a, b, c, d, e);
                        permutations.push([a, b, c, d, e])
                    }
                }
            }
        }
    }
    permutations
}

fn open_file() -> File {
    let file = match File::open("input.txt") {
        Err(_) => panic!("couldn't open the file"),
        Ok(file) => file,
    };
    file
}
