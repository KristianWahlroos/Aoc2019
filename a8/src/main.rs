use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::iter::Iterator;

fn main() {
    let mut buf_reader = BufReader::new(File::open("input.txt").unwrap());
    let mut chunks: Vec<Vec<u8>> = Vec::new();
    loop {
        let mut buffer = [0u8; 150];
        match buf_reader.read_exact(&mut buffer) {
            Ok(_) => (),
            Err(_) => break,
        };
        chunks.push(buffer.to_vec());
    }
    let mut answer = (std::usize::MAX, 0);
    for chunk in &chunks {
        let zero_count = chunk.iter().filter(|&x| *x == b'0').count();
        if answer.0 > zero_count {
            answer.0 = zero_count;
            let one_count = chunk.iter().filter(|&x| *x == b'1').count();
            let two_count = chunk.iter().filter(|&x| *x == b'2').count();
            answer.1 = one_count * two_count;
        }
    }
    println!("Part 1 answer: {}", answer.1);
    for x in 0..150 {
        if x % 25 == 0 {
            println!("");
        }
        for chunk in &chunks {
            match chunk[x] {
                b'1' => {
                    print!("■");
                    break;
                }
                b'0' => {
                    print!(" ");
                    break;
                }
                _ => (),
            }
        }
    }
}
