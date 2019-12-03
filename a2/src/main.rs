use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};


fn main() {
    let buf_reader = BufReader::new(open_file());
    let wanted_answer = 19690720;
    let memory = buf_reader
    .split(",".chars().next().unwrap() as u8)
    .map(|c| u8::from_str_radix(std::str::from_utf8(&c.unwrap()).unwrap(), 10).unwrap() as u32)
    .collect::<Vec<u32>>();
    println!("first part answer: {}", get_answer(memory.clone(), 12, 2));
    let mut x = 0;
    let mut y = 0;
    'outer: while x < 99{
        while y < 99 {
            if wanted_answer == get_answer(memory.clone(), x, y) {
                break 'outer;
            }
            y += 1;
        }
        y = 0;
        x += 1;
    }
    println!("second part answer: {}, ", x * 100 + y);
}

fn open_file() -> File {
    let file = match File::open("input.txt") {
        Err(_) => panic!("couldn't open the file"),
        Ok(file) => file,
    };
    file
}


fn get_answer(mut memory: Vec<u32>, first: u32, second: u32) -> u32 {
    let mut index = 0usize;
    memory[1] = first;
    memory[2] = second;
    
    let numbers_len = memory.len();
    loop {
        let save_address = memory[index + 3] as usize;
        memory[save_address] = match calculate_chunk(index, &memory) {
            Ok(v) => v,
            Err(e) => match e.kind() {
                ErrorKind::Other => {
                    break;
                }
                _ => {
                    eprintln!("index: {} + error: {:?}", index, e);
                    ::std::process::exit(1);
                }
            },
        };
        index += 4;
        if index >= numbers_len {
            eprintln!("index: {} memory[0]: {}", index, memory[0]);
            ::std::process::exit(1);
        }
    }
    memory[0]
}

fn calculate_chunk(index: usize, memory: &Vec<u32>) -> Result<u32, Error> {
    match memory[index] {
        1 => Ok(memory[(memory[index + 1] as usize)] + memory[(memory[index + 2] as usize)]),
        2 => Ok(memory[(memory[index + 1] as usize)] * memory[(memory[index + 2] as usize)]),
        99 => Err(Error::from(ErrorKind::Other)),
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

//Read Dragon book!

// enum Symbols {
    //     Symbol(Symbol),
    //     Symbols(Vec<Symbol>),
    // }
    
    // impl Symbols {
        //     fn connect(&self, other: Symbols, operator: Symbol) -> Symbols {
            //         // -> Symbols{
                //         // let thing = Vec::new();
                //         let thing = match self {
                    //             Self::Symbol(c) => {
//                 let symbol_heap = Vec::new();
//                 symbol_heap.push(c.clone());
//                 symbol_heap
//             }
//             Self::Symbols(c) => c.to_vec(),
//         };
//         thing.push(operator);
//         thing.extend(match other {
    //             Symbols::Symbol(c) => {
        //                 let symbol_heap = Vec::new();
        //                 symbol_heap.push(c.clone());
        //                 symbol_heap
//             }
//             Symbols::Symbols(c) => c.to_vec(),
//         });
//         Symbols::Symbols(thing)
//     }

//     fn get_number(&self) -> usize {
//         match self {
    //             Self::Symbol(Symbol::Number(c)) => c.clone() as usize,
    //             Self::Symbols()
    //             _ => 999999999,
//         }
//     }

// }

// #[derive(Copy, Clone)]
// enum Symbol {
//     X,
//     Y,
//     L_Bracket,
//     R_Bracket,
//     Sum,
//     Multiply,
//     Number(u32),
// }

// //in form x*d+y*c+b=a or go to heuristic approach.
// //will have problem if x and y are in same scale
// fn try_counting() {
//     let buf_reader = BufReader::new(open_file());
//     let mut index = 0usize;
//     let mut memory = buf_reader
//         .split(",".chars().next().unwrap() as u8)
//         // .map(|c| String::from(std::str::from_utf8(&c.unwrap()).unwrap()))
//         .map(|c| {
//             Symbols::Symbol(Symbol::Number(
//                 u8::from_str_radix(std::str::from_utf8(&c.unwrap()).unwrap(), 10).unwrap() as u32,
//             ))
//         })
//         .collect::<Vec<Symbols>>();

//     memory[1] = Symbols::Symbol(Symbol::X(1));
//     memory[2] = Symbols::Symbol(Symbol::Y(1));
// }

// fn calculate_symbol_chunk(index: usize, memory: &Vec<Symbols>) -> Result<Symbols, Error> {
//     // println!(
//     //     "index: {}, numbers[index]: {}, {}, {}, {},  memory[index]: {}, {}, {}",
//     //     index,
//     //     memory[index],
//     //     memory[index + 1],
//     //     memory[index + 2],
//     //     memory[index + 3],
//     //     memory[(memory[index + 1] as usize)],
//     //     memory[(memory[index + 2] as usize)],
//     //     memory[(memory[index + 3] as usize)]
//     // );
//     match memory[index] {
//         Symbols::Symbol(Symbol::Number(1)) => Ok(memory[memory[index + 1].get_number()]
//             .connect(memory[(memory[index + 2] as usize)], Symbol::Sum)),
//         Symbols::Symbol(Symbol::Number(2)) => Ok(memory[(memory[index + 1] as usize)]
//             .connect(memory[(memory[index + 2] as usize)], Symbol::Multiply)),
//         Symbols::Symbol(Symbol::Number(99)) => Err(Error::from(ErrorKind::Other)),
//         // Ok(format!(
//         //     "{}{}",
//         //     &memory[u8::from_str_radix(&memory[index + 1], 10).unwrap() as usize].to_string(),
//         //     &memory[u8::from_str_radix(&memory[index + 2], 10).unwrap() as usize].to_string()
//         // )),
//         // Ok(2) => Ok(memory[(memory[index + 1] as usize)].to_string() + memory[(memory[index + 2] as usize)].to_string()),
//         _ => Err(Error::from(ErrorKind::InvalidData)),
//     }
// }