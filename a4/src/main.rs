use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    joujou();
}

fn open_file() -> File {
    let file = match File::open("input.txt") {
        Err(_) => panic!("couldn't open the file"),
        Ok(file) => file,
    };
    file
}

fn joujou() {
    let buf_reader = BufReader::new(open_file());
    let range = buf_reader
        .split("-".chars().next().unwrap() as u8)
        .map(|c| String::from_utf8(c.unwrap()).unwrap())
        // .map(|c| u32::from_str_radix(std::str::from_utf8(&c.unwrap()).unwrap(), 10).unwrap())
        .collect::<Vec<String>>();
    count_stuff(format!("{:0>6}", &range[0]), format!("{:0>6}", &range[1]))
}

fn count_stuff(start_string: String, end_string: String) {
    let mut counter = 0;
    let mut start_chars = start_string.chars();
    let mut end_chars = end_string.chars();
    let start_numbers = [
        start_chars.next().unwrap().to_digit(10).unwrap(),
        start_chars.next().unwrap().to_digit(10).unwrap(),
        start_chars.next().unwrap().to_digit(10).unwrap(),
        start_chars.next().unwrap().to_digit(10).unwrap(),
        start_chars.next().unwrap().to_digit(10).unwrap(),
        start_chars.next().unwrap().to_digit(10).unwrap(),
    ];
    let end_numbers = [
        end_chars.next().unwrap().to_digit(10).unwrap(),
        end_chars.next().unwrap().to_digit(10).unwrap(),
        end_chars.next().unwrap().to_digit(10).unwrap(),
        end_chars.next().unwrap().to_digit(10).unwrap(),
        end_chars.next().unwrap().to_digit(10).unwrap(),
        end_chars.next().unwrap().to_digit(10).unwrap(),
    ];
    for a in start_numbers[0]..end_numbers[0] {
        for b in 0..10 {
            if a >= end_numbers[0] && b > end_numbers[1] {
                continue;
            }
            if a == start_numbers[0] && b < start_numbers[1] {
                continue;
            }
            if a > b {
                continue;
            }
            for c in 0..10 {
                if a >= end_numbers[0] && b >= end_numbers[1] && c > end_numbers[2] {
                    continue;
                }
                if a == start_numbers[0] && b <= start_numbers[1] && c < start_numbers[2] {
                    continue;
                }
                if b > c {
                    continue;
                }
                for d in 0..10 {
                    if a >= end_numbers[0]
                        && b >= end_numbers[1]
                        && c >= end_numbers[2]
                        && d > end_numbers[3]
                    {
                        continue;
                    }
                    if a == start_numbers[0]
                        && b <= start_numbers[1]
                        && c <= start_numbers[2]
                        && d < start_numbers[3]
                    {
                        continue;
                    }
                    if c > d {
                        continue;
                    }
                    for e in 0..10 {
                        if a >= end_numbers[0]
                            && b >= end_numbers[1]
                            && c >= end_numbers[2]
                            && d >= end_numbers[3]
                            && e > end_numbers[4]
                        {
                            continue;
                        }
                        if a == start_numbers[0]
                            && b <= start_numbers[1]
                            && c <= start_numbers[2]
                            && d <= start_numbers[3]
                            && e < start_numbers[4]
                        {
                            continue;
                        }
                        if d > e {
                            continue;
                        }
                        for f in 0..10 {
                            if a >= end_numbers[0]
                                && b >= end_numbers[1]
                                && c >= end_numbers[2]
                                && d >= end_numbers[3]
                                && e >= end_numbers[4]
                                && f > end_numbers[5]
                            {
                                if a == start_numbers[0]
                                    && b <= start_numbers[1]
                                    && c <= start_numbers[2]
                                    && d <= start_numbers[3]
                                    && e <= start_numbers[4]
                                    && f < start_numbers[5]
                                {
                                    continue;
                                }
                                continue;
                            }
                            if e > f {
                                continue;
                            }
                            if a == b || b == c || c == d || d == e || e == f {
                                println!("found one: {}{}{}{}{}{}", a, b, c, d, e, f);
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", counter);
}
