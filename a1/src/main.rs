use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

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
            let mut temp = count_fuel_need(line.unwrap().parse::<u32>().unwrap());
            juu += temp;
            loop {
                if temp == 0 {
                    break;
                }
                temp = count_fuel_need(temp);
                juu += temp;
            }
    };
    juu
}

fn count_fuel_need(mass : u32) -> u32 {
        let a = mass / 3;
        let b = a as u32;
        let c = b.checked_sub(2).unwrap_or(0);
        c
}
