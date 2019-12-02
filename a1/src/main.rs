use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", read_lines());
}

fn open_file() -> File {
    let file = match File::open("input.txt") {
        Err(_) => panic!("couldn't open the file"),
        Ok(file) => file,
    };
    file
}

fn read_lines() -> u32{
    let buf_reader = BufReader::new(open_file());
    let mut juu: u32 = 0;
    for line in buf_reader.lines() {
        juu += Module {
            mass: line.unwrap().parse::<u32>().unwrap(),
        }
        .count_fuel_need();
    };
    juu
}

struct Module {
    mass: u32,
}

impl Module {
    fn count_fuel_need(&self) -> u32 {
        let a = self.mass / 3;
        let b = a as u32;
        let c = b - 2;
        c
    }
}
